use color_eyre::Report;

use crate::{
    dice::Die,
    roll::{roll_flags::RollFlags, roll_result::RollResult},
};
use std::{fmt::Display, str::FromStr, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq)]
// Required to be thread-safe in order to inter-opt with clap
pub struct RollCommand {
    pub count: u8,
    pub die: Die,
    pub modifiers: Arc<[i8]>,
    pub flags: RollFlags,
}
impl RollCommand {
    #[must_use]
    pub fn new(count: u8, die: Die, modifiers: &[i8], flags: RollFlags) -> Self {
        Self {
            count,
            die,
            modifiers: modifiers.into(),
            flags,
        }
    }
    #[must_use]
    pub fn roll(&self) -> RollResult {
        RollResult::new(self)
    }
    pub fn print_roll(&self) {
        println!("Rolling {self}...");
        println!("{}", self.roll());
    }
    pub(super) fn sum_mods(&self) -> Option<i32> {
        self.modifiers
            .iter()
            .try_fold(0_i32, |acc, &elem| acc.checked_add(elem.into()))
    }
}
impl Default for RollCommand {
    fn default() -> Self {
        Self::new(1, Die::default(), &[], RollFlags::default())
    }
}
impl Display for RollCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mods_str: String = self
            .modifiers
            .iter()
            .map(|&elem| {
                let mut s = elem.to_string();
                if elem >= 0 {
                    s.insert(0, '+');
                }
                s
            })
            .collect();
        write!(f, "{}{}{}", self.count, self.die, mods_str)
    }
}
impl FromStr for RollCommand {
    type Err = Report;
    /// The format of a `RollCommand` is:
    /// `[count]` `['d'size]` `[('+'/'-')modifier]*`
    /// eg `"3d8+2-1"`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let roll_string = crate::cli::RollString::from_str(s)?;
        roll_string.parse()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dice::Die,
        roll::{RollCommand, roll_flags::RollFlags},
    };
    use std::{assert_matches, str::FromStr};

    #[test]
    fn roll_cmd_from_string() {
        let new_cmd = RollCommand::new;
        let ok_strs: Vec<(&str, RollCommand)> = vec![
            ("", RollCommand::default()),
            ("2d3", new_cmd(2, Die(3), &[], RollFlags::default())),
            ("4d5+6", new_cmd(4, Die(5), &[6], RollFlags::default())),
            ("d7", new_cmd(1, Die(7), &[], RollFlags::default())),
            ("d8+9", new_cmd(1, Die(8), &[9], RollFlags::default())),
            ("-0", new_cmd(1, Die::default(), &[0], RollFlags::default())),
            (
                "1-2",
                new_cmd(1, Die::default(), &[-2], RollFlags::default()),
            ),
            (
                "150d200+10-20+30-40",
                new_cmd(150, Die(200), &[10, -20, 30, -40], RollFlags::default()),
            ),
        ];
        for (s, cmd) in ok_strs {
            assert_matches!(RollCommand::from_str(s), Ok(parsed_s) if parsed_s == cmd);
        }
        let err_strs: Vec<&str> = vec![
            "gibberish",
            "1d6+-1",
            "1d6-+1",
            "1d6d6",
            "d",
            "+",
            "d-",
            "-1d6",
        ];
        for s in err_strs {
            assert!(RollCommand::from_str(s).is_err());
        }
    }
}
