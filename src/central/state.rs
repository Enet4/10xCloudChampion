//! Module for the full game state
//!

use std::borrow::Cow;

use js_sys::wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};

use crate::{CloudUserSpec, Cost, Money, Ops, ServiceKind};

use super::{engine::CloudNode, queue::Time};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    /// the number of upgrades done to the cloud service software
    pub software_level: u8,

    /// the op counts of the base service
    pub base_service: ServiceInfo,

    /// the op counts of the super service
    pub super_service: ServiceInfo,

    /// the op counts of the epic service
    pub epic_service: ServiceInfo,

    /// the total number of requests dropped
    /// due to lack of system capacity
    pub requests_dropped: u64,

    /// the op counts of the awesome service
    pub awesome_service: ServiceInfo,

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

    pub fn service_by_kind_mut(&mut self, kind: crate::ServiceKind) -> &mut ServiceInfo {
        match kind {
            ServiceKind::Base => &mut self.base_service,
            ServiceKind::Super => &mut self.super_service,
            ServiceKind::Epic => &mut self.epic_service,
            ServiceKind::Awesome => &mut self.awesome_service,
        }
    }

    pub fn can_afford(&self, cost: &Cost) -> bool {
        self.funds >= cost.money
            && self.base_service.available >= cost.base_ops
            && self.super_service.available >= cost.super_ops
            && self.epic_service.available >= cost.epic_ops
            && self.awesome_service.available >= cost.awesome_ops
    }

    pub fn is_card_used(&self, card_id: &str) -> bool {
        self.cards_used.iter().any(|c| c.id == card_id)
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            time: 0,
            funds: Default::default(),
            spent: Default::default(),
            demand: 0.0,
            software_level: 0,
            base_service: ServiceInfo {
                price: Money::dec_cents(1),
                available: Ops(1_000),
                total: Ops(2_000),
                locked: false,
                private: false,
            },
            super_service: ServiceInfo {
                price: Money::dec_cents(5),
                available: Ops(100),
                total: Ops(200),
                locked: false,
                private: false,
            },
            epic_service: ServiceInfo {
                price: Money::cents(2),
                available: Ops(0),
                total: Ops(0),
                locked: false,
                private: true,
            },
            requests_dropped: 0,
            awesome_service: Default::default(),
            nodes: vec![CloudNode::new(0)],
            user_specs: Default::default(),
            cards_used: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsedCard {
    pub id: Cow<'static, str>,
    pub time: Time,
}

/// Live information about a cloud service in the game,
/// namely the current price per op,
/// how many ops are available to spend,
/// how many ops were performed in total.
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// the price per op
    pub price: Money,
    /// the number of operations that the player has available
    /// from the service
    pub available: Ops,
    /// the total number of operations performed by the service
    pub total: Ops,
    /// whether the service has not been unlocked yet
    pub locked: bool,
    /// whether the service is still private (true)
    /// or available for public use (false)
    #[serde(default)]
    pub private: bool,
}

/// Gracefully try to obtain the Web local storage API.
pub fn try_local_storage() -> Result<web_sys::Storage, JsValue> {
    web_sys::window()
        .ok_or_else(|| JsValue::from_str("Could not obtain window"))
        .and_then(|window| {
            window
                .local_storage()
                .and_then(|x| x.ok_or_else(|| JsValue::from_str("Could not obtain local storage")))
        })
}
