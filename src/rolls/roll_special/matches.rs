use crate::{
    dice::{dice_tray::DiceTray, die::Die},
    rolls::roll_special::pattern::Pattern,
};

pub enum Matches<'a> {
    Dice(Vec<&'a Die>),
    DiceTray(bool),
}
impl<'a, 'd> Matches<'a>
where
    'd: 'a,
{
    pub fn new(dice_tray: &'d DiceTray, pattern: Pattern) -> Self {
        todo!();
    }
}
