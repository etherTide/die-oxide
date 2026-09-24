use crate::{dice::DiceTray, roll::RollCommand};
use std::{fmt::Display, sync::Arc};

#[derive(Debug)]
pub struct RollResult {
    dice_tray: DiceTray,
    modifiers: Arc<[i8]>,
    net_modifier: Option<i32>,
    result: Option<i64>,
}
impl RollResult {
    pub(super) fn new(cmd: &RollCommand) -> Self {
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
            .map_or_else(|| "OVERFLOW!".to_string(), |result| result.to_string());
        let sum = &self
            .dice_tray
            .sum
            .map_or_else(|| "OVERFLOW!".to_string(), |sum| sum.to_string());
        let rolls = &self.dice_tray.rolls;
        let net_modifier = self.net_modifier.map_or_else(
            || "OVERFLOW!".to_string(),
            |modifier| {
                let mut s = modifier.to_string();
                if modifier >= 0 {
                    s.insert(0, '+');
                }
                s
            },
        );
        let modifiers = self.modifiers.clone();
        write!(
            f,
            "{result}\n> Rolled {sum}: {rolls:?}\n> {net_modifier}: {modifiers:?}"
        )
    }
}
