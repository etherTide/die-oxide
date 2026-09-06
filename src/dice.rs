use std::{
    fmt::{Debug, Display},
    str::FromStr,
    sync::Arc,
};

use color_eyre::eyre::Report;
use rand::random_range;

#[derive(Clone, Copy, Debug)]
pub struct Die {
    faces: u8,
}
impl Die {
    #[must_use]
    pub const fn new(faces: u8) -> Self {
        Self { faces }
    }
    #[must_use]
    pub fn roll(&self) -> u8 {
        random_range(1..=self.faces)
    }
}
impl Default for Die {
    fn default() -> Self {
        Self::new(6)
    }
}
impl Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "d{}", self.faces)
    }
}
impl FromStr for Die {
    type Err = Report;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('d') => (),
            _ => {
                return Err(Report::msg("Die string must start with `d`, eg `d20`"));
            }
        };
        s.get(1..)
            .expect("s[0] was 'd', so s[1] should have been a char boundary?")
            .parse()
    }
}

#[derive(Debug)]
pub struct DiceTray {
    pub die: Die,
    pub count: u8,
    pub rolls: Arc<[u8]>,
    pub result: Option<u32>,
}
impl DiceTray {
    #[must_use]
    pub fn new(die: Die, count: u8) -> Self {
        let rolls: Arc<[u8]> = (0..count).into_iter().map(|_| die.roll()).collect();
        let result = Self::sum_rolls(&rolls);
        Self {
            die,
            count,
            rolls,
            result,
        }
    }

    #[must_use]
    fn sum_rolls(rolls: &[u8]) -> Option<u32> {
        rolls
            .iter()
            .fold(Some(0u32), |acc, &elem| acc?.checked_add(elem.into()))
    }
}
impl Display for DiceTray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.result {
            Some(result) => {
                write!(f, "{}: {} {:?}", self.die, result, self.rolls)
            }
            None => {
                write!(f, "{}: OVERFLOW! {:?}", self.die, self.rolls)
            }
        }
    }
}
