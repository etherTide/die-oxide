use crate::{
    dice::die::Die,
    rolls::roll_special::{flag::Flag, flag_triggers::FlagTriggers},
};
use std::rc::Rc;

pub struct DiceTray(pub Die, pub Rc<[u8]>);
impl DiceTray {
    pub fn new(die: Die, count: u8) -> Self {
        let rolls = (1..count).map(|_| die.roll()).collect();
        Self(die, rolls)
    }
    pub fn get_flag_triggers(&mut self, &flag: &Flag) -> FlagTriggers {}
}
