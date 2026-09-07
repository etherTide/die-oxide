use crate::dice::{DiceTray, Die};
use std::{fmt::Display, rc::Rc, str::FromStr, sync::Arc};

use color_eyre::eyre::{Ok, OptionExt, Report, Result};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub roll_cmds: Option<Vec<RollCommand>>,
}

#[derive(Clone, Debug)]
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
        let roll = self.roll();
        println!("{roll}");
    }
    fn sum_mods(&self) -> Option<i32> {
        self.modifiers
            .iter()
            .try_fold(0_i32, |acc, &elem| acc.checked_add(elem.into()))
    }

    fn split_cmd_string(s: &str) -> Result<(CountStr, DieStr, ModsStr)> {
        //! Tries to find and pop a string of numbers from the begining of the slice, or returns an empty
        //! `count_str` and goes to the next step.
        //! If the string starts with `'d'` at this point, tries to find and pop a subsequent string of numbers, else returns empty `die_str` and continues.
        //! Finally, checks that the remaining slice is either empty or only contains modifiers,
        //! returning the entire tail as a `mod_str` on success.
        //! If the slice still contains unmatchable patterns, an `Err(Report)` is returned.
        let mut buf = String::with_capacity(s.len());
        let mut used: Option<usize> = Some(0);
        let mut s_iter = s
            .chars()
            .skip(used.ok_or_eyre("Roll cmd might have been too long?")?)
            .enumerate()
            .peekable();
        // count
        loop {
            if let Some(element) = s_iter.peek() {
                if CountStr::is_match(element) {
                    buf.push(element.1);
                    s_iter.next();
                    used = used.map_or_else(|| None, |used| used.checked_add(1));
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
        s_iter = s
            .chars()
            .skip(used.ok_or_eyre("Roll cmd might have been too long?")?)
            .enumerate()
            .peekable();
        loop {
            if let Some(element) = s_iter.peek() {
                if DieStr::is_match(element) {
                    buf.push(element.1);
                    s_iter.next();
                    used = used.map_or_else(|| None, |used| used.checked_add(1));
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
        s_iter = s
            .chars()
            .skip(used.ok_or_eyre("Roll cmd might have been too long?")?)
            .enumerate()
            .peekable();
        loop {
            if let Some(&(idx, elem)) = s_iter.peek() {
                if ModsStr::is_match(&(idx, elem)) {
                    buf.push(elem);
                    s_iter.next();
                    used = used.map_or_else(|| None, |used| used.checked_add(1));
                } else {
                    // `"+1?".get(2) == '?'` -> could be `"+1-2"` and therefore part of new sequence
                    if idx > 1 {
                        // reset enumeration
                        s_iter = s
                            .chars()
                            .skip(used.ok_or_eyre("Roll cmd might have been too long?")?)
                            .enumerate()
                            .peekable();
                        continue;
                    }
                    // fail-case: mod string cannot be shorter than 2 chars
                    return Err(Report::msg(
                        "Error: Input string was not of the form `[count]d[die]+/-[modifiers]`",
                    ));
                }
            } else {
                return Ok((count_str, die_str, ModsStr::new(&buf)));
            }
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
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
    fn parse(&self) -> Result<u8> {
        let num: u8 = self.0.parse()?;
        Ok(num)
    }
    const fn is_match(element: &(usize, char)) -> bool {
        let (_idx, c) = *element;
        c.is_ascii_digit()
    }
}
struct DieStr(String);
impl DieStr {
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
    fn parse(&self) -> Result<Die> {
        let digits: &str = self
            .0
            .get(1..)
            .map_or_else(|| Err(Report::msg("Couldn't parse die string")), Ok)?;
        let num: u8 = digits.parse()?;
        Ok(Die::new(num))
    }
    const fn is_match(element: &(usize, char)) -> bool {
        let (idx, c) = *element;
        if idx == 0 {
            c == 'd'
        } else {
            c.is_ascii_digit()
        }
    }
}
struct ModsStr(String);
impl ModsStr {
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
    fn parse(&self) -> Result<Vec<i8>> {
        let s = &self.0;
        let mut mods: Vec<Rc<str>> = Vec::with_capacity(self.0.len() / 4); // PERF: assuming 1 `+`/`-` plus 3 digits
        let mut buf = String::with_capacity(4);
        for c in s.chars() {
            if !buf.is_empty() && !c.is_ascii_digit() {
                mods.push(buf.clone().into());
                buf.clear();
            }
            buf.push(c);
        }
        mods.push(buf.into());
        mods.into_iter()
            .map(|string| Self::parse_mod(&string))
            .collect()
    }
    fn parse_mod(modifier: &str) -> Result<i8> {
        let num: i8 = modifier.parse()?;
        Ok(num)
    }
    fn is_match(element: &(usize, char)) -> bool {
        let (idx, c) = *element;
        match idx {
            0 => "+-".contains(c),
            _ => c.is_ascii_digit(),
        }
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
