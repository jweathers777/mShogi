//! Conventional Chess candidate: exact alpha-beta and transparent tapered terms.
//!
//! No transposition cache, selective pruning, quiescence, training or independent
//! legality/adjudication is introduced. Static evaluation is a heuristic only;
//! native outcomes and claims precede it at every searched turn boundary.
use crate::search::{
    MATE_SCORE, MAX_DEPTH, SearchChoice, SearchDecision, SearchError, SearchLimits, SearchResult,
    SearchStatus,
};
use mshogi_core::chess::{
    Action, AdjudicatedGame, ChessOutcome, ClaimPolicy, Color, PieceKind, Position, Promotion,
    Square,
};
use std::cmp::Reverse;

/// Exact conventional search family, separate from the frozen full-width baseline.
pub const SEARCH_ID: &str = "mshogi.chess.alpha-beta-capture-order.v1";
/// Original arithmetic positional parameters; no copied table or trained weights.
pub const EVALUATOR_ID: &str = "mshogi.chess.tapered-position.v1";
// Initial total non-pawn material with the existing material values. Promotion
// may exceed this; cap phase rather than extrapolating the king/positional terms.
const MIDDLE_PHASE: i32 = 6_400;
const INFINITY: i32 = MATE_SCORE + 1;

/// Material and positional contributions, retaining a transparent decomposition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Evaluation {
    /// White's native material, excluding the king.
    pub white_material: i32,
    /// Black's native material, excluding the king.
    pub black_material: i32,
    /// White's tapered positional contribution.
    pub white_positional: i32,
    /// Black's tapered positional contribution.
    pub black_positional: i32,
    /// Shared middle-game phase in 0..=6400; zero has endgame king terms.
    pub middle_phase: i32,
}
impl Evaluation {
    /// Heuristic from the named side's perspective; zero is not a draw outcome.
    pub const fn score_for(self, side: Color) -> i32 {
        let white = self.white_material + self.white_positional;
        let black = self.black_material + self.black_positional;
        match side {
            Color::White => white - black,
            Color::Black => black - white,
        }
    }
}
fn side_index(side: Color) -> usize {
    usize::from(side == Color::Black)
}
fn centrality(file: u8, rank: u8) -> i32 {
    // 0 at corners, 6 on the four central squares; symmetric in both axes.
    (14 - (2 * i32::from(file) - 7).abs() - (2 * i32::from(rank) - 7).abs()) / 2
}
fn file_centrality(file: u8) -> i32 {
    3 - (i32::from(file) - 3).abs().min((i32::from(file) - 4).abs())
}

/// Untrained material plus original, symmetric positional arithmetic.
///
/// Material values are the frozen baseline's 100/320/330/500/900. Middle/end
/// phase is determined only by remaining non-pawn material, capped at the
/// initial total. Knights/bishops prefer central development; pawns receive
/// advancement/structure/passed-pawn terms; rooks prefer open files and the
/// seventh rank. Middle-game kings prefer shelter and a castled square while
/// endgame kings prefer centrality. Queen development is mildly discouraged
/// while most non-pawn material remains. Each term is a documented heuristic,
/// not a game rule or a measured exchange/strength value.
///
/// No turn, counter, claim, history or terminal result is inferred here.
/// Training/model loading and incremental feature state are outside this module.
pub fn evaluate(position: &Position) -> Evaluation {
    let mut material = [0i32; 2];
    let mut pawn_files = [[0u8; 8]; 2];
    let mut pawn_ranks = [[0u8; 8]; 2];
    let mut bishops = [0u8; 2];
    let mut phase = 0;
    for rank in 0..8 {
        for file in 0..8 {
            let square = Square::new(file, rank).expect("bounded board coordinates");
            let Some(piece) = position.piece_at(square) else {
                continue;
            };
            let side = side_index(piece.color);
            let value = crate::chess::piece_value(piece.kind);
            material[side] += value;
            match piece.kind {
                PieceKind::Pawn => {
                    pawn_files[side][usize::from(file)] += 1;
                    pawn_ranks[side][usize::from(file)] |= 1 << rank;
                }
                PieceKind::King => {}
                _ => phase += value,
            }
            bishops[side] += u8::from(piece.kind == PieceKind::Bishop);
        }
    }
    phase = phase.min(MIDDLE_PHASE);
    let mut middle = [0i32; 2];
    let mut ending = [0i32; 2];
    for rank in 0..8 {
        for file in 0..8 {
            let square = Square::new(file, rank).expect("bounded board coordinates");
            let Some(piece) = position.piece_at(square) else {
                continue;
            };
            let side = side_index(piece.color);
            let other = 1 - side;
            let relative_rank = if piece.color == Color::White {
                rank
            } else {
                7 - rank
            };
            let advancement = (i32::from(relative_rank) - 1).max(0);
            let center = centrality(file, rank);
            let file_center = file_centrality(file);
            let file_index = usize::from(file);
            let (mg, eg) = match piece.kind {
                PieceKind::Pawn => {
                    let isolated = (file == 0 || pawn_files[side][file_index - 1] == 0)
                        && (file == 7 || pawn_files[side][file_index + 1] == 0);
                    let doubled = pawn_files[side][file_index] > 1;
                    let ahead_mask = if piece.color == Color::White {
                        (0xffu16 << (u32::from(rank) + 1)) as u8
                    } else {
                        ((1u16 << rank) - 1) as u8
                    };
                    let passed = (file_index.saturating_sub(1)..=(file_index + 1).min(7))
                        .all(|f| pawn_ranks[other][f] & ahead_mask == 0);
                    (
                        8 * advancement + 5 * file_center
                            - 10 * i32::from(isolated)
                            - 10 * i32::from(doubled)
                            + 4 * advancement * advancement * i32::from(passed),
                        14 * advancement + 2 * file_center
                            - 8 * i32::from(isolated)
                            - 16 * i32::from(doubled)
                            + (10 + 10 * advancement * advancement) * i32::from(passed),
                    )
                }
                PieceKind::Knight => (12 * center - 8 * i32::from(relative_rank == 0), 8 * center),
                PieceKind::Bishop => (
                    7 * center + 3 * advancement - 8 * i32::from(relative_rank == 0),
                    6 * center,
                ),
                PieceKind::Rook => {
                    let open = if pawn_files[side][file_index] == 0 {
                        if pawn_files[other][file_index] == 0 {
                            16
                        } else {
                            8
                        }
                    } else {
                        0
                    };
                    (
                        2 * center + open + 18 * i32::from(relative_rank == 6),
                        4 * center + open + 24 * i32::from(relative_rank == 6),
                    )
                }
                PieceKind::Queen => (
                    3 * center - 8 * i32::from(phase > 4_800 && relative_rank > 1),
                    5 * center,
                ),
                PieceKind::King => {
                    let castled = relative_rank == 0 && (file == 2 || file == 6);
                    let shelter_rank = if piece.color == Color::White {
                        rank.checked_add(1).filter(|r| *r < 8)
                    } else {
                        rank.checked_sub(1)
                    };
                    let shelter = shelter_rank.map_or(0, |r| {
                        (file_index.saturating_sub(1)..=(file_index + 1).min(7))
                            .filter(|f| pawn_ranks[side][*f] & (1 << r) != 0)
                            .count() as i32
                    });
                    (
                        -12 * center - 8 * (i32::from(relative_rank) - 1).max(0)
                            + 28 * i32::from(castled)
                            + 8 * shelter,
                        14 * center,
                    )
                }
            };
            middle[side] += mg;
            ending[side] += eg;
        }
    }
    for side in 0..2 {
        if bishops[side] >= 2 {
            middle[side] += 24;
            ending[side] += 32;
        }
    }
    let taper =
        |side: usize| (middle[side] * phase + ending[side] * (MIDDLE_PHASE - phase)) / MIDDLE_PHASE;
    Evaluation {
        white_material: material[0],
        black_material: material[1],
        white_positional: taper(0),
        black_positional: taper(1),
        middle_phase: phase,
    }
}
fn candidate_score(position: &Position) -> i32 {
    evaluate(position).score_for(position.side_to_move())
}

