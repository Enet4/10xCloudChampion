//! Module for containing all player actions and their consequences.

use crate::{Cost, Money, ServiceKind};

/// An action that a player can take that affects the game state.
#[derive(Debug, Clone, PartialEq)]
pub enum UserAction {
    /// Sub-action that deducts certain points
    /// from the player's available ops or funds.
    ApplyCost { cost: Cost },

    /// Sub-action that deducts money from the player's balance.
    Payment { amount: Money },

    /// change the price of a cloud service
    ChangePrice { kind: ServiceKind, new_price: Money },

    /// Upgrade a node's CPU
    UpgradeCpu { node: u32 },

    /// Upgrade a node's RAM
    UpgradeRam { node: u32 },

    /// Acquire a new cloud node
    AddNode,

    /// Use a card by applying its effect.
    ///
    /// Knowing the effects of the card requires
    /// looking it up on the list of all cards.
    UseCard {
        /// the card's identifier
        id: usize,
    },
}
