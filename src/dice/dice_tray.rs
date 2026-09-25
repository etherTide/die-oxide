use super::Die;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct DiceTray {
    pub die: Die,
    pub count: u8,
    pub rolls: Rc<[u8]>,
    pub sum: Option<u32>,
}
impl DiceTray {
    #[must_use]
    pub fn new(die: Die, count: u8) -> Self {
        let rolls: Rc<[u8]> = (0..count).into_iter().map(|_| die.roll()).collect();
        let sum = Self::sum_rolls(&rolls);
        Self {
            die,
            count,
            rolls,
            sum,
        }
    }

    #[must_use]
    fn sum_rolls(rolls: &[u8]) -> Option<u32> {
        rolls
            .iter()
            .try_fold(0u32, |acc, &elem| acc.checked_add(elem.into()))
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
