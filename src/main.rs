use clap::Parser;
use rand::random_range;

#[derive(Clone, Debug)]
struct Die {
    face_count: u8,
}
impl Die {
    const fn new(face_count: u8) -> Self {
        Self { face_count }
    }

    fn roll(&self) -> u8 {
        random_range(1..=self.face_count)
    }

    fn standard_set() -> [Die; 7] {
        let dice: [u8; 7] = [4, 6, 8, 10, 12, 20, 100];
        dice.map(|die| Die::new(die))
    }
}
impl Default for Die {
    fn default() -> Self {
        Self::new(6)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = 1)]
    count: u8,

    /// Kind of dice to roll
    #[arg(short, default_value_t = 6)]
    die: u8,

    /// Roll die twice and take the higher result
    #[arg(short, long)]
    advantage: bool,

    /// Roll die twice and take the lower result
    #[arg(short = 'A', long)]
    disadvantage: bool,
}

fn main() {
    let args = Args::parse();
    println!("{}", Die::new(args.die).roll_multiple(args.count));
}
