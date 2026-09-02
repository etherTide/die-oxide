use crate::dice::{DiceTray, Die};

use std::{fmt::Display, str::FromStr};

use clap::Parser;
use color_eyre::eyre::{Ok, OptionExt, Report, Result, WrapErr};

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
    fn split_cmd_string(s: &str) -> Result<(&str, &str, &str)> {
        todo!();
    }
    fn parse_num(s: &str) -> Result<u8> {
        let mut num: u8 = 0;
        for c in s.chars() {
            // shift digits left and append next
            // HACK: saturating add instead of handling overflow
            num = num.saturating_mul(10);
            num = num.saturating_add(u8::try_from(
                c.to_digit(10).ok_or_eyre("Error parsing digit")?,
            )?);
        }
        if num == 0 { Ok(1) } else { Ok(num) }
    }
    fn parse_die(s: &str) -> Result<Die> {
        match s.chars().nth(0) {
            Some('d') => {
                let num = Self::parse_num(&s[1..])?;
                Ok(Die::new(num))
            }
            None => Ok(Die::default()), // empty string slice
            _ => Err(Report::msg("Die string must start with 'd', eg 'd20'")),
        }
    }
    fn parse_mods(s: &str) -> Result<Vec<i8>> {
        s.split_whitespace()
            .map(|m| {
                let mut m_val: i8 = 0;
                for (i, c) in m[1..].char_indices() {
                    let digit: Result<i8> = i8::try_from(c.to_digit(10).unwrap_or_default())
                        .wrap_err("Error parsing digit");
                    let significance: Result<u32> =
                        u32::try_from(m.len() - i).wrap_err("Error parsing digit");
                    m_val += i8::try_from(digit? * i8::pow(10, significance?))?;
                }
                match m.chars().nth(0) {
                    // FIXME:
                    Some('+') => Ok(m_val),
                    Some('-') => Ok(-1 * m_val),
                    _ => {
                        return Err(Report::msg(
                            "Modifier string must start with ' +' or ' -', eg ' +2'",
                        ));
                    }
                }
            })
            .collect()
    }
}
impl FromStr for RollCommand {
    /// The format of a RollCommand is:
    /// [count] ['d'size] [(' +'/' -')modifier]*
    /// eg "3d8 +2 -1"
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count_str, die_str, mods_str) = Self::split_cmd_string(s)?;
        let count = Self::parse_num(count_str)?;
        let die = Self::parse_die(die_str)?;
        let modifiers = Self::parse_mods(mods_str)?;
        Ok(Self {
            count,
            die,
            modifiers,
        })
    }
}
impl RollCommand {
    fn roll(&self) -> RollResult {
        RollResult::new(&self)
    }
    fn new(count: u8, die: Die, modifiers: &[i8]) -> Self {
        Self {
            count,
            die,
            modifiers: modifiers.to_owned(),
        }
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
        if mods.len() == 0 {
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
        if mods.len() == 0 {
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
