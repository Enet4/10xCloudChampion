//! All card entries encoded here
//!

use crate::{CloudUserSpec, Cost, Money, ServiceKind};

use super::{CardCondition, CardEffect, CardSpec};

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
        id: "c1",
        title: "First Customer",
        description: "Offer a trial period for your first customer",
        cost: Cost::nothing(),
        condition: CardCondition::TimeAfterCard {
            card: 0,
            duration: 50,
        },
        effect: CardEffect::AddClients(CloudUserSpec {
            amount: 1,
            service: ServiceKind::Base,
            trial_time: 100,
            bad: false,
        }),
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
];
