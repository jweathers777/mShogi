//! Display-only JSA notation over admitted native choices and known history.
//!
//! Source: https://www.shogi.or.jp/faq/kihuhyouki/ (basic matters and ambiguity
//! diagrams). Numeric file/rank are the JSA recording convention. This adapter
//! neither parses notation nor changes an action, position, or game outcome.

use mshogi_core::shogi::{Action, Piece, PieceKind, Position, Side, Square};

pub(super) fn notation(position: &Position, action: Action, previous: Option<Square>) -> String {
    let legal = position.legal_actions();
    let (to, kind, promoted) = match action {
        Action::Move { from, to, .. } => {
            let piece = position
                .piece(from)
                .expect("admitted board action has a source");
            (to, piece.kind, piece.promoted)
        }
        Action::Drop { kind, to } => (to, kind, false),
    };
    let mut text = if previous == Some(to) {
        "同".to_owned()
    } else {
        format!("{}{}", digit(to.file()), digit(to.rank()))
    };
    text.push_str(piece_name(kind, promoted));

    // Complete legal alternatives, including both promotion choices and check
    // evasions, come from native rules. Deduplicate sources: two promotion
    // choices of one piece are not two pieces for JSA disambiguation.
    let mut origins = Vec::new();
    for candidate in &legal {
        if let Action::Move {
            from, to: target, ..
        } = *candidate
            && target == to
            && position.piece(from)
                == Some(Piece {
                    side: position.side_to_move(),
                    kind,
                    promoted,
                })
            && !origins.contains(&from)
        {
            origins.push(from);
        }
    }
    match action {
        Action::Drop { .. } => {
            if !origins.is_empty() {
                text.push('打');
            }
        }
        Action::Move { from, promote, .. } => {
            if origins.len() > 1 {
                text.push_str(&ambiguity(
                    position.side_to_move(),
                    kind,
                    promoted,
                    from,
                    to,
                    &origins,
                ));
            }
            if promote {
                text.push('成');
            } else if legal.contains(&Action::Move {
                from,
                to,
                promote: true,
            }) {
                text.push_str("不成");
            }
        }
    }
    text
}

fn digit(value: u8) -> char {
    char::from_u32(u32::from('０') + u32::from(value)).expect("bounded native coordinate")
}

fn piece_name(kind: PieceKind, promoted: bool) -> &'static str {
    match (kind, promoted) {
        (PieceKind::King, _) => "玉",
        (PieceKind::Gold, _) => "金",
        (PieceKind::Rook, false) => "飛",
        (PieceKind::Rook, true) => "竜",
        (PieceKind::Bishop, false) => "角",
        (PieceKind::Bishop, true) => "馬",
        (PieceKind::Silver, false) => "銀",
        (PieceKind::Silver, true) => "成銀",
        (PieceKind::Knight, false) => "桂",
        (PieceKind::Knight, true) => "成桂",
        (PieceKind::Lance, false) => "香",
        (PieceKind::Lance, true) => "成香",
        (PieceKind::Pawn, false) => "歩",
        (PieceKind::Pawn, true) => "と",
    }
}

fn motion(side: Side, from: Square, to: Square) -> &'static str {
    match (from.rank().cmp(&to.rank()), side) {
        (std::cmp::Ordering::Equal, _) => "寄",
        (std::cmp::Ordering::Greater, Side::Sente) | (std::cmp::Ordering::Less, Side::Gote) => "上",
        _ => "引",
    }
}

