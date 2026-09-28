use crate::dice::{Die, die_result::DieResult};

use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct DiceTray {
    pub die: Die,
    pub count: u8,
    pub rolls: Rc<[DieResult]>,
    pub sum: Option<u32>,
}
impl DiceTray {
    #[must_use]
    pub fn new(die: Die, count: u8) -> Self {
        let rolls: Rc<[DieResult]> = Self::get_rolls(die, count);
        let sum = Self::get_sum(&rolls);
        Self {
            die,
            count,
            rolls,
            sum,
        }
    }

    #[must_use]
    fn get_sum(rolls: &[DieResult]) -> Option<u32> {
        rolls
            .iter()
            .try_fold(0u32, |acc, &elem| acc.checked_add(elem.value.into()))
    }
    #[must_use]
    fn get_rolls(die: Die, count: u8) -> Rc<[DieResult]> {
        let results = Vec::<DieResult>::with_capacity(count.into());
    }
}
impl Display for DiceTray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result: String = self
            .sum
            .map_or_else(|| "OVERFLOW!".to_string(), |val| val.to_string());
        write!(f, "{}: {} {:?}", self.die, result, self.rolls)
    }
}
