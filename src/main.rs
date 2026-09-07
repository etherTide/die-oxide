use clap::Parser;
use die_oxide::cli::Cli;

fn main() {
    let cli = Cli::parse();
    cli.run()
}
