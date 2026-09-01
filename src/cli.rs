use crate::dice::{DiceTray, Die};
use clap::Parser;
use color_eyre::eyre::Report;
use std::{fmt::Display, str::FromStr};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub rolls: Option<Vec<RollCommand>>,
}

#[derive(Clone, Debug)]
pub struct RollCommand {
    count: u8,
    die: Die,
    modifiers: Option<Vec<i8>>,
}
impl FromStr for RollCommand {
    type Err = Report;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        // FIXME
        Ok(Self::default())
    }
}
impl RollCommand {
    fn roll(&self) -> RollResult {
        RollResult::new(&self)
    }
    const fn new(count: u8, die: Die, modifiers: Option<Vec<i8>>) -> Self {
        Self {
            count,
            die,
            modifiers,
        }
    }
}
impl Default for RollCommand {
    fn default() -> Self {
        Self::new(1, Die::default(), None)
    }
}
impl Display for RollCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let roll_result = self.roll();
        let roll_str = roll_result
            .dice_tray
            .results
            .into_iter()
            .map(|roll| roll.to_string() + " + ")
            .collect::<String>();
        let roll_str = roll_str.trim_end_matches(" + ");
        match &self.modifiers {
            Some(mods) => {
                let mod_str: String = mods
                    .into_iter()
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
            None => write!(
                f,
                "Result: {}\nRolled {}{}: {}",
                roll_result.result, self.count, self.die, roll_str
            ),
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
            dice_sum += i32::from(*roll);
        }
        match cmd.modifiers {
            Some(ref mods) => {
                let mod_sum: i32 = mods.into_iter().map(|x| i32::from(*x)).sum();
                Self {
                    dice_tray,
                    result: dice_sum + mod_sum,
                }
            }
            None => Self {
                dice_tray,
                result: dice_sum,
            },
        }
    }
}
