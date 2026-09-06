use clap::Parser;
use die_oxide::cli::{Cli, RollCommand};

fn main() {
    let cli = Cli::parse();
    match cli.roll_cmds {
        Some(roll_cmds) => {
            for cmd in roll_cmds {
                cmd.print_roll();
            }
        }
        None => {
            let cmd = RollCommand::default();
            cmd.print_roll();
        }
    }
}
