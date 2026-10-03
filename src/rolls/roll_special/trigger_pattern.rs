use std::ops::Range;

pub enum TriggerPattern {
    Die(DiePattern),
    DiceTray(TrayPattern),
}

pub enum DiePattern {
    LessThan(u8),
    Exactly(u8),
    GreaterThan(u8),
    InRange(Range<u8>),
    InUpperBound(u8),
}

pub enum TrayPattern {
    MatchAtLeast(u8, DiePattern),
    MatchAtMost(u8, DiePattern),
    MatchExactly(u8, DiePattern),
    MatchRange(Range<u8>, DiePattern),
    MatchAll(DiePattern),
    Rank(u8),
    RankRange(Range<u8>),
    Percentile(u8),
    PercentileRange(Range<u8>),
}
