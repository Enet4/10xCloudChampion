//! Module for the full game state
//!

use crate::{Money, Ops};

use super::{cloud_user::CloudClientSet, queue::Tick};

#[derive(Debug, Clone, PartialEq)]
pub struct WorldState {
    /// the current timestamp
    pub tick: Tick,

    /// the player's current available funds
    pub funds: Money,

    /// the player's total money spent
    pub spent: Money,

    /// a measurement of demand for the services
    /// (a higher number means more client inflow)
    pub demand: f32,

    /// the op counts of the base service
    pub base_service: ServiceCounts,

    /// the op counts of the super service
    pub super_service: ServiceCounts,

    /// the op counts of the epic service
    pub epic_service: ServiceCounts,

    /// the total number of requests dropped
    /// due to lack of system capacity
    pub requests_dropped: u64,

    /// the op counts of the awesome service
    pub awesome_service: ServiceCounts,

    /// all active clients
    pub clients: Vec<CloudClientSet>,

    /// all active researchers
    pub researchers: u32,

    /// the indices of the cards
    /// (per [`ALL_CARDS`](crate::central::cards::ALL_CARDS))
    /// already used,
    /// in used time order
    pub cards_used: Vec<UsedCard>,
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            tick: 0,
            funds: Default::default(),
            spent: Default::default(),
            demand: 0.0,
            base_service: Default::default(),
            super_service: Default::default(),
            epic_service: Default::default(),
            requests_dropped: 0,
            awesome_service: Default::default(),
            clients: Default::default(),
            researchers: 0,
            cards_used: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UsedCard {
    pub index: usize,
    pub tick: Tick,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ServiceCounts {
    /// the number of operations that the player has available
    /// from the service
    pub available: Ops,
    /// the total number of operations performed by the service
    pub total: Ops,
}
