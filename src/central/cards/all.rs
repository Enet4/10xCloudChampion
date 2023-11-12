//! All card entries encoded here
//!

use crate::{CloudUserSpec, Cost, ServiceKind};

use super::{CardCondition, CardEffect, CardSpec};

pub static ALL_CARDS: &'static [CardSpec] = &[
    CardSpec {
        title: "Test your service",
        description: "Always test before delivering to the public",
        cost: Cost::base_ops(10),
        condition: CardCondition::appear_immediately(),
        effect: CardEffect::PublishService(ServiceKind::Base),
    },
    CardSpec {
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
];
