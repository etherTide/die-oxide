#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollTrigger {
    DieTrigger,
    TrayTrigger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Trigger matches on each `Die` in the roll
pub enum DieTrigger {
    ScoredMin,
    /// E.g. for "Treat rolls less than 10 as if you rolled a 10"
    ScoredLessThan(u8),
    ScoredExactly(u8),
    ScoredGreaterThan(u8),
    /// Like `ScoredGreaterThan` but grows down from max, not up from min
    ScoredWithinRangeOfMax(u8),
    ScoredMax,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Trigger matches against the entire rolled `DiceTray`
pub enum TrayTrigger {
    /// E.g. Lower quartile. Useful for "4d6 drop lowest 1"
    Rank(RankTrigger),
    NthMatch(usize, DieTrigger),
    RevNthMatch(usize, DieTrigger),
    MatchRange((usize, usize), DieTrigger),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankTrigger {
    Lower(u8),
    Median(u8),
    Upper(u8),
}
