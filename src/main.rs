use clap::Parser;
use die_oxide::cli::{Cli, RollCommand};

fn main() {
    let cli = Cli::parse();
    match cli.rolls {
        Some(roll_cmds) => {
            for roll in roll_cmds {
                println!("{}", roll)
            }
        }
        None => {
            let roll = RollCommand::default();
            println!("{}", roll)
        }
    }
}
