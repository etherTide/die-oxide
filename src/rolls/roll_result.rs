use crate::{
    dice::dice_tray::DiceTray,
    rolls::{roll_command::RollCommand, roll_special::flag_triggers::FlagTriggers},
};
use std::rc::Rc;

pub struct RollResult {
    pub dice_tray: DiceTray,
    pub natural_result: u32,
    pub modifiers: Rc<[i8]>,
    pub modifier_sum: i32,
    flag_triggers: FlagTriggers,
}
impl RollResult {
    pub fn new(&roll_command: &RollCommand) -> Self {
        let mut dice_tray = DiceTray::new(roll_command.die, roll_command.count);
        let flag_triggers = dice_tray.get_flag_triggers(roll_command.flag);
        let natural_result = dice_tray.1.iter().map(|&num| u32::from(num)).sum();
        let modifiers = roll_command.modifiers;
        let modifier_sum = modifiers.iter().map(|&num| i32::from(num)).sum();
        Self {
            dice_tray,
            natural_result,
            modifiers,
            modifier_sum,
            flag_triggers,
        }
    }
}
