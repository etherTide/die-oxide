use crate::{
    dice::dice_tray::DiceTray,
    rolls::roll_special::{flag_triggers::FlagTriggers, pattern::Pattern},
};

pub struct Flag {
    pub action: Action,
    pub pattern: Pattern,
}

pub enum Action {
    Drop,
    /// field: allow recursive re-rolling?
    ReRoll(FlagRecursion),
    Explode(FlagRecursion),
    Substitute,
}

pub enum FlagRecursion {
    None,
    Limit(u8),
}
