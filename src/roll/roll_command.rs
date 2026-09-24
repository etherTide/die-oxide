use crate::{dice::Die, roll::roll_result::RollResult};
use std::{fmt::Display, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RollCommand {
    pub(super) count: u8,
    pub(super) die: Die,
    pub(super) modifiers: Arc<[i8]>,
}
impl RollCommand {
    #[must_use]
    pub fn new(count: u8, die: Die, modifiers: &[i8]) -> Self {
        Self {
            count,
            die,
            modifiers: modifiers.to_owned().into(),
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
        Self::new(1, Die::default(), &[])
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

#[cfg(test)]
mod tests {
    use crate::{dice::Die, roll::RollCommand};
    use std::{assert_matches, str::FromStr};

    #[test]
    fn roll_cmd_from_string() {
        let new_cmd = RollCommand::new;
        let ok_strs: Vec<(&str, RollCommand)> = vec![
            ("", RollCommand::default()),
            ("2d3", new_cmd(2, Die(3), &[])),
            ("4d5+6", new_cmd(4, Die(5), &[6])),
            ("d7", new_cmd(1, Die(7), &[])),
            ("d8+9", new_cmd(1, Die(8), &[9])),
            ("-0", new_cmd(1, Die::default(), &[0])),
            ("1-2", new_cmd(1, Die::default(), &[-2])),
            (
                "150d200+10-20+30-40",
                new_cmd(150, Die(200), &[10, -20, 30, -40]),
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
