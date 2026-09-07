use crate::dice::{DiceTray, Die};
use std::{fmt::Display, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RollCommand {
    count: u8,
    die: Die,
    modifiers: Arc<[i8]>,
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
    fn sum_mods(&self) -> Option<i32> {
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

#[derive(Debug)]
pub struct RollResult {
    dice_tray: DiceTray,
    modifiers: Arc<[i8]>,
    net_modifier: Option<i32>,
    result: Option<i64>,
}
impl RollResult {
    fn new(cmd: &RollCommand) -> Self {
        let dice_tray = DiceTray::new(cmd.die, cmd.count);
        let modifiers = cmd.modifiers.clone();
        let net_modifier: Option<i32> = cmd.sum_mods();
        let result: Option<i64> = match (dice_tray.sum, net_modifier) {
            (Some(a), Some(b)) => i64::checked_add(a.into(), b.into()),
            _ => None,
        };
        Self {
            dice_tray,
            modifiers,
            net_modifier,
            result,
        }
    }
}
impl Display for RollResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .result
            .map_or("OVERFLOW!".to_string(), |result| result.to_string());
        let sum = &self
            .dice_tray
            .sum
            .map_or("OVERFLOW!".to_string(), |sum| sum.to_string());
        let rolls = &self.dice_tray.rolls;
        let net_modifier = self
            .net_modifier
            .map_or("OVERFLOW!".to_string(), |modifier| {
                let mut s = modifier.to_string();
                if modifier >= 0 {
                    s.insert(0, '+');
                }
                s
            });
        let modifiers = self.modifiers.clone();
        write!(
            f,
            "{result}\n> Rolled {sum}: {rolls:?}\n> {net_modifier}: {modifiers:?}"
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use color_eyre::eyre::Result;

    use crate::{dice::Die, roll::RollCommand};

    #[test]
    fn roll_cmd_from_string() -> Result {
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
            assert_eq!(RollCommand::from_str(s)?, cmd);
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
            assert!(RollCommand::from_str(s).is_err())
        }
        Ok(())
    }
}
