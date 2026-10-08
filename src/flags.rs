#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flags {
    Drop,
    Substitute,
    ReRoll(Limit),
    Explode(Limit),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limit(Option<u8>);
