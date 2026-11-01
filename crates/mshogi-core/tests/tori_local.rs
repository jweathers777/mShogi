//! External-consumer tests of the sole public checked local boundary.
use mshogi_core::tori::{Action, Face, LocalPosition, ReversalError};
fn a(from: &str, to: &str) -> Action {
    Action::Move {
        from: from.parse().unwrap(),
        to: to.parse().unwrap(),
        result_face: Face::Swallow,
    }
}
#[test]
fn public_local_tokens_reject_wrong_instance_order_use_and_retired_branch() {
    let mut p = LocalPosition::standard();
    let initial = p.position().state().clone();
    let (_, mut ua) = p.apply(a("1e", "1d")).unwrap();
    let (_, mut ub) = p.apply(a("7c", "7d")).unwrap();
    let applied = p.position().state().clone();
    let mut copy = p.clone();
    assert_eq!(copy.undo(&mut ub).unwrap_err(), ReversalError::InvalidToken);
    assert_eq!(copy.position().state(), &applied);
    assert_eq!(p.undo(&mut ua).unwrap_err(), ReversalError::InvalidToken);
    assert_eq!(p.position().state(), &applied);
    let mut rb = p.undo(&mut ub).unwrap();
    let mut ra = p.undo(&mut ua).unwrap();
    assert_eq!(p.position().state(), &initial);
    assert_eq!(p.undo(&mut ua).unwrap_err(), ReversalError::InvalidToken);
    assert_eq!(p.redo(&mut rb).unwrap_err(), ReversalError::InvalidToken);
    let (_, mut fresh_a) = p.redo(&mut ra).unwrap();
    let (_, mut fresh_b) = p.redo(&mut rb).unwrap();
    assert_eq!(p.position().state(), &applied);
    assert_eq!(p.undo(&mut ub).unwrap_err(), ReversalError::InvalidToken);
    assert_eq!(p.redo(&mut rb).unwrap_err(), ReversalError::InvalidToken);
    let mut retired_b = p.undo(&mut fresh_b).unwrap();
    let mut retired_a = p.undo(&mut fresh_a).unwrap();
    assert_eq!(p.position().state(), &initial);
    let (_, mut branch) = p.apply(a("1e", "1d")).unwrap();
    p.undo(&mut branch).unwrap();
    assert_eq!(p.position().state(), &initial);
    assert_eq!(
        p.redo(&mut retired_a).unwrap_err(),
        ReversalError::InvalidToken
    );
    assert_eq!(
        p.redo(&mut retired_b).unwrap_err(),
        ReversalError::InvalidToken
    );
    assert_eq!(p.position().state(), &initial);
}
