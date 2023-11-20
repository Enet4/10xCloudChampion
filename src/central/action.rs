//! Module for containing all player actions and their consequences.

use std::borrow::Cow;

use crate::{Money, ServiceKind};

/// An action that a player can take that affects the game state.
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerAction {
    /// Perform a cloud service operation
    /// by request of the player.
    OpClick {
        /// the kind of service that was clicked
        kind: ServiceKind,
        /// the number of operations to perform
        amount: u32,
    },

    /// Sub-action that deducts money from the player's balance.
    Payment { amount: Money },

    /// Pay the electricity bill.
    PayElectricityBill,

    /// Change the price of a cloud service.
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
        id: Cow<'static, str>,
    },
}
