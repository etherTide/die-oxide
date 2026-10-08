pub mod dice;
pub mod flags;
pub mod patterns;

fn main() {
    let tower = dice::DiceTower {
        die: dice::Die(8),
        count: 3,
    };
    dbg!(&tower);
    let roll = tower.roll();
    dbg!(roll);
}
