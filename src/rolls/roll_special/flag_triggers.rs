use crate::{
    dice::dice_tray::DiceTray,
    rolls::roll_special::flag::{Action, Flag},
};
use std::rc::Rc;

pub struct FlagTriggers {
    pub trigger_indices: Option<Rc<usize>>,
}
impl FlagTriggers {
    pub fn new(dice_tray: &mut DiceTray, flag: Flag) -> Self {
        let trigger_indices = flag.pattern.get_matches(dice_tray);
        match flag.action {
            Action::Drop | Action::Substitute => _,
        }
        Self { trigger_indices }
    }
}
