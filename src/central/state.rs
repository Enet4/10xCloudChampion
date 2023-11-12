//! Module for the full game state
//!

use crate::{CloudUserSpec, Money, Ops};

use super::{engine::CloudNode, queue::Time};

#[derive(Debug, Clone, PartialEq)]
pub struct WorldState {
    /// the current timestamp
    pub time: Time,

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

    /// all active client specifications
    pub user_specs: Vec<CloudUserSpec>,

    /// all server nodes
    pub nodes: Vec<CloudNode>,

    /// the indices of the cards
    /// (per [`ALL_CARDS`](crate::central::cards::ALL_CARDS))
    /// already used,
    /// in used time order
    pub cards_used: Vec<UsedCard>,
}

impl WorldState {
    /// convenience method to retrieve a cloud node by id
    pub fn node(&self, id: u32) -> Option<&CloudNode> {
        self.nodes
            .binary_search_by_key(&id, |node| node.id)
            .ok()
            .map(|index| &self.nodes[index])
    }

    /// convenience method to retrieve a cloud node by id
    pub fn node_mut(&mut self, id: u32) -> Option<&mut CloudNode> {
        self.nodes
            .binary_search_by_key(&id, |node| node.id)
            .ok()
            .map(|index| &mut self.nodes[index])
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            time: 0,
            funds: Default::default(),
            spent: Default::default(),
            demand: 0.0,
            base_service: Default::default(),
            super_service: Default::default(),
            epic_service: Default::default(),
            requests_dropped: 0,
            awesome_service: Default::default(),
            nodes: vec![CloudNode::new(0)],
            user_specs: Default::default(),
            cards_used: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UsedCard {
    pub index: usize,
    pub time: Time,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ServiceCounts {
    /// the number of operations that the player has available
    /// from the service
    pub available: Ops,
    /// the total number of operations performed by the service
    pub total: Ops,
}
