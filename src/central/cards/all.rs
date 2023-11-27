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
        condition: CardCondition::TotalBaseOps(Ops(1_500)),
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
            duration: 500_000,
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
    CardSpec {
        id: "b4",
        title: "Seamless monetary volition",
        description: "All awesome ops give you an extra $0.05",
        cost: Cost::awesome_ops(36_000).and(Cost::epic_ops(128_000)),
        condition: CardCondition::TotalAwesomeOps(Ops(9_777)),
        effect: CardEffect::UpgradeEntitlements(ServiceKind::Awesome, Money::cents(5)),
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
        title: "High-end predictive caching",
        description: "Improved cache greater throughput",
        cost: Cost::money(Money::dollars(2_000))
            .and(Cost::epic_ops(50_000))
            .and(Cost::super_ops(100_000)),
        condition: CardCondition::TotalMemoryUpgrades(40),
        effect: CardEffect::MoreCaching,
    },
    CardSpec {
        id: "c3",
        title: "Clairvoyant  caching",
        description: "Do caching like it knew almost everything in advance",
        cost: Cost::money(Money::dollars(500_000))
            .and(Cost::awesome_ops(60_000))
            .and(Cost::epic_ops(100_000)),
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
        effect: CardEffect::AddPublicityRate(24., 0.25),
    },
    CardSpec {
        id: "d2",
        title: "Fliers",
        description: "Good ol' paper ads around SV",
        cost: Cost::dollars(70).and(Cost::base_ops(900)),
        condition: CardCondition::Earned(Money::dollars(50)),
        effect: CardEffect::AddPublicityRate(48.0, 0.5),
    },
    CardSpec {
        id: "d3",
        title: "3 second video ad",
        description: "A sneak peek into your services",
        cost: Cost::dollars(290).and(Cost::super_ops(300)),
        condition: CardCondition::Earned(Money::dollars(200)),
        effect: CardEffect::AddPublicityRate(88.0, 1.),
    },
    CardSpec {
        id: "d3.5",
        title: "Conference talk",
        description: "Present your services to a savvy audience",
        condition: CardCondition::Earned(Money::dollars(1_200)),
        cost: Cost::dollars(750).and(Cost::super_ops(1_000)),
        effect: CardEffect::AddPublicityRate(250., 2.0),
    },
    CardSpec {
        id: "d4",
        title: "Capital city billboard ad",
        description: "Millions will see this board",
        cost: Cost::dollars(7_500).and(Cost::super_ops(3_000)),
        condition: CardCondition::Earned(Money::dollars(6_200)),
        effect: CardEffect::AddPublicityRate(600.0, 8.0),
    },
    CardSpec {
        id: "d4.5",
        title: "Blame caching",
        description: "Regain your clients' trust",
        cost: Cost::dollars(2_000),
        condition: CardCondition::RequestsDropped(500),
        effect: CardEffect::AddPublicityRate(64., 0.5),
    },
    CardSpec {
        id: "d5",
        title: "Cricket Championship ad",
        description: "Great services are advertized in great events",
        condition: CardCondition::Earned(Money::dollars(50_000)),
        cost: Cost::dollars(74_000).and(Cost::epic_ops(6_000)),
        effect: CardEffect::AddPublicityRate(1_999., 20.),
    },
    CardSpec {
        id: "d5.5",
        title: "SolVision Song Contest ad",
        description: "“These services are out of this world!”",
        condition: CardCondition::Earned(Money::dollars(265_000)),
        cost: Cost::dollars(300_000).and(Cost::epic_ops(48_000)),
        effect: CardEffect::AddPublicityRate(9_000., 48.),
    },
    CardSpec {
        id: "d6",
        title: "Strategic company purchase",
        description: "Make a deal with EWS, your biggest rival",
        condition: CardCondition::Earned(Money::dollars(10_000_000)),
        cost: Cost::dollars(16_940_000).and(Cost::epic_ops(250_000)),
        effect: CardEffect::AddPublicityRate(60_000.0, 75.),
    },
    CardSpec {
        id: "d7",
        title: "Hypnodrones",
        description: "Your ultimate brand ambassadors",
        condition: CardCondition::TotalAwesomeOps(Ops(20_000)),
        cost: Cost::dollars(50_000_000).and(Cost::awesome_ops(70_000)),
        effect: CardEffect::AddPublicityRate(300_000.0, 200.),
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
        condition: CardCondition::TotalBaseOps(Ops(100_000)),
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
        condition: CardCondition::TotalCloudNodes(6),
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
        title: "Free energy research",
        description: "Develop a groundbreaking source of free energy",
        cost: Cost::dollars(8_000_000).and(Cost::awesome_ops(1_000_000)),
        condition: CardCondition::TotalAwesomeOps(Ops(700_000)),
        effect: CardEffect::SetElectricityCostLevel(6),
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
        description: "Estimate the visibility of your services",
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
        effect: CardEffect::AddPublicityRate(20., 0.),
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
    // winning cards
    CardSpec {
        id: "win0",
        title: "So...",
        description: "How's your Cloud going?",
        cost: Cost::nothing(),
        condition: CardCondition::TotalAwesomeOps(Ops(1_000_000_000)),
        effect: CardEffect::Nothing,
    },
    CardSpec {
        id: "win1",
        title: "You did 1 billion awesome ops",
        description: "That's a great deal!",
        cost: Cost::nothing(),
        condition: CardCondition::TimeAfterCard {
            card: "win0",
            duration: 200_000,
        },
        effect: CardEffect::Nothing,
    },
    CardSpec {
        id: "win2",
        title: "But you must be tired",
        description: "And there is nothing else to offer here",
        cost: Cost::nothing(),
        condition: CardCondition::TimeAfterCard {
            card: "win1",
            duration: 160_000,
        },
        effect: CardEffect::Nothing,
    },
    CardSpec {
        id: "win3",
        title: "There is virtually no op limit",
        description: "And you are far from the 9 quintillion ops needed to break the game",
        cost: Cost::nothing(),
        condition: CardCondition::TimeAfterCard {
            card: "win2",
            duration: 100_000,
        },
        effect: CardEffect::Nothing,
    },
    CardSpec {
        id: "win4",
        title: "So I offer you a winning condition",
        description: "So you can go and live to your potential!",
        cost: Cost::nothing(),
        condition: CardCondition::TimeAfterCard {
            card: "win3",
            duration: 50_000,
        },
        effect: CardEffect::Nothing,
    },
    CardSpec {
        id: "win5",
        title: "Win the game",
        description: "It's a symbolic cost, really (and it's an awesome op because you're awesome)",
        cost: Cost::awesome_ops(1),
        condition: CardCondition::TimeAfterCard {
            card: "win4",
            duration: 20_000,
        },
        effect: CardEffect::Nothing,
    },
    CardSpec {
        id: "win6",
        title: "Congratulations!",
        description: "You are a true 10\u{00d7} Cloud Champion! 💪",
        cost: Cost::nothing(),
        condition: CardCondition::TimeAfterCard {
            card: "win5",
            duration: 5_000,
        },
        effect: CardEffect::Nothing,
    },
    CardSpec {
        id: "win7",
        title: "Thank you for playing 🙏",
        description: "Written by E_net4 for GitHub GameOff 2023",
        cost: Cost::nothing(),
        condition: CardCondition::TimeAfterCard {
            card: "win6",
            duration: 5_000,
        },
        effect: CardEffect::Nothing,
    },
    CardSpec {
        id: "win8",
        title: "It's over",
        description: "Bye bye now 👋",
        cost: Cost::nothing(),
        condition: CardCondition::TimeAfterCard {
            card: "win7",
            duration: 500_000,
        },
        effect: CardEffect::Nothing,
    },
    CardSpec {
        id: "win9",
        title: "What a waste of time",
        description: "Don't you have anything else to do?",
        cost: Cost {
            awesome_ops: Ops(0x7FFF_FFFF_FFFF_FFFF),
            epic_ops: Ops(0),
            super_ops: Ops(0),
            base_ops: Ops(0),
            money: Money::zero(),
        },
        condition: CardCondition::TimeAfterCard {
            card: "win8",
            duration: 800_000,
        },
        effect: CardEffect::Nothing,
    },
];

pub fn card_by_id(id: &str) -> Option<&'static CardSpec> {
    ALL_CARDS
        .binary_search_by(|c| c.id.cmp(id))
        .ok()
        .map(|idx| &ALL_CARDS[idx])
}
