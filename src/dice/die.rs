use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use color_eyre::eyre::{OptionExt, Report};
use rand::random_range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Die(pub u8);
impl Die {
    #[must_use]
    pub fn roll(&self) -> u8 {
        random_range(1..=self.0)
    }
}
impl Default for Die {
    fn default() -> Self {
        Self(6)
    }
}
impl Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "d{}", self.0)
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
        }
        s.get(1..)
            .ok_or_eyre("s[0] was 'd', so s[1] should have been a char boundary?")?
            .parse()
    }
}
