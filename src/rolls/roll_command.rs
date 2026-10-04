use crate::{dice::die::Die, rolls::roll_special::flag::Flag};
use std::rc::Rc;

pub struct RollCommand {
    pub die: Die,
    pub count: u8,
    pub modifiers: Rc<[i8]>,
    pub flag: Option<Flag>,
}
