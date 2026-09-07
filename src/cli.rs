use crate::roll::RollCommand;

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
