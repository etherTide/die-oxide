#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollTrigger {
    BelowMax(u8),
    GreaterThan(u8),
    Exactly(u8),
    LessThan(u8),
    AboveMin(u8),
}
