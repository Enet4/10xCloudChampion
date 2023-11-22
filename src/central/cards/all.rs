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
        title: "Super Ops",
        description: "Next generation Cloud services",
        cost: Cost::base_ops(1_000).and(Cost::dollars(250)),
        condition: CardCondition::TotalBaseOps(Ops(600)),
        effect: CardEffect::UnlockService(ServiceKind::Super),
    },
    CardSpec {
        id: "a2",
        title: "Epic Ops",
        description: "State of the art Cloud services",
        cost: Cost::super_ops(1_500)
            .and(Cost::base_ops(2_500))
            .and(Cost::dollars(1_000)),
        condition: CardCondition::TotalSuperOps(Ops(1_000)),
        effect: CardEffect::UnlockService(ServiceKind::Super),
    },
    CardSpec {
        id: "a3",
        title: "Awesome Ops",
        description: "The Cloud services everyone will want",
        cost: Cost::epic_ops(2_800)
            .and(Cost::super_ops(3_000))
            .and(Cost::dollars(10_000)),
        condition: CardCondition::TotalEpicOps(Ops(2_200)),
        effect: CardEffect::UnlockService(ServiceKind::Super),
    },
    CardSpec {
        id: "b0",
        title: "Extra bonus from your family",
        description: "Grandpa believes in you",
        cost: Cost::base_ops(100),
        condition: CardCondition::TotalBaseOps(Ops(100)),
        effect: CardEffect::AddFunds(Money::dollars(100)),
    },
    CardSpec {
        id: "b1",
        title: "College fund initiative",
        description: "All base ops give you an extra $0.00002",
        cost: Cost::base_ops(720),
        condition: CardCondition::TotalBaseOps(Ops(400)),
        effect: CardEffect::UpgradeEntitlements(ServiceKind::Base, Money::millicents(2)),
    },
    CardSpec {
        id: "b2",
        title: "Government funded project",
        description: "All super ops give you an extra $0.0005",
        cost: Cost::super_ops(1_000),
        condition: CardCondition::TotalSuperOps(Ops(600)),
        effect: CardEffect::UpgradeEntitlements(ServiceKind::Super, Money::millicents(50)),
    },
    CardSpec {
        id: "b3",
        title: "United Nations funding",
        description: "All epic ops give you an extra $0.001",
        cost: Cost::epic_ops(1_400).and(Cost::super_ops(1_800)),
        condition: CardCondition::TotalEpicOps(Ops(800)),
        effect: CardEffect::UpgradeEntitlements(ServiceKind::Epic, Money::dec_cents(1)),
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
        cost: Cost::money(Money::dollars(250)).and(Cost::super_ops(250)),
        condition: CardCondition::TotalSuperOps(Ops(800)),
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
        cost: Cost::dollars(300).and(Cost::base_ops(25)),
        condition: CardCondition::Earned(Money::dollars(100)),
        effect: CardEffect::AddPublicity(10.0),
    },
    CardSpec {
        id: "d2",
        title: "3 second video ad",
        description: "A sneak peek into your services",
        cost: Cost::dollars(2_500).and(Cost::super_ops(50)),
        condition: CardCondition::Earned(Money::dollars(1_000)),
        effect: CardEffect::AddPublicity(30.0),
    },
    CardSpec {
        id: "d3",
        title: "Capital city billboard ad",
        description: "Millions will see this board",
        cost: Cost::dollars(7_000).and(Cost::super_ops(100)),
        condition: CardCondition::Earned(Money::dollars(5_000)),
        effect: CardEffect::AddPublicity(220.0),
    },
    CardSpec {
        id: "d3.5",
        title: "Blame caching",
        description: "Regain your clients' trust",
        cost: Cost::dollars(2_500),
        condition: CardCondition::TimeAfterCard {
            card: "c1",
            duration: 80_000,
        },
        effect: CardEffect::AddPublicity(25.),
    },
    CardSpec {
        id: "d4",
        title: "Cricket World Cup ad",
        description: "Great services should be publicized in great events",
        cost: Cost::dollars(90_000).and(Cost::epic_ops(80)),
        condition: CardCondition::Earned(Money::dollars(50_000)),
        effect: CardEffect::AddPublicity(220.0),
    },
    CardSpec {
        id: "d5",
        title: "Strategic company purchase",
        description: "Make a deal with EWS, your biggest rival",
        cost: Cost::dollars(169_400_000).and(Cost::epic_ops(250)),
        condition: CardCondition::Earned(Money::dollars(10_000_000)),
        effect: CardEffect::AddPublicity(4_000.0),
    },
    CardSpec {
        id: "d7",
        title: "Hypodrones",
        description: "Your ultimate brand ambassadors",
        cost: Cost::dollars(5_000_000_000).and(Cost::awesome_ops(400)),
        condition: CardCondition::Earned(Money::dollars(1_000_000_000)),
        effect: CardEffect::AddPublicity(100_000.0),
    },
    CardSpec {
        id: "s1",
        title: "Clean up trace logs",
        description: "Improve service performance a small bit",
        cost: Cost::money(Money::dollars(30)),
        condition: CardCondition::Funds(Money::dollars(20)),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "s2",
        title: "Profile-guided optimization",
        description: "Improve service performance",
        cost: Cost::money(Money::dollars(40)).and(Cost::base_ops(250)),
        condition: CardCondition::TotalBaseOps(Ops(1_000)),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "s3",
        title: "Peer reviewed algorithmic revision",
        description: "Improve service performance",
        cost: Cost::money(Money::dollars(200)).and(Cost::super_ops(300)),
        condition: CardCondition::TotalBaseOps(Ops(3_000)),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "s4",
        title: "Rewrite in Rust",
        description: "Improve service performance",
        cost: Cost::money(Money::dollars(600)).and(Cost::epic_ops(200)),
        condition: CardCondition::TotalSuperOps(Ops(5_000)),
        effect: CardEffect::UpgradeServices,
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
