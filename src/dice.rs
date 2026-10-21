//! TODO: Write module documentation

use color_eyre::Report;

use std::num::NonZeroU8;
/// A representation of a standard game die.
///
/// # Invariant
/// Always has at least 1 face
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[non_exhaustive]
pub struct Die(pub u8);
impl Die {
    /// Creates a new instance of a fair [`Die`] (i.e. with rolls that roughly follow a
    /// [uniform][rand::distr::Uniform] distribution)
    #[must_use]
    pub const fn new(number_of_sides: NonZeroU8) -> Self {
        Self(number_of_sides.get())
    }
}
impl Default for Die {
    fn default() -> Self {
        Self(6)
    }
}
impl TryFrom<u8> for Die {
    type Error = Report;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let non_zero: NonZeroU8 = value
            .try_into()
            .map_err(|e| Report::from(e).wrap_err("Die must have at least 1 face, found 0!"))?;
        Ok(Self::new(non_zero))
    }
}
impl From<Die> for u8 {
    fn from(value: Die) -> Self {
        value.0
    }
}
impl AsRef<u8> for Die {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

use rand::{RngExt, rngs::SmallRng};
/// For generating random [`Die`] rolls.
/// Generic over a random number generator `R`, by default [`SmallRng`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiceTower<R: rand::Rng = SmallRng> {
    pub die: Die,
    rng: R,
}
impl DiceTower {
    pub fn roll(&mut self) -> u8 {
        self.rng.random_range(1..=self.die.0)
    }
}
