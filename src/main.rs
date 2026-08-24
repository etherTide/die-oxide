use clap::Parser;
use rand::random_range;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
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
    /// Returns a tuple of the sum and a Vec of each die roll in that sum
    fn roll_multiple(&self, count: u8) -> (u32, Vec<u8>) {
        let n = count as usize;
        let mut results = Vec::with_capacity(n);
        let mut sum = 0u32;
        for i in 0..n {
            let roll = self.roll();
            results[i] = roll;
            sum += roll as u32;
        }
        (sum, results)
    }

    // fn standard_set() -> [Die; 7] {
    //     let dice: [u8; 7] = [4, 6, 8, 10, 12, 20, 100];
    //     dice.map(|die| Die::new(die))
    // }
}
impl Default for Die {
    fn default() -> Self {
        Self::new(6)
    }
}
impl From<u8> for Die {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

type DiceBag = HashMap<Die, u8>;
type DieRoll = (Die, u8);
#[derive(Debug)]
struct DiceTray {
    sum: u32,
    roll_results: Vec<DieRoll>,
}
impl DiceTray {
    fn new(dice_bag: DiceBag) -> Self {
        let mut sum = 0u32;
        let mut roll_results = Vec::new();
        for (die, count) in dice_bag {
            for _ in 0..count {
                let roll = die.roll();
                sum += roll as u32;
                let result: DieRoll = (die, roll);
                roll_results.push(result);
            }
        }
        Self { sum, roll_results }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = 1)]
    count: u8,

    /// Kind of dice to roll
    #[arg(short, long, default_value_t = 6)]
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
    let die: Die = args.die.into();
    let mut dice = DiceBag::from_iter([(die, args.count)]);
}
