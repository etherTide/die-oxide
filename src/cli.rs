use crate::dice::Die;
use crate::roll::RollCommand;
use color_eyre::{Report, Result};
use std::{rc::Rc, str::FromStr};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
/// A command-line dice roller
pub struct Cli {
    /// [`count`] [d `die`] [+ | - `mod`]* ...
    ///
    /// e.g. 8d6-1+3 2d4+2+0-1 5 +2 d100 ...
    /// default: 1d6+0
    pub roll_cmds: Option<Vec<RollCommand>>,
}
impl Cli {
    pub fn run(&self) {
        if let Some(roll_cmds) = &self.roll_cmds {
            for cmd in roll_cmds {
                cmd.print_roll();
            }
        } else {
            let cmd = RollCommand::default();
            cmd.print_roll();
        }
    }
}
impl FromStr for RollCommand {
    type Err = Report;
    /// The format of a `RollCommand` is:
    /// `[count]` `['d'size]` `[('+'/'-')modifier]*`
    /// eg `"3d8+2-1"`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count_str, die_str, mods_str) = split_cmd_string(s)?;
        Ok(Self::new(
            count_str.parse()?,
            die_str.parse()?,
            &mods_str.parse()?,
        ))
    }
}
fn split_cmd_string(s: &str) -> Result<(CountStr, DieStr, ModsStr)> {
    //! Tries to find and pop a string of numbers from the begining of the slice, or returns an empty
    //! `count_str` and goes to the next step.
    //! If the string starts with `'d'` at this point, tries to find and pop a subsequent string of numbers, else returns empty `die_str` and continues.
    //! Finally, checks that the remaining slice is either empty or only contains modifiers,
    //! returning the entire tail as a `mod_str` on success.
    //! If the slice still contains unmatchable patterns, an `Err(color_eyre::Report)` is returned.
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

struct CountStr(String);
impl CountStr {
    fn parse(&self) -> Result<u8> {
        let s = &self.0;
        let num: u8 = if s.is_empty() { 1 } else { s.parse()? };
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
            return Ok(Vec::new());
        }
        // PERF: capacity = 4 bc I'm assuming 1x +/- plus 3x digit
        let mut mods: Vec<Rc<str>> = Vec::with_capacity(self.0.len() / 4);
        let mut buf = String::with_capacity(4);
        for c in s.chars() {
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
            if c.is_ascii_digit()
                || ("+-".contains(c)
                    // Don't want "+-XX" etc, should be 1 +/- per mod
                    && buf.chars().last().is_none_or(
                        |prev| !"+-".contains(prev),
                    ))
            {
                buf.push(c);
            } else {
                break;
            }
        }
        buf.shrink_to_fit();
        Self(buf)
    }
}
