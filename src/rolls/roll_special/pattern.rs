use crate::dice::die::Die;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Die(DiePattern),
    DiceTray(TrayPattern),
}
impl Pattern {
    // pub fn get_matches<'a>(&self, dice_tray: &'a DiceTray) -> Matches<'a> {
    //     let die = dice_tray.die;
    //     let results = dice_tray.results.clone();
    //     let mut matches = Vec::new();
    //     match self {
    //         Self::Die(p) => {
    //             let temp_vec = &mut results
    //                 .iter()
    //                 .enumerate()
    //                 .filter_map(|(idx, result)| {
    //                     if p.die_is_match(die, *result) {
    //                         Some(idx)
    //                     } else {
    //                         None
    //                     }
    //                 })
    //                 .collect();
    //             matches.append(temp_vec);
    //         }
    //         Self::DiceTray(_p) => todo!(),
    //     }
    //     Matches::new(&dice_tray, matches)
    // }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiePattern {
    LessThan(u8),
    Exactly(u8),
    GreaterThan(u8),
    InRange(Range<u8>),
    InUpperBound(u8),
}
impl DiePattern {
    pub fn die_is_match(&self, die: Die, result: u8) -> bool {
        let die_max = die.0;
        match self {
            Self::LessThan(p) => result.lt(p),
            Self::Exactly(p) => result.eq(p),
            Self::GreaterThan(p) => result.gt(p),
            Self::InRange(p) => p.contains(&result),
            Self::InUpperBound(p) => result.gt(&die_max.saturating_sub(*p)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
