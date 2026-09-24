use crate::{dice::Die, roll::RollCommand};
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
pub(crate) struct RollString {
    count: String,
    die: String,
    mods: String,
}
impl FromStr for RollString {
    type Err = Report;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        //! Tries to find and pop a string of numbers from the begining of the slice, or returns an empty
        //! `count_str` and goes to the next step.
        //! If the string starts with `'d'` at this point, tries to find and pop a subsequent string of numbers, else returns empty `die_str` and continues.
        //! Finally, checks that the remaining slice is either empty or only contains modifiers,
        //! returning the entire tail as a `mod_str` on success.
        //! If the slice still contains unmatchable patterns, an `Err(color_eyre::Report)` is returned.
        let mut roll_string = s.to_owned();
        let count_string = Self::get_count(&roll_string);
        roll_string = roll_string.replacen(&count_string, "", 1);
        let die_string = Self::get_die(&roll_string);
        roll_string = roll_string.replacen(&die_string, "", 1);
        let mods_string = Self::get_mods(&roll_string);
        roll_string = roll_string.replacen(&mods_string, "", 1);
        if roll_string.is_empty() {
            Ok(Self {
                count: count_string,
                die: die_string,
                mods: mods_string,
            })
        } else {
            Err(Report::msg(format!(
                "Error: Could not parse input into the form `[count]d[die]+/-[modifiers]`\nInput: {s}\nCount: {count_string}\nDie: {die_string}\nMods: {mods_string}"
            )))
        }
    }
}
impl RollString {
    pub(crate) fn parse(&self) -> Result<RollCommand> {
        let count = self.parse_count()?;
        let die = self.parse_die()?;
        let modifiers = &self.parse_mods()?;
        Ok(RollCommand::new(count, die, modifiers))
    }
    fn parse_count(&self) -> Result<u8> {
        let s = &self.count;
        let num: u8 = if s.is_empty() { 1 } else { s.parse()? };
        Ok(num)
    }
    fn get_count(roll_str: &str) -> String {
        let mut count_string = String::with_capacity(roll_str.len());
        for c in roll_str.chars() {
            if c.is_ascii_digit() {
                count_string.push(c);
            } else {
                break;
            }
        }
        count_string.shrink_to_fit();
        count_string
    }
    fn parse_die(&self) -> Result<Die> {
        let s = &self.die;
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
    fn get_die(roll_str: &str) -> String {
        let mut die_string = String::with_capacity(roll_str.len());
        let iter = roll_str.chars().enumerate();
        for (idx, c) in iter {
            if (idx == 0 && c == 'd') || c.is_ascii_digit() {
                die_string.push(c);
            } else {
                break;
            }
        }
        die_string.shrink_to_fit();
        die_string
    }

    fn parse_mods(&self) -> Result<Vec<i8>> {
        let s = &self.mods;
        if s.is_empty() {
            return Ok(Vec::new());
        }
        // PERF: capacity = 4 bc I'm assuming 1x +/- plus 3x digit
        let mut mods: Vec<Rc<str>> = Vec::with_capacity(s.len() / 4);
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
    fn get_mods(roll_str: &str) -> String {
        let mut mods_string = String::with_capacity(roll_str.len());
        for c in roll_str.chars() {
            if c.is_ascii_digit()
                || ("+-".contains(c)
                    // Don't want "+-XX" etc, should be 1 +/- per mod
                    && mods_string.chars().last().is_none_or(
                        |prev| !"+-".contains(prev),
                    ))
            {
                mods_string.push(c);
            } else {
                break;
            }
        }
        mods_string.shrink_to_fit();
        mods_string
    }
}
