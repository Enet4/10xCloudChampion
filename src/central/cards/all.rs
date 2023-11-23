//! All card entries encoded here
//!

use crate::{CloudClientSpec, Cost, Money, Ops, ServiceKind};

use super::{CardCondition, CardEffect, CardSpec};

/// All project cards in the game.
///
/// They _must_ be inserted in id ascending order.
pub static ALL_CARDS: &'static [CardSpec] = &[
    // --- service unlocking and publishing ---
    CardSpec {
        id: "a0p",
        title: "Test your service",
        description: "Always test before delivering to the public",
        cost: Cost::base_ops(8),
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
        id: "a1p",
        title: "Publish Super Ops",
        description: "Deliver Super Ops to the public",
        cost: Cost::super_ops(16),
        condition: CardCondition::TimeAfterCard {
            card: "a1",
            duration: 5_000,
        },
        effect: CardEffect::PublishService(ServiceKind::Super),
    },
    CardSpec {
        id: "a2",
        title: "Epic Ops",
        description: "State of the art Cloud services",
        cost: Cost::super_ops(2_000)
            .and(Cost::base_ops(6_000))
            .and(Cost::dollars(5_420)),
        condition: CardCondition::TotalSuperOps(Ops(1_000)),
        effect: CardEffect::UnlockService(ServiceKind::Epic),
    },
    CardSpec {
        id: "a2p",
        title: "Publish Epic Ops",
        description: "Deliver Epic Ops to the public",
        cost: Cost::epic_ops(32),
        condition: CardCondition::TimeAfterCard {
            card: "a1",
            duration: 22_000,
        },
        effect: CardEffect::PublishService(ServiceKind::Epic),
    },
    CardSpec {
        id: "a3",
        title: "Awesome Ops",
        description: "The Cloud services to rule them all",
        cost: Cost::epic_ops(5_000)
            .and(Cost::super_ops(10_000))
            .and(Cost::dollars(166_000)),
        condition: CardCondition::TotalEpicOps(Ops(2_800)),
        effect: CardEffect::UnlockService(ServiceKind::Awesome),
    },
    CardSpec {
        id: "a3p",
        title: "Publish Awesome Ops",
        description: "Deliver Awesome Ops to the public",
        cost: Cost::awesome_ops(64),
        condition: CardCondition::TimeAfterCard {
            card: "a1",
            duration: 100_000,
        },
        effect: CardEffect::PublishService(ServiceKind::Awesome),
    },
    // --- money bonuses and entitlements ---
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
    // --- caching cards ---
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
    // --- advertisement ---
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
        condition: CardCondition::Earned(Money::dollars(50_000)),
        cost: Cost::dollars(90_000).and(Cost::epic_ops(80)),
        effect: CardEffect::AddPublicity(220.0),
    },
    CardSpec {
        id: "d5",
        title: "Strategic company purchase",
        description: "Make a deal with EWS, your biggest rival",
        condition: CardCondition::Earned(Money::dollars(10_000_000)),
        cost: Cost::dollars(169_400_000).and(Cost::epic_ops(250)),
        effect: CardEffect::AddPublicity(4_000.0),
    },
    CardSpec {
        id: "d7",
        title: "Hypodrones",
        description: "Your ultimate brand ambassadors",
        condition: CardCondition::Earned(Money::dollars(1_000_000_000)),
        cost: Cost::dollars(5_000_000_000).and(Cost::awesome_ops(400)),
        effect: CardEffect::AddPublicity(100_000.0),
    },
    // --- energy cards ---
    CardSpec {
        id: "e0",
        title: "Renegotiate energy contract",
        description: "Get a better deal for your energy",
        cost: Cost::base_ops(170),
        condition: CardCondition::TotalBaseOps(Ops(100)),
        effect: CardEffect::SetElectricityCostLevel(1),
    },
    CardSpec {
        id: "e1",
        title: "Repair A/C system",
        description: "Increase energy efficiency",
        cost: Cost::dollars(100).and(Cost::base_ops(200)),
        condition: CardCondition::TotalBaseOps(Ops(2_000)),
        effect: CardEffect::SetElectricityCostLevel(2),
    },
    CardSpec {
        id: "e2",
        title: "Buy solar panels",
        description: "Generate some energy to reduce future costs",
        cost: Cost::dollars(400).and(Cost::super_ops(300)),
        condition: CardCondition::TotalSuperOps(Ops(800)),
        effect: CardEffect::SetElectricityCostLevel(3),
    },
    CardSpec {
        id: "e3",
        title: "Clean energy plan",
        description: "Commit to clean energy for the long term",
        cost: Cost::dollars(1_300).and(Cost::super_ops(700)),
        condition: CardCondition::TotalSuperOps(Ops(4_000)),
        effect: CardEffect::SetElectricityCostLevel(4),
    },
    CardSpec {
        id: "e4",
        title: "Dedicated Power Plant",
        description: "All systems powered by your own energy",
        cost: Cost::dollars(30_000).and(Cost::epic_ops(600)),
        condition: CardCondition::TotalEpicOps(Ops(1_000)),
        effect: CardEffect::SetElectricityCostLevel(5),
    },
    CardSpec {
        id: "e5",
        title: "Fusion reactor",
        description: "Discover a source of energy that is too good to be true",
        cost: Cost::dollars(120_000).and(Cost::epic_ops(4_000)),
        condition: CardCondition::TotalAwesomeOps(Ops(180)),
        effect: CardEffect::SetElectricityCostLevel(6),
    },
    CardSpec {
        id: "e6",
        title: "Free energy",
        description: "Develop a groundbreaking source of free energy",
        cost: Cost::dollars(4_000_000).and(Cost::awesome_ops(1_000)),
        condition: CardCondition::TotalAwesomeOps(Ops(100)),
        effect: CardEffect::SetElectricityCostLevel(7),
    },
    // --- hardware scaling cards ---
    CardSpec {
        id: "n1",
        title: "Central node routing",
        description: "Prepare the space for more nodes",
        condition: CardCondition::FullyUpgradedNode,
        cost: Cost::dollars(10).and(Cost::base_ops(400)),
        effect: CardEffect::UnlockMultiNodes,
    },
    CardSpec {
        id: "n2",
        title: "Improved routing",
        description: "Distribute routing costs to all nodes",
        condition: CardCondition::Test { test: false }, // TODO
        cost: Cost::dollars(100).and(Cost::super_ops(280)),
        effect: CardEffect::Nothing, // TODO
    },
    CardSpec {
        id: "n3",
        title: "Room for more servers",
        description: "Make space for more racks",
        condition: CardCondition::FullyUpgradedRack,
        cost: Cost::dollars(25).and(Cost::super_ops(600)),
        effect: CardEffect::Nothing, // TODO
    },
    // --- software upgrade cards ---
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
