#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DieResult {
    pub value: u8,
    pub dropped: bool,
    pub exploded: bool,
    pub substituted: bool,
    pub re_rolled: bool,
}
