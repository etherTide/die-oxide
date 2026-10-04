use crate::{
    dice::die::Die,
    rolls::roll_special::{flag::Flag, flag_triggers::FlagTriggers},
};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiceTray {
    pub die: Die,
    pub results: Rc<[u8]>,
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
