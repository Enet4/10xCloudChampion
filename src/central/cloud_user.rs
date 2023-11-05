//! module for Cloud users (clients and researchers)

use crate::{Ops, Tick};

use super::stuff::ServiceKind;

#[derive(Debug, Clone, PartialEq)]
pub struct CloudClientSpec {
    /// the number of clients following this specification
    pub amount: usize,
    /// the service this client will be using
    pub service: ServiceKind,
    /// the number of ticks for which the client does not have to pay
    pub trial_period: u32,
}

impl Default for CloudClientSpec {
    fn default() -> Self {
        Self {
            amount: 1,
            service: ServiceKind::Base,
            trial_period: 0,
        }
    }
}

/// Live information about a set of cloud clients.
#[derive(Debug, Clone, PartialEq)]
pub struct CloudClientSet {
    /// the number of clients in this set
    pub amount: usize,
    /// the service this client will be using
    pub service: ServiceKind,
    /// game time when the client was created
    pub created: Tick,
    /// the number of ticks for which the client does not have to pay
    pub trial_period: u32,
    /// the total number of operation requests fulfilled by this client
    pub total_ops: Ops,
}
