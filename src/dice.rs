use std::fmt::Display;

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
impl Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "d{}", self.faces)
    }
}
impl Default for Die {
    fn default() -> Self {
        Self::new(6)
    }
}
#[derive(Debug)]
pub struct DiceTray {
    pub die: Die,
    pub results: Vec<u8>,
}
impl DiceTray {
    #[must_use]
    pub fn new(die: Die, count: u8) -> Self {
        let results: Vec<u8> = (0..count).into_iter().map(|_| die.roll()).collect();
        Self { die, results }
    }
}
