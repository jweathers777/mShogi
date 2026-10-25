//! Invented 16×16 strider rules shared by action-boundary regression tests and
//! the separately gated desktop development fixture. This is not a real game.
pub const SIZE: u8 = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sq(pub u8);

impl Sq {
    pub fn new(file: u8, rank: u8) -> Option<Sq> {
        if file >= SIZE || rank >= SIZE {
            return None;
        }
        Some(Sq(rank * SIZE + file))
    }
    fn file(self) -> i16 {
        i16::from(self.0 % SIZE)
    }
    fn rank(self) -> i16 {
        i16::from(self.0 / SIZE)
    }
    fn adjacent(self, other: Sq) -> bool {
        self != other
            && (self.file() - other.file()).abs() <= 1
            && (self.rank() - other.rank()).abs() <= 1
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    Strider,
    Stone,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Piece {
    pub side: u8,
    pub kind: Kind,
}

/// A complete player choice: origin plus the ordered route of squares it
/// visits. Two actions with equal endpoints but different routes differ.
/// There is deliberately no field for client-supplied effects.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompoundAction {
    pub from: Sq,
    pub route: Vec<Sq>,
}

/// Ordered rule-derived effects, reported but never accepted as input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Effect {
    Step { to: Sq },
    Capture { at: Sq, piece: Piece },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rejection {
    NotOwnStrider,
    BadRoute,
    OwnPieceInRoute,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    pub cells: Vec<Option<Piece>>,
    pub side: u8,
}

/// Full-snapshot undo, as in production `Undo`.
pub struct Undo {
    before: Board,
    after: Board,
}

impl Board {
    pub fn empty() -> Board {
        Board {
            cells: vec![None; usize::from(SIZE) * usize::from(SIZE)],
            side: 0,
        }
    }

    pub fn put(&mut self, at: Sq, piece: Piece) {
        self.cells[usize::from(at.0)] = Some(piece);
    }

    pub fn at(&self, at: Sq) -> Option<Piece> {
        self.cells[usize::from(at.0)]
    }

    /// Derive the ordered effects of a complete action from the board.
    fn resolve(&self, action: &CompoundAction) -> Result<Vec<Effect>, Rejection> {
        let mover = self.at(action.from).ok_or(Rejection::NotOwnStrider)?;
        if mover.side != self.side || mover.kind != Kind::Strider {
            return Err(Rejection::NotOwnStrider);
        }
        if action.route.is_empty() || action.route.len() > 2 {
            return Err(Rejection::BadRoute);
        }
        let mut effects = Vec::new();
        let mut current = action.from;
        for &next in &action.route {
            if !current.adjacent(next) {
                return Err(Rejection::BadRoute);
            }
            // The origin is vacated once the strider leaves it.
            let occupant = if next == action.from {
                None
            } else {
                self.at(next)
            };
            match occupant {
                Some(piece) if piece.side == mover.side => {
                    return Err(Rejection::OwnPieceInRoute);
                }
                Some(piece) => effects.push(Effect::Capture { at: next, piece }),
                None => {}
            }
            effects.push(Effect::Step { to: next });
            current = next;
        }
        Ok(effects)
    }

    /// Checked boundary: resolve, apply to a candidate copy, publish.
    pub fn apply(&mut self, action: &CompoundAction) -> Result<(Vec<Effect>, Undo), Rejection> {
        let effects = self.resolve(action)?;
        let mut candidate = self.clone();
        let mover = candidate.cells[usize::from(action.from.0)].take().unwrap();
        let mut at = action.from;
        for effect in &effects {
            match *effect {
                Effect::Capture { at: square, .. } => candidate.cells[usize::from(square.0)] = None,
                Effect::Step { to } => at = to,
            }
        }
        candidate.put(at, mover);
        candidate.side ^= 1;
        let undo = Undo {
            before: self.clone(),
            after: candidate.clone(),
        };
        *self = candidate;
        Ok((effects, undo))
    }

    pub fn undo(&mut self, undo: &Undo) -> Result<(), ()> {
        if *self != undo.after {
            return Err(());
        }
        *self = undo.before.clone();
        Ok(())
    }
}

/// Partial UI selection: accumulates squares without touching the board.
#[derive(Default)]
pub struct Selection {
    pub from: Option<Sq>,
    pub route: Vec<Sq>,
}

impl Selection {
    pub fn commit(self) -> Option<CompoundAction> {
        Some(CompoundAction {
            from: self.from?,
            route: self.route,
        })
    }
}

/// Serialized action identity: origin and route only, never effects.
pub fn encode(action: &CompoundAction) -> String {
    let mut text = action.from.0.to_string();
    for step in &action.route {
        text.push('>');
        text.push_str(&step.0.to_string());
    }
    text
}

pub fn decode(text: &str) -> Option<CompoundAction> {
    let mut parts = text.split('>').map(|part| part.parse::<u8>().ok().map(Sq));
    let from = parts.next()??;
    let route: Option<Vec<Sq>> = parts.collect();
    Some(CompoundAction {
        from,
        route: route?,
    })
}
