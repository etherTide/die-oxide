use rand::random_range;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Die(pub u8);
impl Die {
    #[must_use]
    pub fn roll(&self) -> u8 {
        random_range(1..=self.0)
    }
}

pub const STANDARD_SET: [Die; 7] = [Die(4), Die(6), Die(8), Die(10), Die(12), Die(20), Die(100)];
