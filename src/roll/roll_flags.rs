use super::roll_trigger::RollTrigger;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RollFlags {
    drop: Option<DropFlag>,
    explode: Option<ExplodeFlag>,
    substitute: Option<SubstituteFlag>,
    re_roll: Option<ReRollFlag>,
}
impl RollFlags {
    #[must_use]
    pub const fn new(
        drop: Option<DropFlag>,
        explode: Option<ExplodeFlag>,
        substitute: Option<SubstituteFlag>,
        re_roll: Option<ReRollFlag>,
    ) -> Self {
        Self {
            drop,
            explode,
            substitute,
            re_roll,
        }
    }
}
impl Default for RollFlags {
    /// Returns a set of empty flags
    fn default() -> Self {
        Self::new(None, None, None, None)
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
    Ceiling(RollTrigger), // Advantage
    Floor(RollTrigger),   // Disadvantage
    Modify(RollTrigger),  // Treat all Xs as Ys
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReRollFlag {
    Disgressionary(RollTrigger),
    Mandatory(RollTrigger),
}
