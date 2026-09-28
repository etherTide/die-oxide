use crate::{dice::Die, roll::roll_flags::RollFlags};
use rand::random_range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DieResult {
    pub value: u8,
    pub dropped: bool,
    pub exploded: bool,
    pub substituted: bool,
    pub re_rolled: bool,
}
impl DieResult {
    fn new(die: Die, flags: RollFlags) -> Self {
        let value = random_range(1..=die.0);
        for flag in flags {}
        Self {}
    }
}
