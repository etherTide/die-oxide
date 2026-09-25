use crate::dice::DiceTray;

trait RollSpecial {
    fn roll_special(original: DiceTray) -> DiceTray;
}
