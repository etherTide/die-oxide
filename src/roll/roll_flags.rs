use super::roll_trigger::RollTrigger;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RollFlags {
    drop: Option<DropFlag>,
    explode: Option<ExplodeFlag>,
    substitute: Option<SubstituteFlag>,
}
impl RollFlags {
    #[must_use]
    pub const fn new(
        drop: Option<DropFlag>,
        explode: Option<ExplodeFlag>,
        substitute: Option<SubstituteFlag>,
    ) -> Self {
        Self {
            drop,
            explode,
            substitute,
        }
    }
}
impl Default for RollFlags {
    /// Returns a set of empty flags
    fn default() -> Self {
        Self::new(None, None, None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropFlag {
    DropHighest(RollTrigger),
    DropLowest(RollTrigger),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExplodeFlag;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubstituteFlag {
    ReRoll(RollTrigger),
    Ceiling(RollTrigger),
    Floor(RollTrigger),
}
