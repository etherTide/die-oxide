//! TODO: Write module documentation

use color_eyre::Report;
use rand::{RngExt, rngs::SmallRng};
use std::num::NonZeroU8;

/// A representation of a standard game die.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[non_exhaustive]
pub struct Die(pub NonZeroU8);
impl Die {
    #[must_use]
    pub const fn new(number_of_sides: NonZeroU8) -> Self {
        //! Creates a new instance of a fair [`Die`] (i.e. with rolls that roughly follow a
        //! [uniform][rand::distr::Uniform] distribution).
        Self(number_of_sides)
    }
}
impl Default for Die {
    fn default() -> Self {
        //! Returns a [`Die`] with 6 faces.
        //!
        //! # Example
        //! ```
        //! use die_oxide::dice::Die;
        //! assert_eq!(<Die as Default>::default(), <Die as TryFrom<u8>>::try_from(6_u8).unwrap());
        //! ```
        const DEFAULT: u8 = 6_u8;
        #[allow(
            clippy::expect_used,
            reason = "Should panic if invariant cannot be established due to breaking dependancy change or logic bug"
        )]
        Self(
            NonZeroU8::new(DEFAULT)
                .expect("Tried to create NonZeroU8 from {DEFAULT}, expected Some, got None"),
        )
    }
}

impl TryFrom<u8> for Die {
    type Error = Report;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        //! Fails if the conversion from `value` into [`NonZeroU8`] fails.
        //!
        //! # Examples
        //! ```
        //! use die_oxide::dice::Die;
        //! use std::assert_matches;
        //!
        //! assert!(Die::try_from(0).is_err());
        //! assert_matches!(Die::try_from(1), Ok(_))
        //! ```
        let non_zero: NonZeroU8 = value
            .try_into()
            .map_err(|e| Report::from(e).wrap_err("Die must have at least 1 face, found 0!"))?;
        Ok(Self::new(non_zero))
    }
}
impl From<Die> for u8 {
    fn from(value: Die) -> Self {
        value.0.get()
    }
}
impl AsRef<NonZeroU8> for Die {
    fn as_ref(&self) -> &NonZeroU8 {
        &self.0
    }
}

/// For generating random [`Die`] rolls.
/// Generic over a random number generator `R`, by default [`SmallRng`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiceTower<R: rand::Rng = SmallRng> {
    pub die: Die,
    rng: R,
}
impl DiceTower {
    #[must_use]
    pub fn new(die: Die) -> Self {
        //! Returns a new [`DiceTower`] with the given [`Die`] and the default [`Rng`][`rand::Rng`].
        Self {
            die,
            rng: rand::make_rng(),
        }
    }
    #[must_use]
    pub fn roll(&mut self) -> u8 {
        //! Returns a uniformly random [`u8`] in range `1..=die_size`
        self.rng.random_range(1..=self.die.0.into())
    }
    #[must_use]
    pub fn roll_multiple<const N: usize>(&mut self) -> [u8; N] {
        //! Calls [`self.roll()`][`Self::roll`] in series to fill an array of size `N`.
        //! Is not parallelised as [`rng`][`Self::rng`] is [`Rng`][`rand::Rng`], which isn't
        //! thread-safe.
        let arr = [0; N];
        arr.map(|_| self.roll())
    }
}

/// A container for storing [`Die`] roll outputs from [`DiceTower`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiceTray {
    pub die: Die,
    pub tray: Vec<u8>,
}
impl DiceTray {
    // #[must_use]
    // pub fn new() {}
}