/// Search privately with the candidate evaluator and exact alpha-beta backup.
pub fn search(game: &AdjudicatedGame, limits: SearchLimits) -> Result<SearchResult, SearchError> {
    search_with_stop(game, limits, || false)
}

/// Search a native private clone, publishing only fully completed iterations.
///
/// Claims, including a terminal claim at a depth-zero turn, use native Game
/// admission and do not consume an extra board ply. Forced AutoClaim precedes
/// static evaluation and all board moves; optional claims compete at zero with
/// ties preferring the claim. Native legal captures/promotions are ordered first,
/// descending by `16*(victim value + promotion gain) - attacker value` (a king's
/// ordering-only attacker value is 20000); equal priorities use lexical UCI.
/// Equal-scoring moves retain the first choice under that deterministic order,
/// which may differ from the frozen baseline's purely lexical tie preference.
/// These static ordering estimates never admit a move or resolve its effects.
/// There is no cached board-only identity. Node visits include all
/// attempted iterations, and every successful native push is undone before a
/// halt or error is propagated. Stop polling remains cooperative per node.
pub fn search_with_stop(
    game: &AdjudicatedGame,
    limits: SearchLimits,
    mut stop: impl FnMut() -> bool,
) -> Result<SearchResult, SearchError> {
    search_with_evaluator(game, limits, &mut stop, candidate_score)
}
fn search_with_evaluator(
    game: &AdjudicatedGame,
    limits: SearchLimits,
    stop: &mut dyn FnMut() -> bool,
    evaluator: fn(&Position) -> i32,
) -> Result<SearchResult, SearchError> {
    if !(1..=MAX_DEPTH).contains(&limits.depth) {
        return Err(SearchError::InvalidDepth(limits.depth));
    }
    let mut tree = ChessTree {
        game: game.clone(),
        evaluator,
    };
    let mut work = Work {
        nodes: 0,
        limit: limits.nodes,
        stop,
    };
    let mut result = SearchResult {
        decision: None,
        score: None,
        completed_depth: 0,
        nodes: 0,
        principal_variation: vec![],
        status: SearchStatus::Complete,
    };
    for depth in 1..=limits.depth {
        let searched = negamax(&mut tree, depth, 0, -INFINITY, INFINITY, &mut work);
        debug_assert_eq!(tree.game, *game, "balanced native search restoration");
        match searched {
            Ok(value) => {
                result.decision = value
                    .pv
                    .first()
                    .copied()
                    .map(SearchDecision::Choose)
                    .or_else(|| game.outcome().map(SearchDecision::Finished));
                result.score = Some(value.score);
                result.completed_depth = depth;
                result.principal_variation = value.pv;
                if matches!(result.decision, Some(SearchDecision::Finished(_))) {
                    break;
                }
            }
            Err(Halt::Limit(status)) => {
                result.status = status;
                break;
            }
            Err(Halt::Error(error)) => return Err(error),
        }
    }
    result.nodes = work.nodes;
    Ok(result)
}

