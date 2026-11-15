//! Shared classification of saved engine observations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SearchKind {
    SharedCore,
    FullWidth,
}

#[derive(Clone, Copy)]
pub(crate) struct Profile {
    pub evaluator: &'static str,
    pub search: &'static str,
    pub max_depth: u16,
    pub max_nodes: u64,
    pub win_score: i32,
    pub line_bound: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct Identity {
    pub kind: SearchKind,
    pub profile: Profile,
}
impl Identity {
    pub fn classify(
        evaluator: &str,
        search: &str,
        current: Profile,
        legacy: Profile,
    ) -> Option<Self> {
        [
            (SearchKind::SharedCore, current),
            (SearchKind::FullWidth, legacy),
        ]
        .into_iter()
        .find(|(_, p)| evaluator == p.evaluator && search == p.search)
        .map(|(kind, profile)| Self { kind, profile })
    }
    pub fn max_depth(self) -> u16 {
        self.profile.max_depth
    }
    pub fn max_nodes(self) -> u64 {
        self.profile.max_nodes
    }
    pub fn max_line(self, completed_depth: u16) -> usize {
        match self.kind {
            SearchKind::SharedCore => self.profile.line_bound,
            SearchKind::FullWidth => usize::from(completed_depth) + self.profile.line_bound,
        }
    }
    pub fn least_nodes(self, completed_depth: u16) -> u64 {
        match self.kind {
            SearchKind::SharedCore => u64::from(completed_depth),
            SearchKind::FullWidth => 2 * u64::from(completed_depth),
        }
    }
}
