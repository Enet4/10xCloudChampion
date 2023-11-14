use crate::{
    CloudClientSpec, Cost, Money, Ops, ServiceKind, WorldState, TIME_UNITS_PER_MILLISECOND,
};

pub mod all;

/// The specification for a card,
/// including in what circumstances it should become available.
#[derive(Debug)]
pub struct CardSpec {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub cost: Cost,
    pub condition: CardCondition,
    pub effect: CardEffect,
}

/// The condition at which a card should become available.
#[derive(Debug)]
pub enum CardCondition {
    /// the card should appear iif `test` is true
    Test { test: bool },
    /// the player has accumulated funds
    Funds(Money),
    /// the player has spent funds
    Spent(Money),
    /// the player has accrued a net total of base ops
    TotalBaseOps(Ops),
    /// the player has accumulated an amount of extra base ops
    AvailableBaseOps(Ops),
    /// the player has accrued a net total of super ops
    TotalSuperOps(Ops),
    /// the player has accumulated an amount of extra super ops
    AvailableSuperOps(Ops),
    /// the player has accrued a net total of epic ops
    TotalEpicOps(Ops),
    /// the player has accumulated an amount of extra epic ops
    AvailableEpicOps(Ops),
    /// the player has accrued a net total of awesome ops
    TotalAwesomeOps(Ops),
    /// the player has accumulated an amount of extra awesome ops
    AvailableAwesomeOps(Ops),

    /// appear N ticks after another card has been used
    TimeAfterCard {
        /// the card index
        card: usize,
        /// the time after a card was used
        /// at which this card should appear
        duration: u32,
    },
}

impl CardCondition {
    pub const fn appear_immediately() -> Self {
        Self::Test { test: true }
    }

    /// appear approximately N milliseconds
    /// after the player uses a card
    pub const fn after_card_millis(card_index: usize, millis: u32) -> Self {
        Self::TimeAfterCard {
            card: card_index,
            duration: millis / TIME_UNITS_PER_MILLISECOND,
        }
    }

    /// Returns true if the card should be visible
    /// (unless it has already been used).
    pub fn should_appear(&self, state: &WorldState) -> bool {
        match self {
            Self::Test { test } => *test,
            Self::Funds(money) => state.funds >= *money,
            Self::Spent(money) => state.spent < *money,
            Self::TotalBaseOps(ops) => state.base_service.total >= *ops,
            Self::AvailableBaseOps(ops) => state.base_service.available >= *ops,
            Self::TotalSuperOps(ops) => state.super_service.total >= *ops,
            Self::AvailableSuperOps(ops) => state.super_service.available >= *ops,
            Self::TotalEpicOps(ops) => state.epic_service.total >= *ops,
            Self::AvailableEpicOps(ops) => state.epic_service.available >= *ops,
            Self::TotalAwesomeOps(ops) => state.awesome_service.total >= *ops,
            Self::AvailableAwesomeOps(ops) => state.awesome_service.available >= *ops,
            Self::TimeAfterCard { card, duration } => {
                match state
                    .cards_used
                    .binary_search_by_key(&card, |used_card| &used_card.index)
                {
                    Err(_) => false,
                    Ok(index) => {
                        let used_card = &state.cards_used[index];
                        used_card.time + *duration as u64 <= state.time
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CardEffect {
    /// Make the service public,
    /// so that it can be used by customers
    PublishService(ServiceKind),
    /// Make the service visible,
    /// (the base service starts unlocked)
    UnlockService(ServiceKind),
    /// Add or remove funds
    AddFunds(Money),
    /// Add cloud clients with the given specification
    AddClients(CloudClientSpec),
    /// Add cloud clients with the given specification,
    /// plus increase service demand by the given percentage
    AddClientsWithPublicity(CloudClientSpec, f32),
    /// Upgrade software services to the next level
    UpgradeServices,
}
