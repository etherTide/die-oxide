use crate::dice::{DiceTray, Die};
use std::{fmt::Display, rc::Rc, str::FromStr, sync::Arc};

use color_eyre::eyre::{Ok, Report, Result};

#[derive(Clone, Debug, PartialEq)]
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

    // TODO: Extract the loops into their own function as they're very repetitive
    fn split_cmd_string(s: &str) -> Result<(CountStr, DieStr, ModsStr)> {
        //! Tries to find and pop a string of numbers from the begining of the slice, or returns an empty
        //! `count_str` and goes to the next step.
        //! If the string starts with `'d'` at this point, tries to find and pop a subsequent string of numbers, else returns empty `die_str` and continues.
        //! Finally, checks that the remaining slice is either empty or only contains modifiers,
        //! returning the entire tail as a `mod_str` on success.
        //! If the slice still contains unmatchable patterns, an `Err(Report)` is returned.
        let mut roll_string = s.to_owned();
        let count_str = CountStr::get_from_roll_str(&roll_string);
        roll_string = roll_string.replacen(&count_str.0, "", 1);
        let die_str = DieStr::get_from_roll_str(&roll_string);
        roll_string = roll_string.replacen(&die_str.0, "", 1);
        let mods_str = ModsStr::get_from_roll_str(&roll_string);
        roll_string = roll_string.replacen(&mods_str.0, "", 1);
        if roll_string.is_empty() {
            Ok((count_str, die_str, mods_str))
        } else {
            Err(Report::msg(format!(
                "Error: Could not parse input into the form `[count]d[die]+/-[modifiers]`\nInput: {s}\nCount: {}\nDie: {}\nMods: {}",
                count_str.0, die_str.0, mods_str.0
            )))
        }
    }
}
impl FromStr for RollCommand {
    type Err = Report;
    /// The format of a `RollCommand` is:
    /// `[count]` `['d'size]` `[('+'/'-')modifier]*`
    /// eg `"3d8+2-1"`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count_str, die_str, mods_str) = Self::split_cmd_string(s)?;
        Ok(Self::new(
            count_str.parse()?,
            die_str.parse()?,
            &mods_str.parse()?,
        ))
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
struct CountStr(String);
impl CountStr {
    fn parse(&self) -> Result<u8> {
        let s = &self.0;
        let num: u8 = if s.is_empty() { 1 } else { self.0.parse()? };
        Ok(num)
    }
    fn get_from_roll_str(roll_str: &str) -> Self {
        let mut buf = String::with_capacity(roll_str.len());
        for c in roll_str.chars() {
            if c.is_ascii_digit() {
                buf.push(c);
            } else {
                break;
            }
        }
        buf.shrink_to_fit();
        Self(buf)
    }
}
struct DieStr(String);
impl DieStr {
    fn parse(&self) -> Result<Die> {
        let s = &self.0;
        let die = if s.is_empty() {
            Die::default()
        } else {
            let digits: &str = s
                .get(1..)
                .map_or_else(|| Err(Report::msg("Couldn't parse die string")), Ok)?;
            Die(digits.parse()?)
        };
        Ok(die)
    }
    fn get_from_roll_str(roll_str: &str) -> Self {
        let mut buf = String::with_capacity(roll_str.len());
        let iter = roll_str.chars().enumerate();
        for (idx, c) in iter {
            if (idx == 0 && c == 'd') || c.is_ascii_digit() {
                buf.push(c);
            } else {
                break;
            }
        }
        buf.shrink_to_fit();
        Self(buf)
    }
}
struct ModsStr(String);
impl ModsStr {
    fn parse(&self) -> Result<Vec<i8>> {
        let s = &self.0;
        if s.is_empty() {
            return Ok(Vec::with_capacity(0));
        }
        // PERF: capacity = 4 bc I'm assuming 1x +/- plus 3x digit
        let mut mods: Vec<Rc<str>> = Vec::with_capacity(self.0.len() / 4);
        let mut buf = String::with_capacity(4);
        for c in s.chars() {
            // TODO: consider replacing with "+-".contains(c)?
            if !buf.is_empty() && !c.is_ascii_digit() {
                mods.push(buf.clone().into());
                buf.clear();
            }
            buf.push(c);
        }
        mods.push(buf.into());
        mods.into_iter().map(|string| Ok(string.parse()?)).collect()
    }
    fn get_from_roll_str(roll_str: &str) -> Self {
        let mut buf = String::with_capacity(roll_str.len());
        for c in roll_str.chars() {
            if c.is_ascii_digit() {
                buf.push(c);
            } else if "+-".contains(c) && {
                match buf.chars().last() {
                    // Don't want "+-XX" etc, should be 1 +/- per mod
                    Some(prev) => !"+-".contains(prev),
                    _ => true,
                }
            } {
                buf.push(c);
            } else {
                break;
            }
        }
        buf.shrink_to_fit();
        ModsStr(buf)
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
        let result: Option<i64> = match (dice_tray.result, net_modifier) {
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
            .map_or_else(|| "OVERFLOW!".to_string(), |result| result.to_string());
        let dice_tray = &self.dice_tray;
        let net_modifier = self
            .net_modifier
            .map_or_else(|| "OVERFLOW!".to_string(), |modifier| modifier.to_string());
        let modifiers = self.modifiers.clone();
        write!(f, "{result}\n{dice_tray}\n{net_modifier}: {modifiers:?}")
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
            ("2", new_cmd(2, Die::default(), &[])),
            ("+3", new_cmd(1, Die::default(), &[3])),
            ("4d5+6", RollCommand::new(4, Die(5), &[6])),
            ("7d8+9-1", RollCommand::new(7, Die(8), &[9, -1])),
        ];
        for (s, cmd) in ok_strs {
            assert_eq!(RollCommand::from_str(s)?, cmd);
        }
        let err_strs: Vec<&str> = vec!["gibberish", "1d6+-1", "1d6-+1", "1d6d6", "d", "+", "d-"];
        for s in err_strs {
            assert!(RollCommand::from_str(s).is_err())
        }
        Ok(())
    }
}
