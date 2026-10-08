use crate::dice::DieRoll;
use std::{ops::Bound, range::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiePattern {
    MatchesComp(Comparrison, DieRoll),
    InRange(Range<DieRoll>),
    InUpperBound(Bound<DieRoll>),
    InRangeOfUpperBound(Bound<DieRoll>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparrison {
    Less,
    /// Equivalent to >=
    NotLess,
    Equal,
    NotEqual,
    Greater,
    /// Equivalent to <=
    NotGreater,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiceTrayPattern {
    MatchContents(Quantity, DiePattern),
    Percentile(Quantity, Percentile),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantity {
    Comparrison(Comparrison, u8),
    Range(Range<u8>),
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub struct Percentile(u8);
impl Percentile {
    #[must_use]
    pub const fn new(percentile: u8) -> Option<Self> {
        // TODO: Should this be < or <= 100? Not 100% but 100th percentile, so I think < ...
        if percentile < 100 {
            Some(Self(percentile))
        } else {
            None
        }
    }
}
