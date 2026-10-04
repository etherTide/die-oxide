use crate::dice::{dice_tray::DiceTray, die::Die};
use std::{ops::Range, result};

pub struct PatternMatches<'a> {
    dice_tray: &'a DiceTray,
    matches: Vec<usize>,
}
impl<'a> PatternMatches<'a> {
    pub fn new(dice_tray: &'a DiceTray, matches: Vec<usize>) -> Self {
        Self { dice_tray, matches }
    }
}

pub enum Pattern {
    Die(DiePattern),
    DiceTray(TrayPattern),
}
impl Pattern {
    pub fn get_matches<'a>(&self, dice_tray: &'a DiceTray) -> PatternMatches<'a> {
        let die = dice_tray.die;
        let results = dice_tray.results.clone();
        let mut matches = Vec::new();
        match self {
            Self::Die(p) => {
                let temp_vec = &mut results
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, result)| {
                        if p.die_is_match(die, *result) {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect();
                matches.append(temp_vec);
            }
            Self::DiceTray(_p) => (),
        }
        PatternMatches::new(&dice_tray, matches)
    }
}

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
