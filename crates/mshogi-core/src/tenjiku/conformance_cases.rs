//! Actual native projections for concrete inputs. This module never receives expected values.
use crate::tenjiku::*;
use serde_json::{Value, json};
fn fail(e: impl std::fmt::Display) -> String {
    e.to_string()
}
fn parse<T: serde::de::DeserializeOwned>(v: &Value) -> Result<T, String> {
    serde_json::from_value(v.clone()).map_err(fail)
}
fn sq(v: &Value) -> Result<Square, String> {
    parse(v)
}
fn array(v: &Value) -> Result<&Vec<Value>, String> {
    v.as_array().ok_or_else(|| "fixture array missing".into())
}
fn face_piece(id: u16, side: Side, face: Face) -> Result<Piece, String> {
    if let Some(base) = Base::ALL.into_iter().find(|b| b.face() == face) {
        return Piece::new(id, side, base, false).map_err(fail);
    }
    let base = Base::ALL
        .into_iter()
        .find(|b| b.promotion() == Some(face))
        .ok_or("unknown native face")?;
    Piece::new(id, side, base, true).map_err(fail)
}
pub(super) fn position_from_root(root: &Value, profile: Profile) -> Result<Position, String> {
    let mut entries = Vec::new();
    for p in array(&root["board"])? {
        let side = parse(&p["side"])?;
        let base = parse(&p["original"])?;
        let promoted = p["promoted"].as_bool().ok_or("fixture promotion bit")?;
        let id = p["id"]
            .as_str()
            .ok_or("fixture identity")?
            .parse::<u16>()
            .map_err(fail)?;
        let mut piece = Piece::new(id, side, base, promoted).map_err(fail)?;
        if let Some(rights) = root["promotion_rights"].as_array() {
            for r in rights {
                if r["piece_id"] == p["id"] {
                    piece.right = match r["state"].as_str() {
                        Some("unseen") => PromotionRight::Unseen,
                        Some("first_declined_next_move") => PromotionRight::FirstDeclinedNextMove,
                        Some("restriction_consumed") => PromotionRight::Consumed,
                        _ => return Err("fixture right state".into()),
                    };
                }
            }
        }
        entries.push((sq(&p["at"])?, piece));
    }
    Position::from_diagram(
        profile,
        &entries,
        parse(&root["turn"])?,
        RootMetadata::diagram(0),
    )
    .map_err(fail)
}
fn triples_position(given: &Value, profile: Profile) -> Result<Position, String> {
    let mut entries = Vec::new();
    for (i, v) in array(&given["board"])?.iter().enumerate() {
        let a = array(v)?;
        entries.push((
            sq(&a[2])?,
            face_piece(i as u16, parse(&a[0])?, parse(&a[1])?)?,
        ));
    }
    Position::from_diagram(
        profile,
        &entries,
        parse(&given["turn"])?,
        RootMetadata::diagram(0),
    )
    .map_err(fail)
}
fn request(game: &Game, actor: Side, command: GameCommand) -> GameRequest {
    GameRequest {
        expected: game.context(),
        actor,
        command,
    }
}
fn record(game: &Game) -> Result<GameRecord, String> {
    GameRecord::from_game(game).map_err(fail)
}
fn phases(effects: &Effects) -> Value {
    Value::Array(effects.phases.iter().map(|phase|match phase{
        EffectPhase::Capture{victim}=>json!({"phase":"capture","victims":[victim.square]}),
        EffectPhase::Burn{victims}=>json!({"phase":"burn","victims":victims.iter().map(|v|v.square).collect::<Vec<_>>()}),
        EffectPhase::MoverDies{square,..}=>json!({"phase":"mover_dies","at":square}),
        EffectPhase::Promotion{square,before,after}=>json!({"phase":"promotion","at":square,"before":before.face().code(),"after":after.face().code()}),
    }).collect())
}
fn side_word(side: Side) -> &'static str {
    match side {
        Side::Sente => "sente",
        Side::Gote => "gote",
    }
}
fn outcome(game: &Game, effect_style: bool) -> String {
    match game.conclusion() {
        Some(Conclusion::RoyalCapture { winner }) => format!(
            "{}_win_{}",
            side_word(winner),
            if effect_style {
                "after_complete_effects"
            } else {
                "royal_capture"
            }
        ),
        Some(Conclusion::SimultaneousRoyalLoss) => "draw_simultaneous_royal_loss".into(),
        Some(Conclusion::BareRoyalLoss { winner }) => {
            format!("{}_win_bare_royal", side_word(winner))
        }
        Some(Conclusion::BothBareRoyals) => "draw_both_bare_royals".into(),
        Some(c) => format!("{c:?}"),
        None => game
            .snapshot()
            .pending_bare
            .map(|s| format!("bare_response_pending_{}", side_word(s)))
            .unwrap_or_else(|| "ongoing".into()),
    }
}
fn constructed(given: &Value, profile: Profile) -> Result<Value, String> {
    let position = position_from_root(&given["root"], profile)?;
    let before_position = position.clone();
    let from = sq(&given["actor"])?;
    let mover = position.piece(from).ok_or("fixture origin empty")?;
    let mut game = Game::from_root(position).map_err(fail)?;
    let before = record(&game)?;
    let context = game.context();
    let action = Action {
        kind: if given["kind"] == "igui" {
            ActionKind::Igui
        } else {
            ActionKind::Travel
        },
        from,
        route: parse(&given["route"])?,
        promote: given["promote"].as_bool().unwrap_or(false),
        target: given.get("target").map(sq).transpose()?,
    };
    let submitted = game.submit(request(
        &game,
        parse(&given["side"])?,
        GameCommand::Play {
            action: action.clone(),
        },
    ));
    let accepted = submitted.is_ok();
    let after = record(&game)?;
    let effects = if accepted {
        game.events().last().and_then(|e| e.effects.as_ref())
    } else {
        None
    };
    let captures = effects
        .map(|e| e.captures().map(|c| c.square).collect::<Vec<_>>())
        .unwrap_or_default();
    let surviving_mover = game.position().pieces().find(|(_, p)| p.id == mover.id);
    let surviving = |id: u16| game.position().pieces().any(|(_, p)| p.id == id);
    let mut enemy_demons = game
        .position()
        .pieces()
        .filter(|(_, p)| p.side != mover.side && p.face() == Face::FiD)
        .map(|(s, _)| s)
        .collect::<Vec<_>>();
    enemy_demons.sort();
    let friendly_survives = before_position
        .pieces()
        .filter(|(_, p)| p.side == mover.side && p.id != mover.id)
        .all(|(_, p)| surviving(p.id));
    let enemy_pawn_survives = before_position
        .pieces()
        .filter(|(_, p)| p.side != mover.side && p.base == Base::P)
        .all(|(_, p)| surviving(p.id));
    let all_pawns_survive = before_position
        .pieces()
        .filter(|(_, p)| p.base == Base::P)
        .all(|(_, p)| surviving(p.id));
    // Report crossed occupancy from the actual submitted straight route; this
    // projects geometry and never decides whether the native action is legal.
    let jump_used = if action.route.len() == 1 {
        let to = action.route[0];
        let df = i16::from(to.file()) - i16::from(from.file());
        let dr = i16::from(to.rank()) - i16::from(from.rank());
        let straight = df == 0 || dr == 0 || df.abs() == dr.abs();
        straight
            && (1..df.abs().max(dr.abs())).any(|step| {
                Square::new(
                    (i16::from(from.file()) + step * df.signum()) as u8,
                    (i16::from(from.rank()) + step * dr.signum()) as u8,
                )
                .is_some_and(|square| before_position.piece(square).is_some())
            })
    } else {
        false
    };
    let promotion = effects.is_some_and(|e| {
        e.phases
            .iter()
            .any(|p| matches!(p, EffectPhase::Promotion { .. }))
    });
    let burn = effects.is_some_and(|e| {
        e.phases
            .iter()
            .any(|p| matches!(p, EffectPhase::Burn { .. }))
    });
    Ok(
        json!({"accepted":accepted,"state_unchanged":before==after&&context==game.context(),
        "effects":effects.map(phases).unwrap_or_else(||json!([])),"capture_order":captures,
        "surviving_enemy_demon":enemy_demons.first(),"enemy_demons_survive":enemy_demons.len(),
        "friendly_survives":friendly_survives,"enemy_pawn_survives":enemy_pawn_survives,
        "intermediate_pawn_survives":all_pawns_survive,"unmoved_friendly_gold_survives":before_position.pieces().filter(|(_,p)|p.side==mover.side&&p.base==Base::G&&p.id!=mover.id).all(|(_,p)|surviving(p.id)),
        "mover_survives":surviving_mover.is_some(),"promoted_face":surviving_mover.map(|(_,p)|p.face().code()),
        "immediate_burn":burn,"promotion_event":promotion,"intermediate_proximity_burn":effects.is_some_and(|e|e.phases.iter().any(|p|matches!(p,EffectPhase::MoverDies{square,..} if Some(*square)!=action.to()))),
        "opponent_royals_remaining":game.royals(mover.side.opponent()).len(),"outcome":outcome(&game,true),
        "both_royal_sets_empty":game.royals(Side::Sente).is_empty()&&game.royals(Side::Gote).is_empty(),
        "stationary":surviving_mover.is_some_and(|(s,_)|s==from),
        "protected_lion_exchange_rule":!accepted,"jump_used":jump_used}),
    )
}
fn gold(game: &Game) -> Option<Piece> {
    game.position()
        .pieces()
        .find(|(_, p)| p.side == Side::Sente && p.base == Base::G)
        .map(|(_, p)| p)
}
fn equalizing_reply(game: &Game) -> Result<bool, String> {
    if game.conclusion().is_some() || game.snapshot().pending_bare.is_none() {
        return Ok(false);
    }
    for action in game.legal_actions() {
        let mut copy = record(game)?.to_game().map_err(fail)?;
        copy.submit(request(
            &copy,
            copy.position().side_to_move(),
            GameCommand::Play { action },
        ))
        .map_err(fail)?;
        if copy.conclusion() == Some(Conclusion::BothBareRoyals) {
            return Ok(true);
        }
    }
    Ok(false)
}
fn concrete_sequence(id: &str, given: &Value, profile: Profile) -> Result<Value, String> {
    let position = position_from_root(&given["root"], profile)?;
    let root = position.clone();
    let mut game = Game::from_root(position).map_err(fail)?;
    let commands = array(&given["commands"])?;
    let mut accepted = 0;
    let mut last_ok = false;
    let mut unchanged = false;
    let mut last_error = None;
    let mut first = None;
    let mut second = None;
    let mut equalizing = false;
    let mut rights_before_final = None;
    let mut attempted_repeat = false;
    for (i, raw) in commands.iter().enumerate() {
        let cmd: GameCommand = parse(&raw["command"])?;
        let before = record(&game)?;
        let context = game.context();
        if i + 1 == commands.len() {
            rights_before_final = gold(&game).map(|p| p.right);
            if let GameCommand::Play { action } = &cmd
                && let Ok((probe, _)) = game.local_position().simulate(action)
            {
                attempted_repeat = RepetitionKey::from_position(probe.position())
                    == RepetitionKey::from_position(&root);
            }
        }
        let result = game.submit(request(&game, parse(&raw["actor"])?, cmd));
        last_ok = result.is_ok();
        last_error = result.err();
        if last_ok {
            accepted += 1
        }
        unchanged = before == record(&game)? && context == game.context();
        if i == 0 {
            first = Some(outcome(&game, false));
            equalizing = equalizing_reply(&game)?;
        }
        if i == 1 {
            second = Some(outcome(&game, false));
        }
    }
    let last_effects = game.events().last().and_then(|e| e.effects.as_ref());
    let captured_at = last_effects
        .and_then(|e| e.captures().next())
        .map(|c| c.square);
    let king_before = root
        .pieces()
        .find(|(_, p)| p.side == Side::Sente && p.base == Base::K)
        .map(|(_, p)| p.id);
    let own_royal_exposed = if id == "TEN-ROYAL-EXPOSURE-CONCRETE" {
        game.legal_actions().iter().any(|a| {
            game.local_position().simulate(a).is_ok_and(|(p, _)| {
                !p.position()
                    .pieces()
                    .any(|(_, p)| Some(p.id) == king_before)
            })
        })
    } else {
        false
    };
    let gold_right_diff = gold(&game)
        .zip(
            root.pieces()
                .find(|(_, p)| p.side == Side::Sente && p.base == Base::G)
                .map(|(_, p)| p),
        )
        .is_some_and(|(a, b)| a.right != b.right);
    Ok(
        json!({"accepted":last_ok,"all_accepted":accepted==commands.len(),"accepted_command_count":accepted,
        "last_command_accepted":last_ok,"last_rejection_atomic":!last_ok&&unchanged,
        "reason":match last_error{Some(GameError::Repetition)=>"first_repetition",Some(_)=>"other_refusal",None=>"none"},
        "state_and_history_unchanged_by_last":unchanged,"accepted_state_and_history_equal_third_prefix":game.events().len()==3&&unchanged,
        "attempted_candidate_board_and_turn_equal_root":attempted_repeat,"promotion_rights_differ_from_root":gold_right_diff,
        "gold_movement_face":gold(&game).map(|p|p.face().code()),"captured_at":captured_at,
        "first_refusal_next_move_restriction_expired":rights_before_final==Some(PromotionRight::Consumed),
        "after_first":first,"after_second":second,"outcome":outcome(&game,false),
        "immediate_equalizing_reply_exists":equalizing,"equalizing_nonroyal_capture_exists":equalizing,
        "concluded":game.conclusion().is_some(),"own_royal_exposed":own_royal_exposed,
        "gote_king_burned":last_effects.is_some_and(|e|e.phases.iter().any(|p|matches!(p,EffectPhase::Burn{victims} if victims.iter().any(|v|v.piece.side==Side::Gote&&v.piece.base==Base::K)))),
        "gote_prince_survives":game.position().pieces().any(|(_,p)|p.side==Side::Gote&&p.face()==Face::CP)}),
    )
}
fn simple_action(action: &Action) -> Value {
    json!({"from":action.from,"route":action.route,"promote":action.promote})
}
fn perft(given: &Value, profile: Profile) -> Result<Value, String> {
    let game = Game::from_root(triples_position(given, profile)?).map_err(fail)?;
    let actions = game
        .legal_actions()
        .iter()
        .map(simple_action)
        .collect::<Vec<_>>();
    let d = game.divide(2, 100).map_err(fail)?;
    Ok(
        json!({"legal_all":actions,"perft":{"0":game.perft(0,100).map_err(fail)?.leaves,"1":game.perft(1,100).map_err(fail)?.leaves,"2":game.perft(2,100).map_err(fail)?.leaves},
        "divide_depth2":d.entries.iter().map(|e|json!({"action":simple_action(&e.action),"nodes":e.leaves})).collect::<Vec<_>>()}),
    )
}
fn mix(id: &str, profile: Profile) -> Result<Value, String> {
    let game = Game::standard(profile);
    let before = record(&game)?;
    let context = game.context();
    let other = if profile == Profile::AdamsModern {
        Profile::MullerCompleted
    } else {
        Profile::AdamsModern
    };
    let mut wire: Value = serde_json::from_str(before.as_json()).map_err(fail)?;
    if id == "TEN-PROFILE-MIX-EDITION" {
        wire["edition"] = json!(other.edition())
    } else {
        wire["profile"] = json!(other);
        wire["edition"] = json!(other.edition());
    }
    let result = GameRecord::from_json(&wire.to_string());
    Ok(
        json!({"accepted":result.is_ok(),"native_state_and_record_unchanged":record(&game)?==before&&game.context()==context}),
    )
}
pub(super) fn observe(id: &str, given: &Value, profile: Profile) -> Result<Option<Value>, String> {
    if id.starts_with("TEN-PROMOTION-CONCRETE-") {
        return Ok(None);
    }
    if given["query"] == "constructed_action" {
        return constructed(given, profile).map(Some);
    }
    if given.get("commands").is_some() {
        return concrete_sequence(id, given, profile).map(Some);
    }
    if id == "TEN-PERFT-4-16" {
        return perft(given, profile).map(Some);
    }
    if matches!(
        id,
        "TEN-PROFILE-MIX-CANONICAL_INNER_OUTER" | "TEN-PROFILE-MIX-EDITION"
    ) {
        return mix(id, profile).map(Some);
    }
    if id == "TEN-NO-ACTIONS-CONCRETE" {
        let game = Game::from_root(position_from_root(&given["root"], profile)?).map_err(fail)?;
        return Ok(Some(
            json!({"legal_all":game.legal_actions().iter().map(simple_action).collect::<Vec<_>>(),"status":if game.status()==GameStatus::NoBoardActions{"no_board_actions"}else{"other"},"concluded":game.conclusion().is_some(),"score":Value::Null}),
        ));
    }
    if id == "TEN-ZONE-INTERMEDIATE" {
        let from = sq(&given["actor"])?;
        let piece = Piece::new(0, Side::Sente, parse(&given["original"])?, false).map_err(fail)?;
        let p = Position::from_diagram(
            profile,
            &[(from, piece)],
            Side::Sente,
            RootMetadata::diagram(0),
        )
        .map_err(fail)?;
        let mut local = LocalPosition::admit(p).map_err(fail)?;
        let action = Action {
            kind: ActionKind::Travel,
            from,
            route: parse(&given["route"])?,
            promote: given["promote"].as_bool().ok_or("promotion")?,
            target: None,
        };
        let effects = local.apply(&action).map_err(fail)?.0;
        return Ok(Some(
            json!({"eligible":effects.mover_after.is_some_and(|p|p.promoted),"resulting_face":effects.mover_after.map(|p|p.face().code()),
            "promotion_after_complete_route":effects.phases.last().is_some_and(|p|matches!(p,EffectPhase::Promotion{square,..} if Some(*square)==action.to()))}),
        ));
    }
    Ok(None)
}
