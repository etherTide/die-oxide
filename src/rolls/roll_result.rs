use crate::{
    dice::dice_tray::DiceTray,
    rolls::{
        roll_command::RollCommand,
        roll_special::{flag::Flag, matches::Matches},
    },
};
use std::rc::Rc;

pub struct RollResult<'a> {
    pub dice_tray: DiceTray,
    pub natural_result: u32,
    pub modifiers: Rc<[i8]>,
    pub modifier_sum: i32,
    matches: Option<Matches<'a>>,
}
impl RollResult<'_> {
    pub fn new(roll_command: RollCommand) -> Self {
        let mut dice_tray = DiceTray::new(roll_command.die, roll_command.count);
        let matches = if let Some(flag) = roll_command.flag {
            Some(flag.trigger(&mut dice_tray))
        } else {
            None
        };
        let natural_result = dice_tray.results.iter().map(|&num| u32::from(num)).sum();
        let modifiers = roll_command.modifiers;
        let modifier_sum = modifiers.iter().map(|&num| i32::from(num)).sum();
        Self {
            dice_tray,
            natural_result,
            modifiers,
            modifier_sum,
            matches,
        }
    }
}
