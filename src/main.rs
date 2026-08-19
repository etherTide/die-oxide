use rand::random_range;
// use clap::Parser;
//
// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[arg(short, long)]
//     name: String,
//
//     /// Number of times to greet
//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
// }
//

#[derive(Debug)]
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
}

fn main() {
    let standard_dice = [
        Die::new(4),
        Die::new(6),
        Die::new(8),
        Die::new(10),
        Die::new(12),
        Die::new(20),
        Die::new(100),
    ];
    for die in standard_dice {
        let result = die.roll();
        println!("{die:?} => {result:?}");
    }

    // let args = Args::parse();

    // if args.count == 0 {
    //     interactive_mode();
    //     return;
    // }
}
