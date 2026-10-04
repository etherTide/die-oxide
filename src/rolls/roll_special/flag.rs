use crate::{
    dice::{dice_tray::DiceTray, die::Die},
    rolls::roll_special::{matches::Matches, pattern::Pattern},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flag {
    pub action: Action,
    pub pattern: Pattern,
}
impl Flag {
    pub fn new(action: Action, pattern: Pattern) -> Self {
        Self { action, pattern }
    }
    pub fn trigger(&self, dice_tray: &mut DiceTray) -> Matches {
        let mut trigger_dice = Vec::<&Die>::new();
        let recursion: FlagRecursion = match self.action {
            Action::ReRoll(rec) | Action::Explode(rec) => rec,
            _ => FlagRecursion::None,
        };
        let trigger_limit = if let FlagRecursion::Limit(l) = recursion {
            l + 1
        } else {
            1
        };
        let generation = 1;
        todo!();
        // while generation <= trigger_limit {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Drop,
    /// field: allow recursive re-rolling?
    ReRoll(FlagRecursion),
    Explode(FlagRecursion),
    Substitute,
}
impl Action {
    pub fn act_on(&self, dice_tray: &DiceTray, matches: &Matches) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FlagRecursion {
    #[default]
    None,
    Limit(u8),
}
