use crate::{dice::dice_tray::DiceTray, rolls::roll_special::trigger_pattern::TriggerPattern};
use std::rc::Rc;

pub enum Flag {
    Drop(TriggerPattern),
    Reroll(TriggerPattern),
    Explode(TriggerPattern),
    Substitute(TriggerPattern),
}
