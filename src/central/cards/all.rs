//! All card entries encoded here
//!

use crate::{
    central::{engine::DEMAND_DOS_THRESHOLD, state::RoutingLevel},
    CloudClientSpec, Cost, Money, Ops, ServiceKind,
};

use super::{CardCondition, CardEffect, CardSpec};

// declare constants for some card IDs to help prevent mistakes
const ID_BASE_OPS_PUBLISHED: &str = "a0p";
const ID_SUPER_OPS_UNLOCKED: &str = "a1";
const ID_EPIC_OPS_UNLOCKED: &str = "a2";
const ID_AWESOME_OPS_UNLOCKED: &str = "a3";
const ID_MORE_CACHING: &str = "c1";

/// All project cards in the game.
///
/// They _must_ be inserted in id ascending order.
pub static ALL_CARDS: &'static [CardSpec] = &[
    // --- service unlocking and publishing ---
    CardSpec {
        id: ID_BASE_OPS_PUBLISHED,
        title: "Test your service",
        description: "Always test before delivering to the public",
        cost: Cost::base_ops(8),
        condition: CardCondition::appear_immediately(),
        effect: CardEffect::PublishService(ServiceKind::Base),
    },
    CardSpec {
        id: ID_SUPER_OPS_UNLOCKED,
        title: "Super Ops",
        description: "Next generation Cloud services",
        cost: Cost::base_ops(4_000).and(Cost::dollars(200)),
        condition: CardCondition::TotalBaseOps(Ops(1_000)),
        effect: CardEffect::UnlockService(ServiceKind::Super),
    },
    CardSpec {
        id: "a1p",
        title: "Publish Super Ops",
        description: "Deliver Super Ops to the public",
        cost: Cost::super_ops(16),
        condition: CardCondition::TimeAfterCard {
            card: ID_SUPER_OPS_UNLOCKED,
            duration: 6_000,
        },
        effect: CardEffect::PublishService(ServiceKind::Super),
    },
    CardSpec {
        id: ID_EPIC_OPS_UNLOCKED,
        title: "Epic Ops",
        description: "State of the art Cloud services",
        cost: Cost::super_ops(20_000)
            .and(Cost::base_ops(150_000))
            .and(Cost::dollars(5_420)),
        condition: CardCondition::TotalSuperOps(Ops(6_000)),
        effect: CardEffect::UnlockService(ServiceKind::Epic),
    },
    CardSpec {
        id: "a2p",
        title: "Publish Epic Ops",
        description: "Deliver Epic Ops to the public",
        cost: Cost::epic_ops(32),
        condition: CardCondition::TimeAfterCard {
            card: ID_EPIC_OPS_UNLOCKED,
            duration: 50_000,
        },
        effect: CardEffect::PublishService(ServiceKind::Epic),
    },
    CardSpec {
        id: ID_AWESOME_OPS_UNLOCKED,
        title: "Awesome Ops",
        description: "The Cloud services to rule them all",
        cost: Cost::epic_ops(700_000)
            .and(Cost::super_ops(5_000_000))
            .and(Cost::dollars(166_000)),
        condition: CardCondition::TotalEpicOps(Ops(428_900)),
        effect: CardEffect::UnlockService(ServiceKind::Awesome),
    },
    CardSpec {
        id: "a3p",
        title: "Publish Awesome Ops",
        description: "Deliver Awesome Ops to the public",
        cost: Cost::awesome_ops(64),
        condition: CardCondition::TimeAfterCard {
            card: ID_AWESOME_OPS_UNLOCKED,
            duration: 200_000,
        },
        effect: CardEffect::PublishService(ServiceKind::Awesome),
    },
    // --- money bonuses and entitlements ---
    CardSpec {
        id: "b0",
        title: "Incentive from your family",
        description: "Father believes in you",
        cost: Cost::base_ops(50),
        condition: CardCondition::AvailableBaseOps(Ops(100)),
        effect: CardEffect::AddFunds(Money::dollars(60)),
    },
    CardSpec {
        id: "b00",
        title: "Extra bonus from your family",
        description: "Grandpa believes in you",
        cost: Cost::base_ops(500),
        condition: CardCondition::AvailableBaseOps(Ops(1_000)),
        effect: CardEffect::AddFunds(Money::dollars(500)),
    },
    CardSpec {
        id: "b000",
        title: "Donation from cousin V",
        description: "Your cool rich cousin believes in you",
        cost: Cost::super_ops(1_024),
        condition: CardCondition::AvailableSuperOps(Ops(2_048)),
        effect: CardEffect::AddFunds(Money::dollars(10_000)),
    },
    CardSpec {
        id: "b1",
        title: "College fund initiative",
        description: "All base ops give you an extra $0.00005",
        cost: Cost::base_ops(720),
        condition: CardCondition::TotalBaseOps(Ops(500)),
        effect: CardEffect::UpgradeEntitlements(ServiceKind::Base, Money::millicents(5)),
    },
    CardSpec {
        id: "b2",
        title: "Government funded project",
        description: "All super ops give you an extra $0.0005",
        cost: Cost::super_ops(2_990),
        condition: CardCondition::TotalSuperOps(Ops(1_500)),
        effect: CardEffect::UpgradeEntitlements(ServiceKind::Super, Money::millicents(50)),
    },
    CardSpec {
        id: "b3",
        title: "United Nations funding",
        description: "All epic ops give you an extra $0.005",
        cost: Cost::epic_ops(12_800).and(Cost::super_ops(12_800)),
        condition: CardCondition::TotalEpicOps(Ops(2_000)),
        effect: CardEffect::UpgradeEntitlements(ServiceKind::Epic, Money::dec_cents(5)),
    },
    // --- caching cards ---
    CardSpec {
        id: "c0",
        title: "Implement caching",
        description: "Use available memory to make your service faster",
        cost: Cost::money(Money::dollars(100)).and(Cost::base_ops(260)),
        condition: CardCondition::TotalMemoryUpgrades(1),
        effect: CardEffect::MoreCaching,
    },
    CardSpec {
        id: ID_MORE_CACHING,
        title: "More caching",
        description: "Use more memory to make your service even faster",
        cost: Cost::money(Money::dollars(400)).and(Cost::super_ops(250)),
        condition: CardCondition::TotalMemoryUpgrades(4),
        effect: CardEffect::MoreCaching,
    },
    CardSpec {
        id: "c2",
        title: "Even more caching",
        description: "Use a lot more memory for greater throughput",
        cost: Cost::money(Money::dollars(2_500)).and(Cost::super_ops(250_000)),
        condition: CardCondition::TotalMemoryUpgrades(40),
        effect: CardEffect::MoreCaching,
    },
    // --- advertisement ---
    CardSpec {
        id: "d0",
        title: "Let someone try",
        description: "Offer a trial period for your first customer",
        cost: Cost::nothing(),
        condition: CardCondition::TimeAfterCard {
            card: ID_BASE_OPS_PUBLISHED,
            duration: 20_000,
        },
        effect: CardEffect::AddClientsWithPublicity(
            CloudClientSpec {
                service: ServiceKind::Base,
                trial_duration: 50_000,
            },
            2.,
        ),
    },
    CardSpec {
        id: "d1",
        title: "Optimize SEO",
        description: "Improves your ranking on search engines",
        cost: Cost::dollars(5).and(Cost::base_ops(850)),
        condition: CardCondition::AvailableBaseOps(Ops(500)),
        effect: CardEffect::AddPublicity(24.0),
    },
    CardSpec {
        id: "d2",
        title: "Fliers",
        description: "Good ol' paper ads around SV",
        cost: Cost::dollars(70).and(Cost::base_ops(900)),
        condition: CardCondition::Earned(Money::dollars(50)),
        effect: CardEffect::AddPublicity(48.0),
    },
    CardSpec {
        id: "d3",
        title: "3 second video ad",
        description: "A sneak peek into your services",
        cost: Cost::dollars(290).and(Cost::super_ops(300)),
        condition: CardCondition::Earned(Money::dollars(200)),
        effect: CardEffect::AddPublicity(88.0),
    },
    CardSpec {
        id: "d3.5",
        title: "Conference talk",
        description: "Present your services to a savvy audience",
        condition: CardCondition::Earned(Money::dollars(1_200)),
        cost: Cost::dollars(750).and(Cost::super_ops(1_000)),
        effect: CardEffect::AddPublicity(200.),
    },
    CardSpec {
        id: "d4",
        title: "Capital city billboard ad",
        description: "Millions will see this board",
        cost: Cost::dollars(7_500).and(Cost::super_ops(3_000)),
        condition: CardCondition::Earned(Money::dollars(6_200)),
        effect: CardEffect::AddPublicity(580.0),
    },
    CardSpec {
        id: "d4.5",
        title: "Blame caching",
        description: "Regain your clients' trust",
        cost: Cost::dollars(2_000),
        condition: CardCondition::TimeAfterCard {
            card: ID_MORE_CACHING,
            duration: 350_000,
        },
        effect: CardEffect::AddPublicity(64.),
    },
    CardSpec {
        id: "d5",
        title: "Cricket World Cup ad",
        description: "Great services are publicized in great events",
        condition: CardCondition::Earned(Money::dollars(50_000)),
        cost: Cost::dollars(74_000).and(Cost::epic_ops(6_000)),
        effect: CardEffect::AddPublicity(1_999.0),
    },
    CardSpec {
        id: "d6",
        title: "Strategic company purchase",
        description: "Make a deal with EWS, your biggest rival",
        condition: CardCondition::Earned(Money::dollars(10_000_000)),
        cost: Cost::dollars(169_400_000).and(Cost::epic_ops(50_000)),
        effect: CardEffect::AddPublicity(12_000.0),
    },
    CardSpec {
        id: "d7",
        title: "Hypodrones",
        description: "Your ultimate brand ambassadors",
        condition: CardCondition::TotalAwesomeOps(Ops(10_000)),
        cost: Cost::dollars(5_000_000_000).and(Cost::awesome_ops(70_000)),
        effect: CardEffect::AddPublicity(250_000.0),
    },
    // --- energy cards ---
    CardSpec {
        id: "e0",
        title: "Renegotiate energy contract",
        description: "Get a better deal for your energy",
        cost: Cost::base_ops(170),
        condition: CardCondition::FirstBillArrived,
        effect: CardEffect::SetElectricityCostLevel(1),
    },
    CardSpec {
        id: "e1",
        title: "Repair A/C system",
        description: "Increase energy efficiency",
        cost: Cost::dollars(180).and(Cost::base_ops(400)),
        condition: CardCondition::TotalBaseOps(Ops(200_000)),
        effect: CardEffect::SetElectricityCostLevel(2),
    },
    CardSpec {
        id: "e2",
        title: "Buy solar panels",
        description: "Generate some energy to reduce future costs",
        cost: Cost::dollars(520).and(Cost::super_ops(80_000)),
        condition: CardCondition::TotalCloudNodes(2),
        effect: CardEffect::SetElectricityCostLevel(3),
    },
    CardSpec {
        id: "e3",
        title: "Clean energy plan",
        description: "Commit to clean energy for the long term",
        cost: Cost::dollars(8_800).and(Cost::super_ops(1_000_000)),
        condition: CardCondition::TotalCloudNodes(4),
        effect: CardEffect::SetElectricityCostLevel(4),
    },
    CardSpec {
        id: "e4",
        title: "Dedicated Power Plant",
        description: "All systems powered by your own energy",
        cost: Cost::dollars(280_000).and(Cost::epic_ops(222_000)),
        condition: CardCondition::TotalCloudNodes(13),
        effect: CardEffect::SetElectricityCostLevel(5),
    },
    CardSpec {
        id: "e5",
        title: "Fusion reactor research",
        description: "Discover a source of energy that is too good to be true",
        cost: Cost::dollars(689_000).and(Cost::epic_ops(24_000_000)),
        condition: CardCondition::TotalCloudNodes(42),
        effect: CardEffect::SetElectricityCostLevel(6),
    },
    CardSpec {
        id: "e6",
        title: "Free energy",
        description: "Develop a groundbreaking source of free energy",
        cost: Cost::dollars(10_000_000).and(Cost::awesome_ops(1_000_000)),
        condition: CardCondition::TotalAwesomeOps(Ops(700_000)),
        effect: CardEffect::SetElectricityCostLevel(7),
    },
    // --- bad request protection cards ---
    CardSpec {
        id: "f0",
        title: "Request anomaly monitoring",
        description: "Detect obvious cases of malicious requests",
        cost: Cost::base_ops(200).and(Cost::super_ops(200)),
        condition: CardCondition::Demand(DEMAND_DOS_THRESHOLD + 0.25),
        effect: CardEffect::UpgradeSpamProtection(0.5),
    },
    CardSpec {
        id: "f1",
        title: "Adversarial generative spam network detection",
        description: "Detect more cases of DoS attacks",
        cost: Cost::super_ops(4_000).and(Cost::epic_ops(2_000)),
        condition: CardCondition::RequestsFailed(20_000),
        effect: CardEffect::UpgradeSpamProtection(0.875),
    },
    CardSpec {
        id: "f2",
        title: "Universal introspective malice correction",
        description: "Eliminate bad requests",
        cost: Cost::epic_ops(40_000).and(Cost::awesome_ops(20_000)),
        condition: CardCondition::RequestsFailed(1_000_000),
        effect: CardEffect::UpgradeSpamProtection(1.),
    },
    // --- informative cards ---
    CardSpec {
        id: "i0",
        title: "Market introspection",
        description: "Estimate the demand for your service",
        cost: Cost::base_ops(500),
        condition: CardCondition::TotalBaseOps(Ops(200)),
        effect: CardEffect::UnlockDemandEstimate,
    },
    // --- hardware scaling cards ---
    CardSpec {
        id: "n1",
        title: "Central node routing",
        description: "Prepare the space for more nodes",
        condition: CardCondition::FullyUpgradedNode,
        cost: Cost::dollars(150).and(Cost::base_ops(1_000)),
        effect: CardEffect::UnlockMultiNodes,
    },
    CardSpec {
        id: "n2",
        title: "Improved routing",
        description: "Distribute routing costs to all nodes",
        condition: CardCondition::TotalCloudNodes(3),
        cost: Cost::dollars(100).and(Cost::super_ops(800)),
        effect: CardEffect::UpgradeRoutingLevel(RoutingLevel::Distributed),
    },
    CardSpec {
        id: "n3",
        title: "Room for more servers",
        description: "Make space for more racks",
        condition: CardCondition::FullyUpgradedRack,
        cost: Cost::dollars(250).and(Cost::super_ops(6_000)),
        effect: CardEffect::UnlockMultiRacks,
    },
    CardSpec {
        id: "n5",
        title: "Geographical expansion",
        description: "Make reservations for more data centers",
        condition: CardCondition::FullyUpgradedDatacenter,
        cost: Cost::dollars(1_000).and(Cost::super_ops(30_000)),
        effect: CardEffect::UnlockMultiDatacenters,
    },
    CardSpec {
        id: "n6",
        title: "Spectral bandwidth 55G routing",
        description: "Eliminate all routing costs",
        condition: CardCondition::TotalCloudNodes(36),
        cost: Cost::dollars(10_000).and(Cost::awesome_ops(8_000)),
        effect: CardEffect::UpgradeRoutingLevel(RoutingLevel::NoRoutingCost),
    },
    // --- software upgrade cards ---
    CardSpec {
        id: "s1",
        title: "Clean up trace logs",
        description: "Improve service performance a small bit",
        cost: Cost::money(Money::dollars(5)).and(Cost::base_ops(64)),
        condition: CardCondition::Funds(Money::dollars(20)),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "s2",
        title: "Profile-guided optimization",
        description: "Improve service performance",
        cost: Cost::money(Money::dollars(40)).and(Cost::base_ops(750)),
        condition: CardCondition::TotalBaseOps(Ops(2_000)),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "s3",
        title: "Peer reviewed algorithmic revision",
        description: "Improve service performance",
        cost: Cost::money(Money::dollars(220)).and(Cost::super_ops(500)),
        condition: CardCondition::TotalSuperOps(Ops(2_000)),
        effect: CardEffect::UpgradeServices,
    },
    CardSpec {
        id: "s4",
        title: "Rewrite in Rust",
        description: "Improve service performance",
        cost: Cost::money(Money::dollars(980)).and(Cost::epic_ops(4_000)),
        condition: CardCondition::TotalEpicOps(Ops(4_000)),
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
