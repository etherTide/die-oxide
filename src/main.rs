use clap::Parser;
use die_oxide::cli::{Cli, RollCommand};

fn main() {
    let cli = Cli::parse();
    if let Some(roll_cmds) = cli.roll_cmds {
        for cmd in roll_cmds {
            cmd.print_roll();
        }
    } else {
        let cmd = RollCommand::default();
        cmd.print_roll();
    }
}
