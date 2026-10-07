use std::fmt::Display;

use rand::random_range;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub struct Die(pub u8);
impl Die {
    #[must_use]
    pub fn roll(self) -> DieRoll {
        let result = random_range(1..self.into());
        DieRoll(self, result)
    }
}
impl From<Die> for u8 {
    fn from(value: Die) -> Self {
        value.0
    }
}
impl Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "d{}", u8::from(*self))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
#[non_exhaustive]
pub struct DieRoll(pub Die, pub u8);
impl DieRoll {
    #[must_use]
    pub fn distance_from_max(&self) -> u8 {
        u8::from(self.0).strict_sub(self.1)
    }
}
impl From<DieRoll> for u8 {
    fn from(value: DieRoll) -> Self {
        value.1
    }
}

#[derive(Debug, Clone)]
pub struct DiceTower {
    pub die: Die,
    pub count: u8,
}
impl DiceTower {
    #[must_use]
    pub fn roll(self) -> DiceTray {
        let results = (0..self.count).map(|x| Die(x).roll()).collect();
        DiceTray {
            die: self.die,
            results,
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct DiceTray {
    pub die: Die,
    pub results: Vec<DieRoll>,
}
impl DiceTray {
    pub fn sum(&self) -> Option<u32> {
        self.results
            .iter()
            .try_fold(0, |acc, elem| u32::from(u8::from(*elem)).checked_add(acc))
    }
}
