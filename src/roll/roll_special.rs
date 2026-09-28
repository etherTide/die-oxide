use crate::{
    dice::Die,
    roll::{
        RollCommand,
        roll_flags::{DropFlag, RollFlags},
        roll_trigger::RankTrigger::{Lower, Upper},
    },
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AdvantageType {
    #[default]
    Advantage,
    Disadvantage,
}
#[must_use]
pub fn get_advantage_roll(adv_type: AdvantageType, mods: &[i8]) -> RollCommand {
    let die = Die(20);
    let drop_flag = Some(match adv_type {
        AdvantageType::Advantage => DropFlag(Lower(1)),
        AdvantageType::Disadvantage => DropFlag(Upper(1)),
    });
    RollCommand::new(2, die, mods, RollFlags::new(drop_flag, None, None, None))
}