// A private finite-tree seam tests exact backup independently of native rules.
// It is not a public generic game framework or alternate rules implementation.
trait Tree {
    type Choice: Copy;
    fn terminal(&self, ply: u16) -> Option<i32>;
    fn claim(&self) -> Option<(Self::Choice, bool)>;
    fn evaluate(&self) -> i32;
    fn actions(&self) -> Vec<Self::Choice>;
    fn push(&mut self, choice: Self::Choice) -> Result<(), SearchError>;
    fn pop(&mut self);
}
struct ChessTree {
    game: AdjudicatedGame,
    evaluator: fn(&Position) -> i32,
}
// This heuristic receives an action already supplied by native legal_actions.
// In particular, a native legal pawn diagonal to an empty EP target is the
// capture of a pawn. This labels ordering only: native apply still resolves and
// checks every actual capture, promotion and other effect before transition.
fn action_priority(position: &Position, action: Action) -> (bool, i32) {
    let mover = position
        .piece_at(action.from)
        .expect("native legal action has a moving piece");
    let victim = position.piece_at(action.to).map_or_else(
        || {
            if mover.kind == PieceKind::Pawn
                && action.from.file() != action.to.file()
                && position.en_passant_target() == Some(action.to)
            {
                crate::chess::piece_value(PieceKind::Pawn)
            } else {
                0
            }
        },
        |piece| crate::chess::piece_value(piece.kind),
    );
    let promotion_gain = action.promotion.map_or(0, |promotion| {
        let face = match promotion {
            Promotion::Queen => PieceKind::Queen,
            Promotion::Rook => PieceKind::Rook,
            Promotion::Bishop => PieceKind::Bishop,
            Promotion::Knight => PieceKind::Knight,
        };
        crate::chess::piece_value(face) - crate::chess::piece_value(PieceKind::Pawn)
    });
    let attacker = if mover.kind == PieceKind::King {
        20_000 // Ordering only; the evaluator's native king material stays zero.
    } else {
        crate::chess::piece_value(mover.kind)
    };
    (
        victim > 0 || promotion_gain > 0,
        16 * (victim + promotion_gain) - attacker,
    )
}
impl Tree for ChessTree {
    type Choice = SearchChoice;
    fn terminal(&self, ply: u16) -> Option<i32> {
        self.game.outcome().map(|outcome| match outcome {
            ChessOutcome::Draw(_) => 0,
            ChessOutcome::Checkmate { winner } => {
                if winner == self.game.position().side_to_move() {
                    MATE_SCORE - i32::from(ply)
                } else {
                    -MATE_SCORE + i32::from(ply)
                }
            }
        })
    }
    fn claim(&self) -> Option<(Self::Choice, bool)> {
        self.game.available_claims().first().map(|&claim| {
            (
                SearchChoice::Claim(claim),
                self.game.history().claim_policy() == ClaimPolicy::AutoClaim,
            )
        })
    }
    fn evaluate(&self) -> i32 {
        (self.evaluator)(self.game.position())
    }
    fn actions(&self) -> Vec<Self::Choice> {
        let mut actions = self.game.position().legal_actions();
        actions.sort_by_cached_key(|action| {
            let (tactical, gain) = action_priority(self.game.position(), *action);
            // Quiet moves have no material-exchange ordering: their lexical
            // ties remain stable, rather than prioritizing cheap moving pieces.
            (
                Reverse(tactical),
                Reverse(if tactical { gain } else { 0 }),
                action.to_string(),
            )
        });
        actions.into_iter().map(SearchChoice::Move).collect()
    }
    fn push(&mut self, choice: Self::Choice) -> Result<(), SearchError> {
        let SearchChoice::Move(action) = choice else {
            unreachable!("claims are terminal choices, never board transitions")
        };
        self.game.apply(action).map_err(SearchError::Transition)
    }
    fn pop(&mut self) {
        self.game
            .undo()
            .expect("balanced private native transition");
    }
}
struct Value<A> {
    score: i32,
    pv: Vec<A>,
}
enum Halt {
    Limit(SearchStatus),
    Error(SearchError),
}
struct Work<'a> {
    nodes: u64,
    limit: Option<u64>,
    stop: &'a mut dyn FnMut() -> bool,
}
impl Work<'_> {
    fn visit(&mut self) -> Result<(), Halt> {
        if (self.stop)() {
            return Err(Halt::Limit(SearchStatus::Stopped));
        }
        if self.limit.is_some_and(|limit| self.nodes >= limit) {
            return Err(Halt::Limit(SearchStatus::NodeLimit));
        }
        self.nodes += 1;
        Ok(())
    }
}
fn negamax<T: Tree>(
    tree: &mut T,
    depth: u16,
    ply: u16,
    mut alpha: i32,
    beta: i32,
    work: &mut Work<'_>,
) -> Result<Value<T::Choice>, Halt> {
    work.visit()?;
    if let Some(score) = tree.terminal(ply) {
        return Ok(Value { score, pv: vec![] });
    }
    let claim = tree.claim();
    if let Some((choice, true)) = claim {
        return Ok(Value {
            score: 0,
            pv: vec![choice],
        });
    }
    if depth == 0 {
        let score = tree.evaluate();
        return Ok(if let Some((choice, _)) = claim.filter(|_| score <= 0) {
            Value {
                score: 0,
                pv: vec![choice],
            }
        } else {
            Value { score, pv: vec![] }
        });
    }
    let mut best = claim.map(|(choice, _)| Value {
        score: 0,
        pv: vec![choice],
    });
    if let Some((choice, _)) = claim {
        alpha = alpha.max(0);
        if alpha >= beta {
            return Ok(Value {
                score: 0,
                pv: vec![choice],
            });
        }
    }
    for choice in tree.actions() {
        tree.push(choice).map_err(Halt::Error)?;
        let child = negamax(tree, depth - 1, ply + 1, -beta, -alpha, work);
        tree.pop();
        let child = child?;
        let score = -child.score;
        if best.as_ref().is_none_or(|previous| score > previous.score) {
            let mut pv = vec![choice];
            pv.extend(child.pv);
            best = Some(Value { score, pv });
        }
        alpha = alpha.max(score);
        if alpha >= beta {
            break;
        }
    }
    Ok(best.expect("native nonterminal Chess position has a legal action"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_core::chess::{Action, ActionError, DrawReason, PlayError, STANDARD_START_FEN};

    fn game(fen: &str, policy: ClaimPolicy) -> AdjudicatedGame {
        AdjudicatedGame::from_fen(fen, policy).unwrap()
    }
    fn run(game: &AdjudicatedGame, depth: u16) -> SearchResult {
        let before = game.clone();
        let result = search(game, SearchLimits { depth, nodes: None }).unwrap();
        assert_eq!(*game, before);
        assert_eq!(result.status, SearchStatus::Complete);
        let mut replay = game.clone();
        let mut board_plies = 0;
        for (i, choice) in result.principal_variation.iter().enumerate() {
            assert!(replay.outcome().is_none());
            match choice {
                SearchChoice::Move(action) => {
                    assert!(replay.position().legal_actions().contains(action));
                    replay.apply(*action).unwrap();
                    board_plies += 1;
                }
                SearchChoice::Claim(claim) => {
                    assert_eq!(i + 1, result.principal_variation.len());
                    replay.claim(*claim).unwrap();
                }
            }
        }
        assert!(board_plies <= result.completed_depth);
        assert!(result.principal_variation.len() <= usize::from(result.completed_depth) + 1);
        result
    }
    fn chosen(result: &SearchResult) -> Action {
        let Some(SearchDecision::Choose(SearchChoice::Move(action))) = result.decision else {
            panic!("expected move: {result:?}");
        };
        action
    }
    fn material_score(position: &Position) -> i32 {
        crate::chess::evaluate(position).score_for(position.side_to_move())
    }
    #[test]
    fn cached_keys_preserve_complete_native_actions_and_lexical_priority_ties() {
        for fen in [
            STANDARD_START_FEN,
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            "7k/8/8/3pP3/8/8/8/KR6 w - d6 0 1",
            "7k/P7/8/8/8/8/8/KR6 w - - 0 1",
        ] {
            let g = game(fen, ClaimPolicy::ExplicitOnly);
            let mut original = g.position().legal_actions();
            original.sort_by_key(ToString::to_string);
            let actions = ChessTree {
                game: g.clone(),
                evaluator: candidate_score,
            }
            .actions();
            let mut complete = actions
                .iter()
                .map(|choice| {
                    let SearchChoice::Move(action) = choice else {
                        panic!("only board choices are ordered")
                    };
                    *action
                })
                .collect::<Vec<_>>();
            complete.sort_by_key(ToString::to_string);
            assert_eq!(
                complete, original,
                "ordering preserves every native complete action"
            );
            // Keep the cached-key optimization checked against uncached sorting;
            // separate hand-derived cases below establish the intended priority.
            let mut uncached = original;
            uncached.sort_by_key(|action| {
                let (tactical, gain) = action_priority(g.position(), *action);
                (
                    Reverse(tactical),
                    Reverse(if tactical { gain } else { 0 }),
                    action.to_string(),
                )
            });
            assert_eq!(
                actions,
                uncached
                    .into_iter()
                    .map(SearchChoice::Move)
                    .collect::<Vec<_>>()
            );
            for pair in actions.windows(2) {
                let [SearchChoice::Move(a), SearchChoice::Move(b)] = pair else {
                    unreachable!()
                };
                let pa = action_priority(g.position(), *a);
                let pb = action_priority(g.position(), *b);
                if pa == pb || (!pa.0 && !pb.0) {
                    assert!(a.to_string() < b.to_string());
                }
            }
        }
    }
    #[test]
    fn hand_derived_capture_promotion_and_ep_order_is_not_legality() {
        // Q900 by N320:14080; Q900 by R500:13900; R500 by P100:7900.
        let captures = game(
            "7k/8/8/8/1rN5/2P5/1q6/1R5K w - - 0 1",
            ClaimPolicy::ExplicitOnly,
        );
        let tree = ChessTree {
            game: captures.clone(),
            evaluator: candidate_score,
        };
        let actual = tree
            .actions()
            .iter()
            .map(|c| match c {
                SearchChoice::Move(a) => a.to_string(),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        assert_eq!(&actual[..3], ["c4b2", "b1b2", "c3b4"]);
        // Capturing R500 and promoting Q(+800) precedes capture+R(+400),
        // quiet Q(+800), capture+B(+230), capture+N(+220), then quiet R/B/N.
        let promotions = game("1r5k/P7/8/8/8/8/8/7K w - - 0 1", ClaimPolicy::ExplicitOnly);
        let tree = ChessTree {
            game: promotions.clone(),
            evaluator: candidate_score,
        };
        let actual = tree
            .actions()
            .iter()
            .map(|c| match c {
                SearchChoice::Move(a) => a.to_string(),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            &actual[..8],
            [
                "a7b8q", "a7b8r", "a7a8q", "a7b8b", "a7b8n", "a7a8r", "a7a8b", "a7a8n"
            ]
        );
        let ep = game(
            "7k/8/8/3pP3/8/8/8/KR6 w - d6 0 1",
            ClaimPolicy::ExplicitOnly,
        );
        let tree = ChessTree {
            game: ep.clone(),
            evaluator: candidate_score,
        };
        assert_eq!(
            tree.actions()[0],
            SearchChoice::Move("e5d6".parse().unwrap())
        );
        // None of these reads mutates native history or resolves a move itself.
        assert_eq!(tree.game, ep);
        assert_eq!(captures.history().ply_count(), 0);
        assert_eq!(promotions.history().ply_count(), 0);
    }
    fn reflected(position: &Position) -> Position {
        let fen = position.to_fen();
        let fields: Vec<_> = fen.split(' ').collect();
        assert_eq!((fields[2], fields[3]), ("-", "-"));
        let board = fields[0]
            .split('/')
            .rev()
            .map(|rank| {
                rank.chars()
                    .map(|c| {
                        if c.is_ascii_uppercase() {
                            c.to_ascii_lowercase()
                        } else {
                            c.to_ascii_uppercase()
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("/");
        Position::from_fen(&format!(
            "{board} {} - - {} {}",
            if position.side_to_move() == Color::White {
                "b"
            } else {
                "w"
            },
            fields[4],
            fields[5]
        ))
        .unwrap()
    }

    #[test]
    fn transparent_arithmetic_and_original_positional_preferences() {
        assert_eq!(EVALUATOR_ID, "mshogi.chess.tapered-position.v1");
        assert_eq!(SEARCH_ID, "mshogi.chess.alpha-beta-capture-order.v1");
        // Kd4 has centrality6; Kh8 has0. No non-pawn material => endgame.
        let p = Position::from_fen("7k/8/8/8/3K4/8/8/8 w - - 0 1").unwrap();
        let before = p.clone();
        let e = evaluate(&p);
        assert_eq!(
            (e.middle_phase, e.white_material, e.black_material),
            (0, 0, 0)
        );
        assert_eq!((e.white_positional, e.black_positional), (84, 0));
        assert_eq!(e.score_for(Color::White), 84);
        assert_eq!(e.score_for(Color::Black), -84);
        assert_eq!(p, before);
        // Nb1: centrality1; middle4/end8 weighted at phase320 gives7.
        let corner = Position::from_fen("7k/8/8/8/8/8/8/KN6 w - - 0 1").unwrap();
        let e = evaluate(&corner);
        assert_eq!(
            (e.middle_phase, e.white_material, e.white_positional),
            (320, 320, 7)
        );
        assert_eq!(e.score_for(Color::White), 327);
        // Nc3: centrality4; middle48/end32 weighted at phase320 gives32.
        let developed = Position::from_fen("7k/8/8/8/8/2N5/8/K7 w - - 0 1").unwrap();
        assert_eq!(evaluate(&developed).score_for(Color::White), 352);
        // A2 isolated passed pawn: endgame 0advance -8isolated +10passed.
        let pawn = Position::from_fen("7k/8/8/8/8/8/P7/K7 w - - 0 1").unwrap();
        assert_eq!(evaluate(&pawn).score_for(Color::White), 102);
        // A6: advance4 gives56 -8 +(10+10*16)=218 positional.
        let advanced = Position::from_fen("7k/8/P7/8/8/8/8/K7 w - - 0 1").unwrap();
        assert_eq!(evaluate(&advanced).score_for(Color::White), 318);
        let initial = game(STANDARD_START_FEN, ClaimPolicy::ExplicitOnly);
        assert_eq!(evaluate(initial.position()).middle_phase, MIDDLE_PHASE);
        assert_eq!(evaluate(initial.position()).score_for(Color::White), 0);
        let choice = run(&initial, 1);
        assert_eq!(chosen(&choice).to_string(), "b1c3");
        assert_eq!(choice.score, Some(44)); // N1 -> N3: middle48 - middle4.
    }

    #[test]
    fn evaluation_symmetry_and_no_outcome_or_turn_inference() {
        for fen in [
            "6nk/5p2/8/8/8/8/P1B5/KRQ5 w - - 0 1",
            "7k/8/8/8/3K4/8/8/8 b - - 150 999",
            "7k/8/8/8/8/8/1q6/K7 w - - 0 1",
            "7k/8/P7/8/8/8/8/K7 w - - 0 1",
        ] {
            let p = Position::from_fen(fen).unwrap();
            let other = reflected(&p);
            assert_eq!(reflected(&other), p);
            assert_eq!(
                evaluate(&other).score_for(Color::White),
                -evaluate(&p).score_for(Color::White)
            );
            assert_eq!(
                evaluate(&other).score_for(other.side_to_move()),
                evaluate(&p).score_for(p.side_to_move())
            );
        }
        let a = Position::from_fen("7k/8/8/8/8/8/8/KQ6 w - - 0 1").unwrap();
        let b = Position::from_fen("7k/8/8/8/8/8/8/KQ6 b - - 150 999").unwrap();
        assert_eq!(evaluate(&a), evaluate(&b));
        // A nonzero static king positional value still produces a native draw.
        let dead = game("7k/8/8/8/3K4/8/8/8 w - - 0 1", ClaimPolicy::ExplicitOnly);
        assert_ne!(evaluate(dead.position()).score_for(Color::White), 0);
        assert_eq!(run(&dead, 3).score, Some(0));
    }

    struct Finite {
        at: usize,
        path: Vec<usize>,
        leaves: [i32; 4],
        fail_on: Option<usize>,
    }
    impl Tree for Finite {
        type Choice = usize;
        fn terminal(&self, _: u16) -> Option<i32> {
            None
        }
        fn claim(&self) -> Option<(usize, bool)> {
            None
        }
        fn evaluate(&self) -> i32 {
            match self.at {
                0 => 0,
                1 => -8,
                2 => -2,
                n => self.leaves[n - 3],
            }
        }
        fn actions(&self) -> Vec<usize> {
            match self.at {
                0 => vec![1, 2],
                1 => vec![3, 4],
                2 => vec![5, 6],
                _ => vec![],
            }
        }
        fn push(&mut self, choice: usize) -> Result<(), SearchError> {
            if self.fail_on == Some(choice) {
                return Err(SearchError::Transition(PlayError::Action(
                    ActionError::CounterOverflow,
                )));
            }
            self.path.push(self.at);
            self.at = choice;
            Ok(())
        }
        fn pop(&mut self) {
            self.at = self.path.pop().unwrap();
        }
    }
    fn finite(leaves: [i32; 4]) -> Finite {
        Finite {
            at: 0,
            path: vec![],
            leaves,
            fail_on: None,
        }
    }

    #[test]
    fn independent_cutoff_tree_and_exact_minimax_values() {
        // A=min(7,6)=6; B=min(5,9)=5. Once B's first leaf establishes <=5,
        // its9 leaf cannot beat A. Exact backup chooses A6 with one fewer visit.
        let mut tree = finite([7, 6, 5, 9]);
        let mut stop = || false;
        let mut work = Work {
            nodes: 0,
            limit: None,
            stop: &mut stop,
        };
        let value = negamax(&mut tree, 2, 0, -INFINITY, INFINITY, &mut work)
            .unwrap_or_else(|_| panic!("unexpected halt"));
        assert_eq!((value.score, value.pv), (6, vec![1, 4]));
        assert_eq!(work.nodes, 6);
        assert_eq!(tree.at, 0);
        assert!(tree.path.is_empty());
        for leaves in [
            [7, 3, 5, 9],
            [7, 6, 5, 9],
            [-7, -6, -5, -9],
            [0, -2, -1, 4],
            [3, 3, 3, 3],
        ] {
            // Independent two-level max/min arithmetic, no alpha-beta oracle.
            let a = leaves[0].min(leaves[1]);
            let b = leaves[2].min(leaves[3]);
            let expected = a.max(b);
            let root = if a >= b { 1 } else { 2 };
            let mut tree = finite(leaves);
            let mut stop = || false;
            let mut work = Work {
                nodes: 0,
                limit: None,
                stop: &mut stop,
            };
            let value = negamax(&mut tree, 2, 0, -INFINITY, INFINITY, &mut work)
                .unwrap_or_else(|_| panic!("unexpected halt"));
            assert_eq!(value.score, expected);
            assert_eq!(value.pv[0], root);
            assert!(work.nodes <= 7);
            assert_eq!(tree.at, 0);
            assert!(tree.path.is_empty());
        }
    }

    #[test]
    fn cutoff_interruption_and_errors_restore_every_private_transition() {
        for budget in 0..6 {
            let mut tree = finite([7, 6, 5, 9]);
            let mut stop = || false;
            let mut work = Work {
                nodes: 0,
                limit: Some(budget),
                stop: &mut stop,
            };
            assert!(matches!(
                negamax(&mut tree, 2, 0, -INFINITY, INFINITY, &mut work),
                Err(Halt::Limit(SearchStatus::NodeLimit))
            ));
            assert_eq!(work.nodes, budget);
            assert_eq!(tree.at, 0);
            assert!(tree.path.is_empty());
            let mut tree = finite([7, 6, 5, 9]);
            let mut polls = 0;
            let mut stop = || {
                polls += 1;
                polls > budget
            };
            let mut work = Work {
                nodes: 0,
                limit: None,
                stop: &mut stop,
            };
            assert!(matches!(
                negamax(&mut tree, 2, 0, -INFINITY, INFINITY, &mut work),
                Err(Halt::Limit(SearchStatus::Stopped))
            ));
            assert_eq!(work.nodes, budget);
            assert_eq!(tree.at, 0);
            assert!(tree.path.is_empty());
        }
        let mut tree = finite([7, 6, 5, 9]);
        tree.fail_on = Some(4);
        let mut stop = || false;
        let mut work = Work {
            nodes: 0,
            limit: None,
            stop: &mut stop,
        };
        assert!(matches!(
            negamax(&mut tree, 2, 0, -INFINITY, INFINITY, &mut work),
            Err(Halt::Error(SearchError::Transition(_)))
        ));
        assert_eq!(tree.at, 0);
        assert!(tree.path.is_empty());
    }

    #[test]
    fn material_only_alpha_beta_agrees_with_frozen_full_width_separately_from_position_terms() {
        for (fen, policy, depth) in [
            (STANDARD_START_FEN, ClaimPolicy::ExplicitOnly, 2),
            (
                "7k/8/4QK2/8/8/8/8/8 w - - 0 1",
                ClaimPolicy::ExplicitOnly,
                3,
            ),
            (
                "7k/8/8/8/8/8/q7/R6K w - - 0 1",
                ClaimPolicy::ExplicitOnly,
                2,
            ),
            ("7k/8/8/8/8/8/8/K7 w - - 0 1", ClaimPolicy::ExplicitOnly, 3),
            ("7k/8/8/8/8/8/8/KR6 w - - 98 1", ClaimPolicy::AutoClaim, 2),
            (
                "7k/8/8/8/8/8/8/KR6 b - - 99 1",
                ClaimPolicy::ExplicitOnly,
                2,
            ),
        ] {
            let g = game(fen, policy);
            let before = g.clone();
            let limits = SearchLimits { depth, nodes: None };
            let baseline = crate::search::search(&g, limits).unwrap();
            let ab = search_with_evaluator(&g, limits, &mut || false, material_score).unwrap();
            assert_eq!(
                (ab.score, ab.completed_depth, ab.status),
                (baseline.score, baseline.completed_depth, baseline.status),
                "{fen}"
            );
            if let Some(SearchDecision::Finished(expected)) = baseline.decision {
                assert_eq!(ab.decision, Some(SearchDecision::Finished(expected)));
            }
            if fen == "7k/8/8/8/8/8/q7/R6K w - - 0 1" {
                assert_eq!(
                    chosen(&ab).to_string(),
                    "a1a2",
                    "unique unprotected queen capture"
                );
                assert_eq!(ab.decision, baseline.decision);
            }
            let root_side = g.position().side_to_move();
            let mut leaf = g.clone();
            let mut plies = 0;
            for (i, choice) in ab.principal_variation.iter().enumerate() {
                match choice {
                    SearchChoice::Move(action) => {
                        assert!(leaf.position().legal_actions().contains(action));
                        leaf.apply(*action).unwrap();
                        plies += 1;
                    }
                    SearchChoice::Claim(claim) => {
                        assert_eq!(i + 1, ab.principal_variation.len());
                        leaf.claim(*claim).unwrap();
                    }
                }
            }
            let expected = match leaf.outcome() {
                Some(ChessOutcome::Draw(_)) => 0,
                Some(ChessOutcome::Checkmate { winner }) => {
                    if winner == root_side {
                        MATE_SCORE - plies
                    } else {
                        -MATE_SCORE + plies
                    }
                }
                None => crate::chess::evaluate(leaf.position()).score_for(root_side),
            };
            assert_eq!(ab.score, Some(expected), "exact published PV score: {fen}");
            assert!(ab.nodes <= baseline.nodes);
            assert_eq!(g, before);
        }
    }

    #[test]
    fn native_mates_terminal_precedence_claims_and_leaf_claim_tail() {
        for (fen, expected) in [
            ("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", "g6g7"),
            ("8/8/8/8/8/5kq1/8/7K b - - 0 1", "g3g2"),
        ] {
            let g = game(fen, ClaimPolicy::ExplicitOnly);
            let result = run(&g, 1);
            assert_eq!(chosen(&result).to_string(), expected);
            assert_eq!(result.score, Some(MATE_SCORE - 1));
            let mut mated = g.clone();
            mated.apply(chosen(&result)).unwrap();
            let finished = run(&mated, 4);
            assert_eq!(finished.score, Some(-MATE_SCORE));
            assert_eq!(finished.nodes, 1);
            assert!(matches!(
                finished.decision,
                Some(SearchDecision::Finished(ChessOutcome::Checkmate { .. }))
            ));
        }
        let two = game("7k/8/4QK2/8/8/8/8/8 w - - 0 1", ClaimPolicy::ExplicitOnly);
        assert_eq!(run(&two, 3).score, Some(MATE_SCORE - 3));
        for (fen, reason) in [
            ("7k/5K2/6Q1/8/8/8/8/8 b - - 0 1", DrawReason::Stalemate),
            (
                "7k/8/8/8/8/8/8/KR6 w - - 150 1",
                DrawReason::SeventyFiveMove,
            ),
        ] {
            assert_eq!(
                run(&game(fen, ClaimPolicy::ExplicitOnly), 2).decision,
                Some(SearchDecision::Finished(ChessOutcome::Draw(reason)))
            );
        }
        let fen = "7k/8/5KQ1/8/8/8/8/8 w - - 100 1";
        assert_eq!(
            run(&game(fen, ClaimPolicy::ExplicitOnly), 1).score,
            Some(MATE_SCORE - 1)
        );
        let forced = run(&game(fen, ClaimPolicy::AutoClaim), 1);
        assert_eq!(forced.score, Some(0));
        assert!(matches!(
            forced.decision,
            Some(SearchDecision::Choose(SearchChoice::Claim(_)))
        ));
        let leaf = run(
            &game("7k/8/8/8/8/8/8/KR6 w - - 98 1", ClaimPolicy::AutoClaim),
            1,
        );
        assert_eq!(leaf.score, Some(0));
        assert_eq!(leaf.principal_variation.len(), 2);
        assert!(matches!(
            leaf.principal_variation[1],
            SearchChoice::Claim(_)
        ));
        let losing = run(
            &game("7k/8/8/8/8/8/8/KR6 b - - 99 1", ClaimPolicy::ExplicitOnly),
            1,
        );
        assert_eq!(losing.score, Some(0));
        assert!(matches!(
            losing.decision,
            Some(SearchDecision::Choose(SearchChoice::Claim(_)))
        ));
    }

    #[test]
    fn same_board_different_repetition_history_stays_distinct() {
        let mut repeated = game("6nk/8/8/8/8/8/8/KR4N1 b - - 0 1", ClaimPolicy::ExplicitOnly);
        for _ in 0..2 {
            for text in ["g8f6", "g1f3", "f6g8", "f3g1"] {
                repeated.apply(text.parse().unwrap()).unwrap();
            }
        }
        let imported = game(&repeated.position().to_fen(), ClaimPolicy::ExplicitOnly);
        assert_eq!(repeated.position(), imported.position());
        let a = run(&repeated, 1);
        let b = run(&imported, 1);
        assert_eq!(a.score, Some(0));
        assert!(matches!(
            a.decision,
            Some(SearchDecision::Choose(SearchChoice::Claim(_)))
        ));
        assert!(b.score.unwrap() < 0);
        assert!(matches!(
            b.decision,
            Some(SearchDecision::Choose(SearchChoice::Move(_)))
        ));
    }

    #[test]
    fn ordered_native_captures_unwind_on_limits_and_retain_completed_iteration() {
        let g = game(
            "7k/8/8/8/1rN5/2P5/1q6/1R5K w - - 0 1",
            ClaimPolicy::ExplicitOnly,
        );
        let original = g.clone();
        let mut tree = ChessTree {
            game: g.clone(),
            evaluator: candidate_score,
        };
        let mut stop = || false;
        let mut work = Work {
            nodes: 0,
            limit: Some(2),
            stop: &mut stop,
        };
        assert!(matches!(
            negamax(&mut tree, 2, 0, -INFINITY, INFINITY, &mut work),
            Err(Halt::Limit(SearchStatus::NodeLimit))
        ));
        assert_eq!(work.nodes, 2);
        assert_eq!(tree.game, original);
        let first = run(&g, 1);
        let budget = first.nodes + 2;
        let result = search(
            &g,
            SearchLimits {
                depth: 2,
                nodes: Some(budget),
            },
        )
        .unwrap();
        assert_eq!(result.completed_depth, 1);
        assert_eq!(result.status, SearchStatus::NodeLimit);
        assert_eq!(result.nodes, budget);
        assert_eq!(
            (result.decision, result.score, result.principal_variation),
            (first.decision, first.score, first.principal_variation)
        );
        assert_eq!(g, original);
    }

    #[test]
    fn node_stop_partial_iteration_and_counter_failures_are_honest() {
        let g = game(STANDARD_START_FEN, ClaimPolicy::ExplicitOnly);
        let before = g.clone();
        let first = run(&g, 1);
        assert_eq!(first.nodes, 21);
        for nodes in [0, 1, 20] {
            let r = search(
                &g,
                SearchLimits {
                    depth: 2,
                    nodes: Some(nodes),
                },
            )
            .unwrap();
            assert_eq!(r.nodes, nodes);
            assert_eq!(r.status, SearchStatus::NodeLimit);
            assert_eq!(r.completed_depth, 0);
            assert!(r.decision.is_none() && r.score.is_none() && r.principal_variation.is_empty());
        }
        let limits = SearchLimits {
            depth: 2,
            nodes: Some(25),
        };
        let r = search(&g, limits).unwrap();
        assert_eq!(r.completed_depth, 1);
        assert_eq!(r.nodes, 25);
        assert_eq!(r.status, SearchStatus::NodeLimit);
        assert_eq!(
            (r.decision, r.score, &r.principal_variation),
            (first.decision, first.score, &first.principal_variation)
        );
        assert_eq!(search(&g, limits).unwrap(), r);
        let mut polls = 0;
        let stopped = search_with_stop(
            &g,
            SearchLimits {
                depth: 2,
                nodes: None,
            },
            || {
                polls += 1;
                polls > 25
            },
        )
        .unwrap();
        assert_eq!(stopped.status, SearchStatus::Stopped);
        assert_eq!(stopped.nodes, 25);
        assert_eq!(stopped.completed_depth, 1);
        assert_eq!(
            (stopped.decision, stopped.score, stopped.principal_variation),
            (first.decision, first.score, first.principal_variation)
        );
        assert_eq!(g, before);
        for depth in [0, MAX_DEPTH + 1] {
            assert_eq!(
                search(&g, SearchLimits { depth, nodes: None }),
                Err(SearchError::InvalidDepth(depth))
            );
        }
        let overflow = game(
            "7k/8/8/8/8/8/8/KR6 b - - 0 4294967295",
            ClaimPolicy::ExplicitOnly,
        );
        let before = overflow.clone();
        assert_eq!(
            search(
                &overflow,
                SearchLimits {
                    depth: 1,
                    nodes: None
                }
            ),
            Err(SearchError::Transition(PlayError::Action(
                ActionError::CounterOverflow
            )))
        );
        assert_eq!(overflow, before);
    }
}
