//! All card entries encoded here
//!

use crate::{CloudClientSpec, Cost, Money, ServiceKind};

use super::{CardCondition, CardEffect, CardSpec};

/// All project cards in the game.
///
/// They _must_ be inserted in id ascending order.
pub static ALL_CARDS: &'static [CardSpec] = &[
    CardSpec {
        id: "a0",
        title: "Test your service",
        description: "Always test before delivering to the public",
        cost: Cost::base_ops(10),
        condition: CardCondition::appear_immediately(),
        effect: CardEffect::PublishService(ServiceKind::Base),
    },
    CardSpec {
        id: "a1",
        title: "Clean up trace logs",
        description: "Improve service performance by 5%",
        cost: Cost::money(Money::dollars(40)),
        condition: CardCondition::Funds(Money::dollars(25)),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "c0",
        title: "First Customer",
        description: "Offer a trial period for your first customer",
        cost: Cost::nothing(),
        condition: CardCondition::TimeAfterCard {
            card: "a0",
            duration: 80,
        },
        effect: CardEffect::AddClientsWithPublicity(
            CloudClientSpec {
                amount: 1,
                service: ServiceKind::Base,
                trial_duration: 1_000,
            },
            1.,
        ),
    },
    // test cards
    CardSpec {
        id: "test-0",
        title: "New card",
        description: "A test card to give you a welcoming bonus",
        cost: Cost::nothing(),
        condition: CardCondition::appear_immediately(),
        effect: CardEffect::AddFunds(Money::dollars(200)),
    },
    CardSpec {
        id: "test-1",
        title: "Powerup",
        description: "Test improving your services",
        cost: Cost::base_ops(500),
        condition: CardCondition::appear_immediately(),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "test-2",
        title: "YouTube ads",
        description: "Test adding advertisements",
        cost: Cost::super_ops(100),
        condition: CardCondition::appear_immediately(),
        effect: CardEffect::AddPublicity(20.),
    },
    CardSpec {
        id: "test-3",
        title: "Unreachable",
        description: "This one is too expensive",
        cost: Cost::super_ops(500_000),
        condition: CardCondition::appear_immediately(),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "test-4",
        title: "Wat",
        description: "This one should not appear",
        cost: Cost::nothing(),
        condition: CardCondition::Test { test: false },
        effect: CardEffect::Nothing,
    },
];

pub fn card_by_id(id: &str) -> Option<&'static CardSpec> {
    ALL_CARDS
        .binary_search_by(|c| c.id.cmp(id))
        .ok()
        .map(|idx| &ALL_CARDS[idx])
}