fn ambiguity(
    side: Side,
    kind: PieceKind,
    promoted: bool,
    from: Square,
    to: Square,
    origins: &[Square],
) -> String {
    let movement = motion(side, from, to);
    if origins
        .iter()
        .filter(|source| motion(side, **source, to) == movement)
        .count()
        == 1
    {
        return movement.to_owned();
    }
    // JSA's Dragon/Horse diagrams explicitly forbid 直. Their two native
    // pieces use the relative left/right source, including a straight path.
    let horse_or_dragon = promoted && matches!(kind, PieceKind::Bishop | PieceKind::Rook);
    let relative = |source: Square| {
        let comparison = if horse_or_dragon {
            source
                .file()
                .cmp(&origins.iter().map(|square| square.file()).min().unwrap())
        } else {
            source.file().cmp(&to.file())
        };
        match (comparison, side) {
            (std::cmp::Ordering::Equal, _) if !horse_or_dragon => {
                if motion(side, source, to) == "上" {
                    "直"
                } else {
                    ""
                }
            }
            (std::cmp::Ordering::Greater, Side::Sente)
            | (std::cmp::Ordering::Equal | std::cmp::Ordering::Less, Side::Gote) => "左",
            _ => "右",
        }
    };
    let location = relative(from);
    if origins
        .iter()
        .filter(|source| relative(**source) == location)
        .count()
        == 1
    {
        location.to_owned()
    } else {
        format!("{location}{movement}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect(sfen: &str, usi: &str, previous: Option<&str>, expected: &str) {
        let position = Position::from_sfen(sfen).expect("independently specified diagram");
        let action = usi.parse().unwrap();
        assert!(
            position.legal_actions().contains(&action),
            "fixture action {usi} must be native legal"
        );
        assert_eq!(
            notation(
                &position,
                action,
                previous.map(|square| square.parse().unwrap())
            ),
            expected
        );
    }

    #[test]
    fn basic_numeric_coordinates_known_same_target_and_real_promoted_names() {
        expect(mshogi_core::shogi::INITIAL_SFEN, "7g7f", None, "７６歩");
        // The opposing piece occupies the actual preceding known target; the
        // same imported board without that history must retain coordinates.
        let capture = "k8/7B1/9/9/9/9/9/1b7/8K b - 1";
        expect(capture, "2b8h+", Some("8h"), "同角成");
        expect(capture, "2b8h+", None, "８８角成");
        // Imported counters establish no earlier destination by themselves.
        expect("8k/9/9/9/9/9/2P6/9/8K b - 91", "7g7f", None, "７６歩");
        for (piece, name) in [
            ("+P", "と"),
            ("+L", "成香"),
            ("+N", "成桂"),
            ("+S", "成銀"),
            ("+B", "馬"),
            ("+R", "竜"),
        ] {
            expect(
                &format!("8k/9/9/9/4{piece}4/9/9/9/8K b - 1"),
                "5e5d",
                None,
                &format!("５４{name}"),
            );
        }
    }

    #[test]
    fn promotion_optional_compulsory_and_contextual_drop_mark() {
        let silver = "8k/9/9/4S4/9/9/9/9/8K b - 1";
        expect(silver, "5d5c+", None, "５３銀成");
        expect(silver, "5d5c", None, "５３銀不成");
        expect("k8/8P/9/9/9/9/9/9/8K b - 1", "1b1a+", None, "１１歩成");
        expect("8k/9/9/9/9/9/9/9/5G2K b G 1", "G*5h", None, "５８金打");
        expect("8k/9/9/9/9/9/9/9/8K b G 1", "G*5h", None, "５８金");
        expect("8k/9/9/9/9/9/9/9/5G2K b G 1", "4i5h", None, "５８金");
        expect("8k/9/9/9/9/4s4/9/9/8K w - 1", "5f5g+", None, "５７銀成");
        expect("8k/9/9/9/9/4s4/9/9/8K w - 1", "5f5g", None, "５７銀不成");
    }

    #[test]
    fn jsa_motion_first_then_mover_relative_left_right_straight() {
        let gold = "8k/2G6/G8/9/9/9/9/9/8K b - 1";
        expect(gold, "9c8b", None, "８２金上");
        expect(gold, "7b8b", None, "８２金寄");
        let two = "8k/G1G6/9/9/9/9/9/9/8K b - 1";
        expect(two, "9b8a", None, "８１金左");
        expect(two, "7b8a", None, "８１金右");
        let three = "8k/9/3GGG3/9/9/9/9/9/8K b - 1";
        expect(three, "6c5b", None, "５２金左");
        expect(three, "5c5b", None, "５２金直");
        expect(three, "4c5b", None, "５２金右");
        let gote = "8k/9/9/9/9/9/3ggg3/9/8K w - 1";
        expect(gote, "6g5h", None, "５８金右");
        expect(gote, "5g5h", None, "５８金直");
        expect(gote, "4g5h", None, "５８金左");
    }

    #[test]
    fn jsa_five_tokins_need_both_motion_and_location_only_when_necessary() {
        let five = "8k/9/9/9/9/9/1+P7/+P8/+P+P+P5K b - 1";
        expect(five, "7i8h", None, "８８と右");
        expect(five, "8i8h", None, "８８と直");
        expect(five, "9i8h", None, "８８と左上");
        expect(five, "9h8h", None, "８８と寄");
        expect(five, "8g8h", None, "８８と引");
    }

    #[test]
    fn pinned_pseudo_alternative_does_not_invent_disambiguation() {
        // The 5h Gold has a geometric path to 4g but exposes its King to the
        // 5a Rook. Only the native-legal 4h Gold is a candidate.
        expect("4r3k/9/9/9/9/9/9/4GG3/4K4 b - 1", "4h4g", None, "４７金");
    }

    #[test]
    fn jsa_dragons_and_horses_never_use_straight() {
        let dragons = "8k/9/9/9/9/9/9/9/+R+R6K b - 1";
        expect(dragons, "9i8h", None, "８８竜左");
        expect(dragons, "8i8h", None, "８８竜右");
        let horses = "+B+B6k/9/9/9/9/9/9/9/8K b - 1";
        expect(horses, "9a8b", None, "８２馬左");
        expect(horses, "8a8b", None, "８２馬右");
        let gote = "+r+r6k/9/9/9/9/9/9/9/8K w - 1";
        expect(gote, "9a8b", None, "８２竜右");
        expect(gote, "8a8b", None, "８２竜左");
    }
}
