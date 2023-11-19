//! module for Cloud users (clients and researchers)

use serde::{Deserialize, Serialize};

use crate::Time;

use super::stuff::ServiceKind;

/// The behavioral specification for a cloud user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

/// The non-live behavioral specification for a cloud client.
///
/// It is different from CloudUserSpec because it is never evil
/// and trial time is specified as a duration instead of a timestamp.
#[derive(Debug, Clone, PartialEq)]
pub struct CloudClientSpec {
    /// a base multiplier for users following this specification
    pub amount: u32,
    /// the service this client will be using
    pub service: ServiceKind,
    /// the time duration in which the user does not have to pay per request
    /// (set 0 to always pay)
    pub trial_duration: u32,
}
