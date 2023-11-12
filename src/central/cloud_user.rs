//! module for Cloud users (clients and researchers)

use crate::Time;

use super::stuff::ServiceKind;

/// The behavioral specification for a cloud user.
#[derive(Debug, Clone, PartialEq)]
pub struct CloudUserSpec {
    /// a base multiplier for users following this specification
    pub amount: u32,
    /// the service this client will be using
    pub service: ServiceKind,
    /// the time up to which the user does not have to pay per request
    /// (set 0 to always pay)
    pub trial_time: Time,
    /// whether the user is evil and only produces bad requests
    pub bad: bool,
}

impl CloudUserSpec {
    pub fn is_paying(&self, time: Time) -> bool {
        !self.bad && time >= self.trial_time
    }
}

impl Default for CloudUserSpec {
    fn default() -> Self {
        Self {
            amount: 1,
            service: ServiceKind::Base,
            trial_time: 0,
            bad: false,
        }
    }
}
