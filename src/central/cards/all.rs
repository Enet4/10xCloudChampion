//! All card entries encoded here
//!

use crate::{CloudClientSpec, Cost, Money, Ops, ServiceKind};

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
        description: "Improve service performance a small bit",
        cost: Cost::money(Money::dollars(30)),
        condition: CardCondition::Funds(Money::dollars(20)),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "a2",
        title: "Profile-guided optimization",
        description: "Improve service performance",
        cost: Cost::money(Money::dollars(75)).and(Cost::base_ops(100)),
        condition: CardCondition::TotalBaseOps(Ops(1_000)),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "a3",
        title: "Algorithmic revision",
        description: "Improve service performance",
        cost: Cost::money(Money::dollars(200)).and(Cost::super_ops(200)),
        condition: CardCondition::TotalSuperOps(Ops(1_200)),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "a4",
        title: "Rewrite in Rust",
        description: "Improve service performance",
        cost: Cost::money(Money::dollars(600)).and(Cost::epic_ops(200)),
        condition: CardCondition::TotalSuperOps(Ops(1_200)),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "c0",
        title: "Implement caching",
        description: "Use available memory to make your service faster",
        cost: Cost::money(Money::dollars(100)).and(Cost::base_ops(75)),
        condition: CardCondition::TotalBaseOps(Ops(500)),
        effect: CardEffect::MoreCaching,
    },
    CardSpec {
        id: "c1",
        title: "More caching",
        description: "Use more memory to make your service even faster",
        cost: Cost::money(Money::dollars(100)).and(Cost::base_ops(75)),
        condition: CardCondition::TotalSuperOps(Ops(600)),
        effect: CardEffect::MoreCaching,
    },
    CardSpec {
        id: "d0",
        title: "Let someone try",
        description: "Offer a trial period for your first customer",
        cost: Cost::nothing(),
        condition: CardCondition::TimeAfterCard {
            card: "a0",
            duration: 5_000,
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
    CardSpec {
        id: "d1",
        title: "Fliers",
        description: "Good ol' paper ads",
        cost: Cost::dollars(300),
        condition: CardCondition::Earned(Money::dollars(100)),
        effect: CardEffect::AddPublicity(2.5),
    },
    CardSpec {
        id: "2.5",
        title: "Blame caching",
        description: "Regain your clients' trust",
        cost: Cost::dollars(2_000),
        condition: CardCondition::TimeAfterCard {
            card: "d1",
            duration: 20_000,
        },
        effect: CardEffect::AddPublicity(5.),
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
