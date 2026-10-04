use crate::dice::die::Die;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiceTray {
    pub die: Die,
    pub results: Vec<u8>,
}
impl DiceTray {
    pub fn new(die: Die, count: u8) -> Self {
        let rolls = (1..count).map(|_| die.roll()).collect();
        Self {
            die,
            results: rolls,
        }
    }
}
