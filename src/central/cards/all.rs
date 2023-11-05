//! All card entries encoded here
//!

use crate::{CloudClientSpec, Cost, ServiceKind};

use super::{CardCondition, CardEffect, CardSpec};

pub static ALL_CARDS: &'static [CardSpec] = &[
    CardSpec {
        title: "Test your service",
        description: "Always test before delivery",
        cost: Cost::base_ops(10),
        condition: CardCondition::appear_immediately(),
        effect: CardEffect::PublishService(ServiceKind::Base),
    },
    CardSpec {
        title: "First Customer",
        description: "Offer a trial period for your first customer",
        cost: Cost::nothing(),
        condition: CardCondition::TicksAfterCard { card: 0, ticks: 50 },
        effect: CardEffect::AddClients(CloudClientSpec {
            amount: 1,
            service: ServiceKind::Base,
            trial_period: 100,
        }),
    },
];
