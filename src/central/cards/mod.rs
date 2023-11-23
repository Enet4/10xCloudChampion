use crate::{
    CloudClientSpec, Cost, Money, Ops, ServiceKind, WorldState, TIME_UNITS_PER_MILLISECOND,
};

use super::engine::CPU_LEVELS;

pub mod all;

/// The specification for a card,
/// including in what circumstances it should become available.
#[derive(Debug)]
pub struct CardSpec {
    /// the unique identifier as a small static string
    pub id: &'static str,
    /// the card's title
    pub title: &'static str,
    /// a short description of the card
    pub description: &'static str,
    /// the cost of the card,
    /// including which operations are needed
    pub cost: Cost,
    /// the primary condition for the card to appear
    /// in the project's panel
    /// (the secondary condition is that
    /// the operation service kinds in `cost` are unlocked)
    pub condition: CardCondition,
    /// the effect of the card once used
    pub effect: CardEffect,
}

impl CardSpec {
    /// Returns true if the card should be visible
    /// according to the given world state.
    pub fn should_appear(&self, state: &WorldState) -> bool {
        // should not be a used card
        !state.is_card_used(self.id)
        // condition of appearance is fulfilled
            && self.condition.should_appear(&state)
        // check if the player has unlocked the service kinds
            && self.has_services_unlocked(&state)
        // and should not be a test card
            && !self.id.starts_with("test")
    }

    fn has_services_unlocked(&self, state: &WorldState) -> bool {
        // super service must be unlocked if it costs super ops
        (self.cost.super_ops == Ops(0) || state.super_service.unlocked)
        // epic service must be unlocked if it costs epic ops
            && (self.cost.epic_ops == Ops(0) || state.epic_service.unlocked)
        // awesome service must be unlocked if it costs awesome ops
            && (self.cost.awesome_ops == Ops(0) || state.awesome_service.unlocked)
    }
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
    /// the player has earned this much money
    Earned(Money),
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
    /// at least N requests have been dropped
    RequestsDropped(u32),
    /// the player received their first electricity bill
    FirstBillArrived,
    /// appear N ticks after another card has been used
    TimeAfterCard {
        /// the card index
        card: &'static str,
        /// the time after a card was used
        /// at which this card should appear
        duration: u32,
    },
    /// the first node has been upgraded to maximum CPU
    FullyUpgradedNode,
    /// the first rack has been fully upgraded
    FullyUpgradedRack,
    /// the first data center has been fully upgraded
    FullyUpgradedDatacenter,
}

impl CardCondition {
    pub const fn appear_immediately() -> Self {
        Self::Test { test: true }
    }

    /// appear approximately N milliseconds
    /// after the player uses a card
    pub const fn after_card_millis(card_id: &'static str, millis: u32) -> Self {
        Self::TimeAfterCard {
            card: card_id,
            duration: millis / TIME_UNITS_PER_MILLISECOND,
        }
    }

    /// Returns true if the card condition is true for the given world state.
    ///
    /// Does not check whether the card has been used
    /// nor whether the service kinds in the cost are unlocked.
    pub fn should_appear(&self, state: &WorldState) -> bool {
        // then check the primary condition
        match self {
            Self::Test { test } => *test,
            Self::Funds(money) => state.funds >= *money,
            Self::Spent(money) => state.spent < *money,
            Self::Earned(money) => state.earned >= *money,
            Self::TotalBaseOps(ops) => state.base_service.total >= *ops,
            Self::AvailableBaseOps(ops) => state.base_service.available >= *ops,
            Self::TotalSuperOps(ops) => state.super_service.total >= *ops,
            Self::AvailableSuperOps(ops) => state.super_service.available >= *ops,
            Self::TotalEpicOps(ops) => state.epic_service.total >= *ops,
            Self::AvailableEpicOps(ops) => state.epic_service.available >= *ops,
            Self::TotalAwesomeOps(ops) => state.awesome_service.total >= *ops,
            Self::AvailableAwesomeOps(ops) => state.awesome_service.available >= *ops,
            Self::RequestsDropped(count) => state.requests_dropped >= *count as u64,
            Self::FirstBillArrived => state.electricity.last_bill_time > 0,
            Self::TimeAfterCard { card, duration } => {
                match state
                    .cards_used
                    .binary_search_by(|used_card| used_card.id.as_ref().cmp(*card))
                {
                    Err(_) => false,
                    Ok(index) => {
                        let used_card = &state.cards_used[index];
                        used_card.time + *duration as u64 <= state.time
                    }
                }
            }
            Self::FullyUpgradedNode => state.nodes[0].cpu_level == (CPU_LEVELS.len() - 1) as u8,
            Self::FullyUpgradedRack => {
                state.nodes.len() == 4 && state.nodes[1].cpu_level == (CPU_LEVELS.len() - 1) as u8
            }
            Self::FullyUpgradedDatacenter => false, // TODO
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CardEffect {
    /// Might be useful
    Nothing,
    /// Make the service public,
    /// so that it can be used by customers
    PublishService(ServiceKind),
    /// Make the service visible to the player,
    /// staying private to customers.
    ///
    /// The base service does not need unlocking.
    UnlockService(ServiceKind),
    /// Add or remove funds
    AddFunds(Money),
    /// Change how much extra money you earn per op
    /// (regardless of who issued it).
    UpgradeEntitlements(ServiceKind, Money),
    /// Add cloud clients with the given specification
    AddClients(CloudClientSpec),
    /// Add cloud clients with the given specification,
    /// plus increase general service demand by the given percentage
    AddClientsWithPublicity(CloudClientSpec, f32),
    /// Increase general service demand by the given amount
    AddPublicity(f32),
    /// Increase the number of operations per player click
    UpgradeOpsPerClick(u32),
    /// Set the electricity bill level (higher levels mean cheaper electricity)
    SetElectricityCostLevel(u8),
    /// Upgrade software services to the next level
    UpgradeServices,
    /// Upgrade the software caching level
    MoreCaching,
    /// Unlock node purchasing
    UnlockMultiNodes,
    /// Unlock rack purchasing
    UnlockMultiRacks,
    /// Unlock purchasing of new data center warehouses
    UnlockMultiDatacenters,
    /// Unlock demand estimate in business panel
    UnlockDemandEstimate,
}
