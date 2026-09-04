use crate::dice::{DiceTray, Die};
use std::{
    fmt::Display,
    iter::Enumerate,
    str::{Chars, FromStr},
};

use clap::Parser;
use color_eyre::eyre::{Ok, Report, Result};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub rolls: Option<Vec<RollCommand>>,
}

#[derive(Clone, Debug)]
pub struct RollCommand {
    count: u8,
    die: Die,
    modifiers: Vec<i8>,
}
impl RollCommand {
    fn new(count: u8, die: Die, modifiers: &[i8]) -> Self {
        Self {
            count,
            die,
            modifiers: modifiers.to_owned(),
        }
    }
    fn roll(&self) -> RollResult {
        RollResult::new(&self)
    }
    /// Tries to find and pop a string of numbers from the begining of the slice, or returns an empty
    /// `count_str` and goes to the next step.
    /// If the string starts with `'d'` at this point, tries to find and pop a subsequent string of numbers, else returns empty `die_str` and continues.
    /// Finally, checks that the remaining slice is either empty or only contains modifiers,
    /// returning the entire tail as a `mod_str` on success.
    /// If the slice still contains unmatchable patterns, an `Err(Report)` is returned.
    fn split_cmd_string(s: &str) -> Result<(CountStr, DieStr, ModsStr)> {
        let mut buf = String::with_capacity(s.len());
        let mut used: usize = 0;
        let mut s_iter = s.chars().skip(used).enumerate().peekable();
        // count
        loop {
            if let Some(element) = s_iter.peek() {
                if CountStr::is_match(&element) {
                    buf.push(element.1);
                    s_iter.next();
                    used += 1;
                } else {
                    break;
                }
            } else {
                return Ok((CountStr::new(&buf), DieStr::new(""), ModsStr::new("")));
            }
        }
        let count_str = CountStr::new(&buf);
        buf.clear();
        // die
        // TODO: Check that enumerate doesn't keep counting from before the `skip()`
        s_iter = s.chars().skip(used).enumerate().peekable();
        loop {
            if let Some(element) = s_iter.peek() {
                if DieStr::is_match(element) {
                    buf.push(element.1);
                    s_iter.next();
                    used += 1;
                } else {
                    break;
                }
            } else {
                return Ok((count_str, DieStr::new(&buf), ModsStr::new("")));
            }
        }
        let die_str = DieStr::new(&buf);
        buf.clear();
        // mods
        s_iter = s.chars().skip(used).enumerate().peekable();
        loop {
            if let Some(element) = s_iter.peek() {
                if ModsStr::is_match(element) {
                    buf.push(element.1);
                    s_iter.next();
                    used += 1;
                } else {
                    // `" +1?".get(3) == '?'` -> could be `" +1 -2"` and therefore part of new sequence
                    if element.0 > 2 {
                        // reset enumeration
                        s_iter = s.chars().skip(used).enumerate().peekable();
                        continue;
                    };
                    // fail-case: mod string cannot be shorter than 3 chars
                    return Err(Report::msg(
                        "Error: Input string was not of the form `[count]d[die] +/-[modifiers]`",
                    ));
                }
            } else {
                return Ok((count_str, die_str, ModsStr::new(&buf)));
            }
        }
        // let mods_str = ModsStr::new(&buf);
        // if used == s.len() {
        //     Ok((count_str, die_str, mods_str))
        // } else {
        //     Err(Report::msg("Error: Failed to parse input"))
        // }
    }
}
impl FromStr for RollCommand {
    type Err = Report;
    /// The format of a `RollCommand` is:
    /// `[count]` `['d'size]` `[(' +'/' -')modifier]*`
    /// eg `"3d8 +2 -1"`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count_str, die_str, mods_str) = Self::split_cmd_string(s)?;
        let count = count_str.parse()?;
        let die = die_str.parse()?;
        let modifiers = mods_str.parse()?;
        Ok(Self {
            count,
            die,
            modifiers,
        })
    }
}
impl Default for RollCommand {
    fn default() -> Self {
        Self::new(1, Die::default(), &[])
    }
}
impl Display for RollCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let roll_result = self.roll();
        let roll_str = roll_result
            .dice_tray
            .results
            .iter()
            .map(|roll| roll.to_string() + " + ")
            .collect::<String>();
        let roll_str = roll_str.trim_end_matches(" + ");
        let mods = &self.modifiers;
        if mods.is_empty() {
            write!(
                f,
                "Result: {}\nRolled {}{}: {}",
                roll_result.result, self.count, self.die, roll_str
            )
        } else {
            let mod_str: String = mods
                .iter()
                .map(|&m| {
                    let mut s = String::with_capacity(3);
                    if m >= 0 {
                        s = "+".to_owned();
                    }
                    s += &m.to_string();
                    s + " "
                })
                .collect();
            write!(
                f,
                "Result: {}\nRolled {}{}: {}\nModifiers: {}",
                roll_result.result, self.count, self.die, roll_str, mod_str
            )
        }
    }
}
struct CountStr(String);
impl CountStr {
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
    fn parse(&self) -> Result<u8> {
        let big_num: u32 = self.0.parse()?;
        let small_num: u8 = big_num.try_into()?;
        Ok(small_num)
    }
    fn is_match(element: &(usize, char)) -> bool {
        let (_idx, c) = *element;
        c.is_digit(10)
    }
}
struct DieStr(String);
impl DieStr {
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
    fn parse(&self) -> Result<Die> {
        let digits: &str = match self.0.get(1..) {
            Some(slice) => Ok(slice),
            None => Err(Report::msg("Couldn't parse die string")),
        }?;
        let big_num: u32 = digits.parse()?;
        let small_num: u8 = big_num.try_into()?;
        Ok(Die::new(small_num))
    }
    fn is_match(element: &(usize, char)) -> bool {
        let (idx, c) = *element;
        if idx == 0 { c == 'd' } else { c.is_digit(10) }
    }
}
struct ModsStr(String);
impl ModsStr {
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
    fn parse(&self) -> Result<Vec<i8>> {
        let mods = self.0.split(' ');
        let (lower_bound, _) = mods.size_hint();
        let mut vec = Vec::<i8>::with_capacity(lower_bound);
        for modifier in mods {
            vec.push(Self::parse_mod(modifier)?);
        }
        Ok(vec)
    }
    fn parse_mod(modifier: &str) -> Result<i8> {
        let big_num: i32 = modifier.parse()?;
        let small_num: i8 = big_num.try_into()?;
        Ok(small_num)
    }
    fn is_match(element: &(usize, char)) -> bool {
        let (idx, c) = *element;
        match idx {
            0 => c == ' ',
            1 => "+-".contains(c),
            _ => c.is_digit(10),
        }
    }
}

#[derive(Debug)]
struct RollResult {
    dice_tray: DiceTray,
    result: i32,
}
impl RollResult {
    fn new(cmd: &RollCommand) -> Self {
        let dice_tray = DiceTray::new(cmd.die, cmd.count);
        let mut dice_sum: i32 = 0;
        for roll in &dice_tray.results {
            // HACK: saturate_add instead of handling overflow
            dice_sum = dice_sum.saturating_add(i32::from(*roll));
        }
        let mods = &cmd.modifiers;
        if mods.is_empty() {
            Self {
                dice_tray,
                result: dice_sum,
            }
        } else {
            let mod_sum: i32 = mods.iter().map(|x| i32::from(*x)).sum();
            // HACK: saturate_add instead of handling overflow
            let result = i32::saturating_add(dice_sum, mod_sum);
            Self { dice_tray, result }
        }
    }
}
fn split_at_non_digit(s: &str) -> Result<(&str, &str)> {
    match s.find(|c: char| !c.is_digit(10)) {
        Some(non_digit) => match s.split_at_checked(non_digit) {
            Some((first, second)) => Ok((first, second)),
            None => Err(Report::msg("Split occured inside UTF-character")),
        },
        None => Ok((s, "")),
    }
}
