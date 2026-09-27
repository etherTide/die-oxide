#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollTrigger {
    Die(DieTriggerCondition),
    DiceTray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DieTriggerCondition {
    ScoredLessThan(u8),
    ScoredExactly(u8),
    ScoredGreaterThan(u8),
    ScoredWithinRangeOfMax(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DieTrayTriggerCondition {
    EachMatch(DieTriggerCondition),
    NthMatch(usize, DieTriggerCondition),
    RevNthMatch(usize, DieTriggerCondition),
    Rank(RankTriggerCondition),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RankTriggerCondition {
    Lower(u8),
    Median(u8),
    Upper(u8),
}
