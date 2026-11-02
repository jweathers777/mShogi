//! Actual native Game/history/record projections of the independently authored
//! Chu corpus. Expected answers are read only by the common comparator.
use super::conformance::{action, action_value, root, side};
use super::*;
use mshogi_conformance::{
    assertions::Assertion,
    chu::{Corpus, Fixture, NativeAdapter},
};
use serde_json::{Value, json};
use std::{cell::RefCell, collections::BTreeMap};

type ProjectionCache = RefCell<BTreeMap<String, Result<BTreeMap<String, Value>, String>>>;

#[derive(Default)]
struct GameAdapter {
    cache: ProjectionCache,
}
impl NativeAdapter for GameAdapter {
    fn observe(&self, f: &Fixture, a: &Assertion) -> Result<Option<Value>, String> {
        if !self.cache.borrow().contains_key(&f.id) {
            self.cache.borrow_mut().insert(f.id.clone(), project(f));
        }
        self.cache
            .borrow()
            .get(&f.id)
            .unwrap()
            .as_ref()
            .map(|m| m.get(&a.field).cloned())
            .map_err(Clone::clone)
    }
}
#[derive(Default)]
struct CompleteAdapter {
    local: super::conformance::LocalAdapter,
    game: GameAdapter,
}
impl NativeAdapter for CompleteAdapter {
    fn observe(&self, f: &Fixture, a: &Assertion) -> Result<Option<Value>, String> {
        match self.local.observe(f, a)? {
            Some(v) => Ok(Some(v)),
            None => self.game.observe(f, a),
        }
    }
}
fn put(m: &mut BTreeMap<String, Value>, field: &str, v: impl serde::Serialize) {
    m.insert(field.into(), serde_json::to_value(v).unwrap());
}
fn submit(g: &mut Game, actor: Side, c: GameCommand) -> Result<GameUndo, GameError> {
    g.submit(GameRequest {
        expected: g.context(),
        actor,
        command: c,
    })
}
fn play(g: &mut Game, a: Action) -> Result<GameUndo, GameError> {
    submit(
        g,
        g.position().side_to_move(),
        GameCommand::Play { action: a },
    )
}
fn fixture_game(f: &Fixture) -> Result<Game, String> {
    let mut r = f.given["root"].clone();
    if let Some(n) = f.given.get("declared_counter_override") {
        r["declared_board_plies"] = n.clone();
    }
    Game::from_root(root(&r).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}
fn fresh(g: &Game) -> Result<Game, String> {
    GameRecord::from_game(g)
        .and_then(|r| r.to_game())
        .map_err(|e| e.to_string())
}
fn conclusion(g: &Game) -> Value {
    outcome(g.conclusion())
}
fn outcome(c: Option<Conclusion>) -> Value {
    c.map_or(Value::Null, |c| {
        json!(match c {
            Conclusion::RoyalCapture { .. } => "royal_capture",
            Conclusion::Mate { .. } => "mate",
            Conclusion::Material { .. } => "material_exhaustion",
            Conclusion::Resignation { .. } => "resignation",
            Conclusion::AgreedDraw => "agreed_draw",
        })
    })
}
fn logical(g: &Game) -> Value {
    json!({"record":serde_json::from_str::<Value>(GameRecord::from_game(g).unwrap().as_json()).unwrap(),"caches":g.cache_snapshot()})
}
fn command(c: &Value, g: &Game) -> Result<(Side, GameCommand), String> {
    let actor = c.get("by").map(side).unwrap_or(g.position().side_to_move());
    let cmd = match c["kind"].as_str() {
        Some("move") => GameCommand::Play { action: action(c) },
        Some("offer_draw") => GameCommand::OfferDraw,
        Some("accept_draw") => GameCommand::AcceptDraw,
        Some("decline_draw") => GameCommand::DeclineDraw,
        Some("resign") => GameCommand::Resign,
        _ => return Err("unknown fixture command".into()),
    };
    Ok((actor, cmd))
}
struct Journey {
    game: Game,
    accepted: Vec<bool>,
    errors: Vec<Option<GameError>>,
    atomic: Vec<bool>,
    before: HistorySnapshot,
}
fn run(f: &Fixture) -> Result<Journey, String> {
    let mut g = fixture_game(f)?;
    let before = g.snapshot().clone();
    let mut accepted = Vec::new();
    let mut errors = Vec::new();
    let mut atomic = Vec::new();
    let inputs: Vec<Value> = if let Some(a) = f.given.get("action") {
        vec![a.clone()]
    } else {
        f.given["commands"].as_array().cloned().unwrap_or_default()
    };
    for c in inputs {
        let (actor, cmd) = command(&c, &g)?;
        let context = g.context();
        let caches = g.cache_snapshot();
        let snap = g.snapshot().clone();
        match submit(&mut g, actor, cmd) {
            Ok(_) => {
                accepted.push(true);
                errors.push(None);
                atomic.push(true)
            }
            Err(e) => {
                accepted.push(false);
                errors.push(Some(e));
                atomic.push(
                    g.context() == context && g.snapshot() == &snap && g.cache_snapshot() == caches,
                );
                if f.given["on_rejection"] == "stop" {
                    break;
                }
            }
        }
    }
    Ok(Journey {
        game: g,
        accepted,
        errors,
        atomic,
        before,
    })
}
fn root_occurrences(g: &Game) -> usize {
    let initial = RepetitionKey::from_local(&g.event_prefix(0).unwrap().local_position());
    1 + g
        .events()
        .iter()
        .enumerate()
        .filter(|(_, e)| e.effects.is_some())
        .filter(|(i, _)| {
            RepetitionKey::from_local(&g.event_prefix(i + 1).unwrap().local_position()) == initial
        })
        .count()
}
fn witness(g: &Game) -> Option<&Obligation> {
    g.obligations()
        .iter()
        .find(|o| o.occurrences[0] == 0)
        .or_else(|| g.obligations().first())
}
fn royals(g: &Game) -> Vec<String> {
    let mut v: Vec<_> = g
        .position()
        .pieces()
        .filter(|(_, p)| p.is_royal())
        .map(|(_, p)| {
            format!(
                "{}:{}",
                if p.side == Side::Sente {
                    "sente"
                } else {
                    "gote"
                },
                p.face().code()
            )
        })
        .collect();
    v.sort();
    v
}
fn attack_events(g: &Game, owner: Side) -> Vec<bool> {
    g.events()
        .iter()
        .enumerate()
        .filter(|(_, e)| e.actor == owner && e.effects.is_some())
        .map(|(i, e)| {
            let prev = g
                .event_prefix(i)
                .unwrap()
                .local_position()
                .capture_influence(owner);
            let after = g
                .event_prefix(i + 1)
                .unwrap()
                .local_position()
                .capture_influence(owner);
            let moved = *e.effects.as_ref().unwrap().action.route.last().unwrap();
            after.iter().any(|p| p.0 == moved || !prev.contains(p))
        })
        .collect()
}

fn project(f: &Fixture) -> Result<BTreeMap<String, Value>, String> {
    let mut out = BTreeMap::new();
    if f.id.starts_with("CHU-UNDO-") {
        return reversal(f);
    }
    if f.id.starts_with("CHU-RECORD-") {
        return records(f);
    }
    if f.id.starts_with("CHU-INVALID-") {
        return invalid(f);
    }
    if f.id == "CHU-HISTORY-LION-WINDOW" {
        return lion_history(f);
    }
    if !f.given["root"].is_object() && f.given["root"] != "standard_start" {
        return Ok(out);
    }
    if f.id == "CHU-HISTORY-SAME-BOARD" {
        return different_history(f);
    }
    if f.id == "CHU-REP-ROUTE-IDENTITY" {
        return route_identity(f);
    }
    let journey = run(f)?;
    let g = &journey.game;
    if let Some(accepted) = journey.accepted.first() {
        put(&mut out, "admitted", accepted);
        put(&mut out, "first_admitted", accepted);
        put(&mut out, "rejection_atomic", journey.atomic[0]);
    }
    put(
        &mut out,
        "all_admitted",
        !journey.accepted.is_empty() && journey.accepted.iter().all(|a| *a),
    );
    put(&mut out, "concluded", g.conclusion().is_some());
    put(&mut out, "outcome", conclusion(g));
    put(&mut out, "final_outcome", conclusion(g));
    put(&mut out, "root_outcome", outcome(journey.before.conclusion));
    put(
        &mut out,
        "winner",
        g.conclusion().and_then(Conclusion::winner),
    );
    put(
        &mut out,
        "remaining_sente_royals",
        g.royal_squares(Side::Sente),
    );
    put(
        &mut out,
        "material_winner",
        match g.conclusion() {
            Some(Conclusion::Material { winner }) => Some(winner),
            _ => None,
        },
    );
    put(&mut out, "accepted_board_plies", g.snapshot().board_ply);
    put(&mut out, "board_plies", g.snapshot().board_ply);
    put(&mut out, "native_event_count", g.events().len());
    put(&mut out, "occurrences_of_root", root_occurrences(g));
    put(&mut out, "live_root_occurrences", root_occurrences(g));
    put(&mut out, "known_root_occurrences", root_occurrences(g));
    put(
        &mut out,
        "earlier_occurrences_inferred",
        g.occurrences().iter().any(|i| *i > g.snapshot().board_ply),
    );
    put(
        &mut out,
        "declared_counter_preserved",
        g.position().root().declared_completed_plies(),
    );
    put(
        &mut out,
        "refusal_atomic",
        journey.atomic.iter().all(|v| *v),
    );
    if let Some(index) = journey.errors.iter().position(Option::is_some) {
        put(&mut out, "rejected_command_index", index + 1);
    }
    put(
        &mut out,
        "moves9and10_admitted",
        journey.accepted.get(8) == Some(&true) && journey.accepted.get(9) == Some(&true),
    );
    if let Some(w) = witness(g) {
        put(&mut out, "responsible_sides", &w.responsible_sides);
        put(&mut out, "category", w.category);
        put(&mut out, "prospective_closing_mover", w.closing_mover);
    }
    if let Some(e) = g.events().iter().find_map(|e| e.effects.as_ref()) {
        put(
            &mut out,
            "ordered_captures",
            e.captures.iter().map(|c| c.square).collect::<Vec<_>>(),
        );
    }
    match f.id.as_str() {
        "CHU-REP-IMPORTED" => {
            // Read the native cache, not a second count which would conceal an
            // implementation inventing duplicate root occurrences from8000.
            put(&mut out, "known_root_occurrences", g.occurrences().len());
            put(
                &mut out,
                "earlier_occurrences_inferred",
                g.occurrences() != [0] || !g.position().root().earlier_history_unknown(),
            );
        }
        "CHU-ROYAL-EXPOSURE" | "CHU-ROYAL-IGNORE" => {
            let actor = journey.before.local_position().position().side_to_move();
            let attacked = g.royal_squares(actor).iter().any(|s| {
                g.local_position()
                    .capture_influence(actor.opponent())
                    .iter()
                    .any(|(_, t)| t == s)
            });
            // The accepted native action leaves the royal exposed; there was no
            // imported Chess safety rejection.
            put(
                &mut out,
                "self_check_filter",
                !journey.accepted.first().copied().unwrap_or(false) || !attacked,
            );
        }
        "CHU-ROYAL-COUNTERCAPTURE" | "CHU-MATERIAL-COUNTERWIN" => {
            if matches!(g.conclusion(), Some(Conclusion::RoyalCapture { .. })) {
                put(&mut out, "reason", "last_royal_capture");
            }
            put(
                &mut out,
                "no_material_award_before_reply",
                !matches!(journey.before.conclusion, Some(Conclusion::Material { .. })),
            );
        }
        "CHU-ROYAL-COMPOUND" => {
            if g.conclusion().is_none()
                && g.position()
                    .pieces()
                    .any(|(_, p)| p.side == Side::Gote && p.base == Base::King)
            {
                put(&mut out, "reason", "gote_king_remains");
            }
        }
        "CHU-ROYAL-LAST-COMPOUND" | "CHU-ROYAL-BOTH-COMPOUND" => {
            put(
                &mut out,
                "outcome_after_complete_action",
                g.events().last().is_some_and(|e| {
                    e.conclusion.is_some()
                        && e.effects.as_ref().is_some_and(|e| e.captures.len() == 2)
                }),
            );
        }
        "CHU-NOACTION" => {
            put(
                &mut out,
                "complete_actions",
                g.legal_actions()
                    .iter()
                    .map(action_value)
                    .collect::<Vec<_>>(),
            );
            put(
                &mut out,
                "status",
                match g.status() {
                    GameStatus::NoBoardActions if g.conclusion().is_none() => {
                        "no_board_actions_unresolved_unscored"
                    }
                    GameStatus::Concluded => "concluded",
                    _ => "ongoing",
                },
            );
            let mut resign = fresh(g)?;
            let resigned = submit(&mut resign, Side::Sente, GameCommand::Resign).is_ok();
            let mut agree = fresh(g)?;
            let offered = submit(&mut agree, Side::Sente, GameCommand::OfferDraw).is_ok();
            let accepted = submit(&mut agree, Side::Gote, GameCommand::AcceptDraw).is_ok();
            put(
                &mut out,
                "resignation_and_agreement_remain_available",
                resigned && offered && accepted,
            );
        }
        "CHU-MATERIAL-RECAPTURE" => {
            put(
                &mut out,
                "not_concluded_after_second",
                g.event_prefix(2)
                    .map_err(|e| e.to_string())?
                    .conclusion
                    .is_none(),
            );
            put(&mut out, "final_royals", royals(g));
            let mut copy = fresh(g)?;
            let available = submit(&mut copy, Side::Sente, GameCommand::OfferDraw).is_ok()
                && submit(&mut copy, Side::Gote, GameCommand::AcceptDraw).is_ok();
            put(&mut out, "draw_agreement_available", available);
            put(
                &mut out,
                "automatic_draw",
                g.conclusion() == Some(Conclusion::AgreedDraw),
            );
        }
        "CHU-REP-PREFIX" => {
            let n = f.given["historical_event_prefix"].as_u64().unwrap() as usize;
            let prefix = g
                .record_prefix(n)
                .and_then(|r| r.to_game())
                .map_err(|e| e.to_string())?;
            put(
                &mut out,
                "prefix_root_occurrences",
                root_occurrences(&prefix),
            );
            put(
                &mut out,
                "prefix_has_future_conclusion",
                prefix.conclusion().is_some(),
            );
        }
        "CHU-REP-ATTACK" => {
            put(
                &mut out,
                "gote_moves_deliver_attack",
                attack_events(g, Side::Gote).iter().any(|a| *a),
            );
        }
        "CHU-REP-BOTH-ATTACK" => {
            put(
                &mut out,
                "both_sides_deliver_attack",
                [Side::Sente, Side::Gote].into_iter().all(|s| {
                    let a = attack_events(g, s);
                    !a.is_empty() && a.into_iter().all(|a| a)
                }),
            );
        }
        "CHU-REP-PASS-OVERRIDES-MAINTAINED-THREAT" => {
            let mut attempted = fresh(g)?;
            let mut rejected = true;
            for a in f.given["attempts"].as_array().unwrap() {
                let before = attempted.context();
                let result = play(&mut attempted, action(a));
                rejected &= matches!(result, Err(GameError::RepetitionDeparture))
                    && before == attempted.context();
            }
            put(&mut out, "both_attempts_rejected", rejected);
            // Exercise the production classifier on the complete known cycle
            // beginning at the shifted anchor. Its first actor is Sente, but
            // the full-history continuous-pass streak actually began with Gote.
            // The forbidden fifth pass need not be committed to inspect this
            // already known two-turn cycle.
            let prefix = f.given["rotated_anchor_prefix"].as_u64().unwrap() as usize;
            let shifted = g
                .event_prefix(prefix)
                .map_err(|e| e.to_string())?
                .local_position()
                .position()
                .side_to_move();
            let (shifted_sides, shifted_category) = g.cycle_responsibility(prefix, prefix + 2);
            put(
                &mut out,
                "same_culprit_at_rotated_anchor",
                witness(g).is_some_and(|w| {
                    w.responsible_sides == shifted_sides && w.category == shifted_category
                }) && !shifted_sides.contains(&shifted),
            );
        }
        "CHU-MATE" => {
            put(
                &mut out,
                "position_board_action_count",
                g.local_position().legal_actions().len(),
            );
            put(
                &mut out,
                "game_winner",
                g.conclusion().and_then(Conclusion::winner),
            );
            put(&mut out, "game_conclusion", conclusion(g));
        }
        "CHU-MATE-COUNTERWIN" => {
            put(
                &mut out,
                "root_mate",
                matches!(g.conclusion(), Some(Conclusion::Mate { .. })),
            );
            let mut reply = fresh(g)?;
            let accepted = play(&mut reply, action(&f.given["winning_reply"])).is_ok();
            put(&mut out, "reply_admitted", accepted);
            put(
                &mut out,
                "reply_winner",
                reply.conclusion().and_then(Conclusion::winner),
            );
        }
        "CHU-MATE-PRINCE-RESPONSE" => {
            put(
                &mut out,
                "root_mate",
                matches!(g.conclusion(), Some(Conclusion::Mate { .. })),
            );
            let mut reply = fresh(g)?;
            let accepted = play(&mut reply, action(&f.given["response"])).is_ok();
            put(&mut out, "response_admitted", accepted);
            let royals = reply.royal_squares(Side::Sente);
            let attacks = reply.local_position().capture_influence(Side::Gote);
            put(
                &mut out,
                "both_royals_individually_threatened",
                royals.len() == 2 && royals.iter().all(|s| attacks.iter().any(|(_, t)| t == s)),
            );
            put(
                &mut out,
                "one_opponent_complete_action_captures_both",
                reply
                    .has_capture_all(Side::Sente)
                    .map_err(|e| e.to_string())?,
            );
        }
        "CHU-MATERIAL-IMPORT-SENTE" | "CHU-MATERIAL-IMPORT-GOTE" => {
            put(
                &mut out,
                "immediate_counterwin_available_to_gote",
                g.position().side_to_move() == Side::Gote
                    && g.immediate_material_replies().iter().any(|a| {
                        g.local_position()
                            .preview_for(Side::Gote, a)
                            .is_ok_and(|e| {
                                g.royal_squares(Side::Sente)
                                    .iter()
                                    .all(|s| e.captures.iter().any(|c| c.square == *s))
                            })
                    }),
            );
            put(
                &mut out,
                "inferred_preceding_move",
                g.position().accepted_plies() != 0,
            );
        }
        "CHU-MATERIAL-DECLINE-COUNTERCAPTURE" => {
            put(
                &mut out,
                "before_material_award",
                matches!(journey.before.conclusion, Some(Conclusion::Material { .. })),
            );
            let initial = fixture_game(f)?;
            let replies = initial.immediate_material_replies();
            if replies.len() != 1 {
                return Err(format!(
                    "expected independently unique equalizer but native returned {}",
                    replies.len()
                ));
            }
            put(&mut out, "available_equalizer", action_value(&replies[0]));
            put(
                &mut out,
                "after_winner",
                g.conclusion().and_then(Conclusion::winner),
            );
        }
        "CHU-MATERIAL-AGREEMENT" => {
            put(
                &mut out,
                "offer_alone_concludes",
                g.event_prefix(1)
                    .map_err(|e| e.to_string())?
                    .conclusion
                    .is_some(),
            );
        }
        "CHU-MATERIAL-DEAD-EXCLUSION" => {
            let dead: Vec<_> = g
                .position()
                .pieces()
                .filter(|(s, p)| dead_for_material(*s, *p))
                .collect();
            put(
                &mut out,
                "ignored_for_count_only",
                dead.iter().map(|(s, _)| *s).collect::<Vec<_>>(),
            );
            put(
                &mut out,
                "dead_pieces_remain_on_board",
                !dead.is_empty()
                    && dead
                        .into_iter()
                        .all(|(s, p)| g.position().piece(s) == Some(p)),
            );
        }
        _ => {}
    }
    Ok(out)
}

fn different_history(f: &Fixture) -> Result<BTreeMap<String, Value>, String> {
    let mut out = BTreeMap::new();
    let mut a = fixture_game(f)?;
    let mut b = fixture_game(f)?;
    for v in f.given["history_A"].as_array().unwrap() {
        play(&mut a, action(v)).map_err(|e| e.to_string())?;
    }
    for v in f.given["history_B"].as_array().unwrap() {
        play(&mut b, action(v)).map_err(|e| e.to_string())?;
    }
    put(
        &mut out,
        "same_current_board_and_turn",
        a.position().pieces().collect::<Vec<_>>() == b.position().pieces().collect::<Vec<_>>()
            && a.position().side_to_move() == b.position().side_to_move(),
    );
    put(&mut out, "known_occurrences_A", root_occurrences(&a));
    put(&mut out, "known_occurrences_B", root_occurrences(&b));
    put(
        &mut out,
        "same_search_context",
        a.context().same_semantics(&b.context()),
    );
    for (label, g) in [("next_cycle_A", &mut a), ("next_cycle_B", &mut b)] {
        let mut result = "ongoing";
        for v in f.given["next_cycle"].as_array().unwrap() {
            if let Err(e) = play(g, action(v)) {
                if matches!(e, GameError::RepetitionDeparture) && g.snapshot().board_ply == 10 {
                    result = "move11_native_must_vary_refusal"
                } else {
                    return Err(e.to_string());
                }
                break;
            }
        }
        put(&mut out, label, result);
    }
    Ok(out)
}
fn lion_history(f: &Fixture) -> Result<BTreeMap<String, Value>, String> {
    let corpus = Corpus::embedded()?;
    let referenced = corpus
        .fixture(f.given["fixture_ref"].as_str().unwrap())
        .unwrap();
    let mut known = fixture_game(referenced)?;
    for v in referenced.given["commands"]
        .as_array()
        .unwrap()
        .iter()
        .take(f.given["prefix_commands"].as_u64().unwrap() as usize)
    {
        play(&mut known, action(v)).map_err(|e| e.to_string())?;
    }
    let entries: Vec<_> = known.position().pieces().collect();
    let mut fresh = Game::from_root(
        Position::from_diagram(
            &entries,
            known.position().side_to_move(),
            RootMetadata::diagram(known.position().declared_total_plies()),
        )
        .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let mut out = BTreeMap::new();
    put(
        &mut out,
        "same_repetition_or_search_identity",
        known.repetition_key() == fresh.repetition_key()
            || known.context().same_semantics(&fresh.context()),
    );
    put(
        &mut out,
        "known_history_capture_admitted",
        play(&mut known, action(&f.given["next_action"])).is_ok(),
    );
    put(
        &mut out,
        "fresh_diagram_capture_admitted",
        play(&mut fresh, action(&f.given["next_action"])).is_ok(),
    );
    Ok(out)
}

fn route_identity(f: &Fixture) -> Result<BTreeMap<String, Value>, String> {
    let mut out = BTreeMap::new();
    let mut a = fixture_game(f)?;
    let mut b = fixture_game(f)?;
    let aa = action(&f.given["routes"][0]);
    let bb = action(&f.given["routes"][1]);
    play(&mut a, aa.clone()).map_err(|e| e.to_string())?;
    play(&mut b, bb.clone()).map_err(|e| e.to_string())?;
    put(
        &mut out,
        "both_are_quiet_return",
        a.events()[0].effects.as_ref().unwrap().quiet_return
            && b.events()[0].effects.as_ref().unwrap().quiet_return,
    );
    put(
        &mut out,
        "same_board_result",
        a.position().state() == b.position().state(),
    );
    put(&mut out, "same_complete_action", aa == bb);
    put(
        &mut out,
        "same_cycle_transition",
        a.repetition_key() == b.repetition_key(),
    );
    // Build the actual four-turn repetition using an ordinary reversible remote
    // King move. The two supplied quiet-return routes remain actual native actions.
    let mut cycle = fixture_game(f)?;
    let king = cycle
        .position()
        .pieces()
        .find(|(_, p)| p.side == Side::Gote && p.base == Base::King)
        .unwrap()
        .0;
    let step = cycle
        .local_position()
        .legal_actions_for(Side::Gote)
        .into_iter()
        .find(|a| {
            a.from == king
                && !a.promote
                && a.route.len() == 1
                && cycle.position().piece(a.route[0]).is_none()
        })
        .ok_or("missing remote native King step")?;
    let back = Action::single(step.route[0], king);
    for _ in 0..2 {
        for turn in [aa.clone(), step.clone(), aa.clone(), back.clone()] {
            play(&mut cycle, turn).map_err(|e| e.to_string())?;
        }
    }
    play(&mut cycle, aa.clone()).map_err(|e| e.to_string())?;
    play(&mut cycle, step).map_err(|e| e.to_string())?;
    let first = play(&mut cycle, aa);
    let second = play(&mut cycle, bb);
    put(
        &mut out,
        "alternate_quiet_route_does_not_evade_obligation",
        matches!(first, Err(GameError::RepetitionDeparture))
            && matches!(second, Err(GameError::RepetitionDeparture)),
    );
    Ok(out)
}

fn invalid(f: &Fixture) -> Result<BTreeMap<String, Value>, String> {
    let mut g = fixture_game(f)?;
    let before = g.context();
    let caches = g.cache_snapshot();
    let mut raw = f.given["raw_request"].clone();
    let actor = raw
        .get("side")
        .map(side)
        .unwrap_or(g.position().side_to_move());
    if raw.get("action").is_some() {
        raw = raw["action"].clone();
    }
    let mut native = raw.clone();
    if native["kind"] == "move" {
        native.as_object_mut().unwrap().remove("kind");
    }
    let parse = Action::from_json(&native.to_string());
    let mut action_error = None;
    let mut parse_error = None;
    let admitted = match parse {
        Ok(a) => match submit(&mut g, actor, GameCommand::Play { action: a }) {
            Ok(_) => true,
            Err(e) => {
                action_error = Some(e);
                false
            }
        },
        Err(e) => {
            parse_error = Some(e);
            false
        }
    };
    // Every explanatory label below is conditional on an actual native refusal.
    // The fixture action grammar is mapped to the native Action shape without
    // deleting unknown supplied fields such as client victim lists.
    let reason = match (&action_error, &parse_error) {
        (Some(GameError::WrongActor), _) => "wrong_side",
        (Some(GameError::Action(ActionError::InvalidRoute)), _)
            if raw["route"].as_array().is_some_and(Vec::is_empty) =>
        {
            "no_stationary_null_action"
        }
        (Some(GameError::Action(ActionError::InvalidRoute)), _) => "more_than_two_legs",
        (Some(GameError::Action(ActionError::Promotion)), _) => "unpromotable_king",
        (_, Some(RecordError::Json)) if raw["kind"] == "drop" => "no_drops",
        (_, Some(RecordError::Json)) if raw.get("victims").is_some() => {
            "client_victims_not_authority"
        }
        (_, Some(RecordError::Json)) => "offboard",
        _ => "unexpected_native_result",
    };
    let mut out = BTreeMap::new();
    put(&mut out, "admitted", admitted);
    put(&mut out, "reason", reason);
    put(
        &mut out,
        "all_state_history_and_counters_unchanged",
        g.context() == before && g.cache_snapshot() == caches,
    );
    Ok(out)
}
fn reversal(f: &Fixture) -> Result<BTreeMap<String, Value>, String> {
    let corpus = Corpus::embedded()?;
    let reference = corpus
        .fixture(f.given["fixture_ref"].as_str().unwrap())
        .unwrap();
    let mut game = fixture_game(reference)?;
    let before = logical(&game);
    let runtime = game.context();
    let mut undos = Vec::new();
    let inputs: Vec<_> = if let Some(a) = reference.given.get("action") {
        vec![a.clone()]
    } else {
        reference.given["commands"].as_array().cloned().unwrap()
    };
    for v in inputs {
        let (actor, cmd) = command(&v, &game)?;
        undos.push(submit(&mut game, actor, cmd).map_err(|e| e.to_string())?);
    }
    let after = logical(&game);
    let mut redos = Vec::new();
    for undo in undos.iter_mut().rev() {
        redos.push(game.undo(undo).map_err(|e| e.to_string())?);
    }
    let mut out = BTreeMap::new();
    put(
        &mut out,
        "apply_then_undo",
        json!({"before":before,"after":logical(&game)}),
    );
    put(
        &mut out,
        "runtime_authority",
        json!({"before":format!("{:?}",runtime.runtime()),"after":format!("{:?}",game.context().runtime())}),
    );
    for redo in redos.iter_mut().rev() {
        game.redo(redo).map_err(|e| e.to_string())?;
    }
    put(
        &mut out,
        "redo",
        json!({"before":after,"after":logical(&game)}),
    );
    Ok(out)
}
fn records(f: &Fixture) -> Result<BTreeMap<String, Value>, String> {
    let corpus = Corpus::embedded()?;
    let reference = corpus
        .fixture(f.given["fixture_ref"].as_str().unwrap())
        .unwrap();
    let game = run(reference)?.game;
    let record = GameRecord::from_game(&game).map_err(|e| e.to_string())?;
    let before = game.context();
    let mut out = BTreeMap::new();
    if f.id == "CHU-RECORD-ROUNDTRIP" {
        let replay = GameRecord::from_json(record.as_json())
            .and_then(|r| r.to_game())
            .map_err(|e| e.to_string())?;
        put(
            &mut out,
            "roundtrip",
            json!({"before":logical(&game),"after":logical(&replay)}),
        );
        put(
            &mut out,
            "derive_facts_from_native_replay",
            game.context().same_semantics(&replay.context())
                && game.context().runtime() != replay.context().runtime(),
        );
        return Ok(out);
    }
    let mutation = f.given["mutation"].as_str().unwrap();
    let mut raw: Value = serde_json::from_str(record.as_json()).unwrap();
    let mutated = match mutation {
        "duplicate_nested_field" => {
            let needle = "\"promoted\":false";
            if !record.as_json().contains(needle) {
                return Err("record lacks nested promoted field for duplicate probe".into());
            }
            record
                .as_json()
                .replacen(needle, "\"promoted\":false,\"promoted\":false", 1)
        }
        "unknown_profile" => {
            raw["profile"] = json!("mshogi.chu.unsupported.v99");
            raw.to_string()
        }
        "conflicting_profile_edition" => {
            raw["edition"] = json!("not-the-selected-edition");
            raw.to_string()
        }
        "wrong_native_schema" => {
            raw["schema_version"] = json!(999);
            raw.to_string()
        }
        "tampered_capture_effect" => {
            let effect = raw["events"][0]["effects"]["captures"]
                .as_array_mut()
                .ok_or("missing real native captured effects")?;
            if effect.is_empty() {
                return Err("source fixture did not actually capture".into());
            }
            effect.remove(0);
            raw.to_string()
        }
        "tampered_conclusion" => {
            raw["conclusion"] = json!({"kind":"agreed_draw"});
            raw.to_string()
        }
        "event_reorder" => {
            raw["events"].as_array_mut().unwrap().swap(0, 1);
            raw.to_string()
        }
        "missing_root_provenance" => {
            raw["root"]["state"].as_object_mut().unwrap().remove("root");
            raw.to_string()
        }
        "oversized_record" => format!("{}{}", record.as_json(), " ".repeat(MAX_GAME_RECORD_BYTES)),
        _ => return Err("unknown fixture mutation".into()),
    };
    let result = GameRecord::from_json(&mutated);
    put(&mut out, "replay_admitted", result.is_ok());
    put(
        &mut out,
        "accepted_live_game_unchanged",
        game.context() == before,
    );
    if result.is_err() {
        put(
            &mut out,
            "diagnostic_kind",
            "record_or_resource_failure_not_chess_outcome",
        );
    }
    Ok(out)
}

#[test]
fn complete_independently_expected_native_corpus() {
    let corpus = Corpus::embedded().unwrap();
    let report = mshogi_conformance::chu::run_adapter(&corpus, &CompleteAdapter::default());
    let failures: Vec<_> = report
        .assertions
        .iter()
        .filter(|a| a.status != mshogi_conformance::assertions::Status::Passed)
        .collect();
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
    assert_eq!(report.native_rows_passed, 240);
    assert_eq!(report.native_rows_pending, 0);
    assert_eq!(report.failed, 0);
    assert_eq!(report.later_consumer_requirements.len(), 10);
    eprintln!(
        "Chu complete native corpus: {} fields passed; {} rows passed; {} documentary annotations; {} future consumer rows remain pending",
        report.passed,
        report.native_rows_passed,
        report.documentary_annotations,
        report.later_consumer_requirements.len()
    );
    if let Ok(path) = std::env::var("MSHOGI_CHU_CONFORMANCE_REPORT") {
        std::fs::write(path, serde_json::to_string_pretty(&report).unwrap()).unwrap();
    }
}
