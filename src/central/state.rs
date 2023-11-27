//! Module for the full game state
//!

use std::borrow::Cow;

use js_sys::wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};

use crate::{CloudUserSpec, Cost, Memory, Money, Ops, ServiceKind};

use super::{
    engine::{
        CloudNode, AWESOME_MEMORY_RESERVE, BASE_MEMORY_RESERVE, ELECTRICITY_BILL_PERIOD,
        ELECTRICITY_COST_LEVELS, EPIC_MEMORY_RESERVE, SOFTWARE_LEVELS, SUPER_MEMORY_RESERVE,
    },
    queue::Time,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldState {
    /// the current timestamp
    pub time: Time,

    /// the player's current available funds
    pub funds: Money,

    /// the player's total money spent
    pub spent: Money,

    /// the total money earned by cloud service provisioning
    pub earned: Money,

    /// a measurement of demand for the services
    /// (a higher number means more request inflow
    /// post service & price adjustments)
    pub demand: f32,

    /// a measurement of demand for the services
    /// (a higher number means more client inflow)
    #[serde(default = "demand_rate_default")]
    pub demand_rate: f32,

    /// the number of upgrades done to the cloud service software
    pub software_level: u8,

    /// the number of upgrades done to caching
    /// (higher means more caching)
    pub cache_level: u8,

    /// the routing level used for requests
    #[serde(default, skip_serializing_if = "is_default_routing_level")]
    pub routing_level: RoutingLevel,

    /// number of operation requests performed per player click
    pub ops_per_click: u32,

    /// the op counts of the base service
    pub base_service: ServiceInfo,

    /// the op counts of the super service
    pub super_service: ServiceInfo,

    /// the op counts of the epic service
    pub epic_service: ServiceInfo,

    /// the total number of requests dropped
    /// due to lack of system capacity
    pub requests_dropped: u64,

    /// the total number of requests discarded
    /// for being bad
    #[serde(default, skip_serializing_if = "is_zero_u64")]
    pub requests_failed: u64,

    /// the op counts of the awesome service
    pub awesome_service: ServiceInfo,

    /// all active client specifications
    pub user_specs: Vec<CloudUserSpec>,

    /// electricity cost, consumption, and due payments
    pub electricity: Electricity,

    /// all server nodes
    pub nodes: Vec<CloudNode>,

    /// whether the player can see the service demand estimate
    /// in the business panel
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_see_demand: bool,

    /// whether the player can see the energy consumption estimate
    /// in the hardware panel
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_see_energy_consumption: bool,

    /// whether the player can see the request fulfillment & drop rates
    /// in the hardware panel
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_see_request_rates: bool,

    /// whether the player has unlocked
    /// buying more cloud nodes
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_buy_nodes: bool,

    /// whether the player has unlocked
    /// buying more cloud racks
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_buy_racks: bool,

    /// whether the player has unlocked
    /// buying more datacenters
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_buy_datacenters: bool,

    /// the rate at which to detect bad requests before routing them
    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub spam_protection: f32,

    /// the indices of the cards
    /// (per [`ALL_CARDS`](crate::central::cards::ALL_CARDS))
    /// already used,
    /// in used time order
    pub cards_used: Vec<UsedCard>,
}

fn demand_rate_default() -> f32 {
    0.25
}

fn is_default_routing_level(&routing_level: &RoutingLevel) -> bool {
    routing_level == RoutingLevel::default()
}

fn is_false(&b: &bool) -> bool {
    !b
}

fn is_zero_u64(&x: &u64) -> bool {
    x == 0
}

fn is_zero_f32(&x: &f32) -> bool {
    x == 0.
}

const LOCAL_STORAGE_KEY_NAME: &str = "10xCloudChampion_save";

impl WorldState {
    /// Load the game from local storage.
    ///
    /// Returns `Ok(None)` if there is no game save.
    pub fn load_game() -> Result<Option<Self>, JsValue> {
        let storage = try_local_storage()?;
        let json = storage.get_item(LOCAL_STORAGE_KEY_NAME)?;
        if let Some(json) = json {
            let state =
                serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            gloo_console::log!("Saved game loaded successfully");
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    /// Checks whether there is a saved game.
    pub fn has_saved_game() -> Result<bool, JsValue> {
        let storage = try_local_storage()?;
        let item = storage.get_item(LOCAL_STORAGE_KEY_NAME)?;
        Ok(item.is_some())
    }

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

    /// Get total processing power and memory usage,
    /// between 0 and 1
    pub fn total_processing(&self) -> (f32, f32) {
        let mut cpu = 0;
        let mut mem = Memory::zero();
        let mut cpu_capacity = 0;
        let mut mem_capacity = Memory::zero();
        for node in &self.nodes {
            cpu += node.processing;
            mem += node.ram_usage;
            cpu_capacity += node.num_cores;
            mem_capacity += node.ram_capacity;
        }
        (cpu as f32 / cpu_capacity as f32, mem.ratio(mem_capacity))
    }

    /// Returns `Ok(())` if the game environment can be saved.
    pub fn can_save_game() -> Result<(), JsValue> {
        try_local_storage().map(|_| ())
    }

    /// save the world state to local storage
    pub fn save_game(&self) -> Result<(), JsValue> {
        let storage = try_local_storage()?;
        let json = serde_json::to_string(self).map_err(|e| JsValue::from_str(&e.to_string()))?;
        storage.set_item(LOCAL_STORAGE_KEY_NAME, &json)?;
        gloo_console::log!("Game saved");
        Ok(())
    }

    pub(crate) fn user_spec(&self, id: u32) -> Option<&CloudUserSpec> {
        self.user_specs
            .binary_search_by_key(&id, |spec| spec.id)
            .ok()
            .map(|index| &self.user_specs[index])
    }

    pub(crate) fn next_user_spec_id(&self) -> u32 {
        self.user_specs
            .iter()
            .map(|spec| spec.id)
            .max()
            .unwrap_or(0)
            + 1
    }

    pub(crate) fn apply_cost(&mut self, cost: &Cost) {
        self.funds -= cost.money;
        self.spent += cost.money;
        self.base_service.available -= cost.base_ops;
        self.super_service.available -= cost.super_ops;
        self.epic_service.available -= cost.epic_ops;
        self.awesome_service.available -= cost.awesome_ops;
    }

    /// The maximum amount of memory that a cloud node is expected to reserve
    /// in order to provide all unlocked services.
    pub(crate) fn expected_ram_reserved(&self) -> Memory {
        // check highest service tier
        let base_reserve = match self.service_tier() {
            ServiceKind::Awesome => AWESOME_MEMORY_RESERVE,
            ServiceKind::Epic => EPIC_MEMORY_RESERVE,
            ServiceKind::Super => SUPER_MEMORY_RESERVE,
            ServiceKind::Base => BASE_MEMORY_RESERVE,
        };

        // apply factor based on software level
        let factor = SOFTWARE_LEVELS[self.software_level as usize].1;
        base_reserve * factor
    }

    pub(crate) fn service_tier(&self) -> ServiceKind {
        match (
            self.base_service.unlocked,
            self.super_service.unlocked,
            self.epic_service.unlocked,
            self.awesome_service.unlocked,
        ) {
            (_, _, _, true) => ServiceKind::Awesome,
            (_, _, true, false) => ServiceKind::Epic,
            (_, true, false, false) => ServiceKind::Super,
            (true, false, false, false) => ServiceKind::Base,
            _ => unreachable!("base service should not be locked"),
        }
    }

    pub fn is_powersaving(&self) -> bool {
        self.electricity.total_due > Money::dollars(10)
            && self.time - self.electricity.last_bill_time >= ELECTRICITY_BILL_PERIOD
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            time: 0,
            funds: Money::dollars(10),
            spent: Default::default(),
            earned: Default::default(),
            demand: 0.0,
            demand_rate: 0.25,
            software_level: 0,
            cache_level: 0,
            ops_per_click: 1,
            spam_protection: 0.0,
            base_service: ServiceInfo::new_private(Money::millicents(50)),
            super_service: ServiceInfo::new_locked(Money::dec_cents(5)),
            epic_service: ServiceInfo::new_locked(Money::cents(5)),
            awesome_service: ServiceInfo::new_locked(Money::dollars(1)),
            electricity: Default::default(),
            requests_dropped: 0,
            requests_failed: 0,
            nodes: vec![CloudNode::new(0)],
            can_see_demand: false,
            can_see_energy_consumption: false,
            can_see_request_rates: false,
            can_buy_nodes: false,
            can_buy_racks: false,
            can_buy_datacenters: false,
            routing_level: RoutingLevel::default(),
            user_specs: Default::default(),
            cards_used: Default::default(),
        }
    }
}

/// The different forms of request routing implemented.
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RoutingLevel {
    /// The first node of the first rack handles all the routing.
    #[default]
    MainNode = 0,
    /// Routing costs are distributed among all nodes.
    Distributed = 1,
    /// All routing costs removed.
    NoRoutingCost = 2,
}

impl RoutingLevel {
    /// Get the highest routing level of the two.
    pub fn max(self, other: Self) -> Self {
        match (self, other) {
            (Self::NoRoutingCost, _) | (_, Self::NoRoutingCost) => Self::NoRoutingCost,
            (Self::Distributed, _) | (_, Self::Distributed) => Self::Distributed,
            _ => Self::MainNode,
        }
    }
}

/// The record that a project card has been used, and when.
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
    /// entitlement: money earned per op
    /// regardless of who issued it
    /// (except for DoS attacks)
    #[serde(default)]
    pub entitlement: Money,
    /// the number of operations that the player has available
    /// from the service
    pub available: Ops,
    /// the total number of operations performed by the service
    pub total: Ops,
    /// whether the service has already been unlocked
    #[serde(default = "unlocked_default")]
    pub unlocked: bool,
    /// whether the service is still private (true)
    /// or available for public use (false)
    #[serde(default)]
    pub private: bool,
}

fn unlocked_default() -> bool {
    true
}

impl ServiceInfo {
    pub const fn new_private(price: Money) -> Self {
        Self {
            price,
            entitlement: Money::zero(),
            available: Ops(0),
            total: Ops(0),
            unlocked: true,
            private: true,
        }
    }

    pub const fn new_locked(price: Money) -> Self {
        Self {
            price,
            entitlement: Money::zero(),
            available: Ops(0),
            total: Ops(0),
            unlocked: false,
            private: true,
        }
    }

    /// calculate service demand based on base demand and price
    pub fn calculate_demand(&self, base_demand: f32) -> f32 {
        let millicents = (self.price.to_millicents() as f32).max(0.25);
        base_demand * 0.0009765625 + base_demand * 12288. / millicents.powf(2.125)
    }
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

/// World state portion for electricity cost, consumption, and due payments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Electricity {
    /// the current electricity cost level.
    ///
    /// Use [`ELECTRICITY_COST_LEVELS`] to translate this to money per Wattever
    pub cost_level: u8,

    /// the amount of electricity consumed since the last bill in milliWattever
    pub consumed: f64,

    /// the total amount of electricity consumed in milliWattever
    pub total_consumed: f64,

    /// the total amount of electricity payment due
    pub total_due: Money,

    /// the timestamp of the last bill
    /// (or 0 if no bills have been issued yet)
    pub last_bill_time: Time,

    /// The amount of energy recently consumed
    ///
    /// Transient.
    #[serde(skip, default)]
    pub recent_energy_consumed: f64,

    /// The energy consumption calculated last major update
    ///
    /// Transient.
    #[serde(skip, default)]
    pub energy_consumption_rate: f64,
}

impl Electricity {
    pub fn add_consumption(&mut self, milli_wattever: f64) {
        self.consumed += milli_wattever;
        self.total_consumed += milli_wattever;
        self.recent_energy_consumed += milli_wattever;
    }

    pub fn calculate_consumption_rate(&mut self) -> f64 {
        let rate = self.recent_energy_consumed;
        self.recent_energy_consumed = 0.;
        self.energy_consumption_rate = rate;
        rate
    }

    /// Calculate the cost of the bill if it were to be emitted now
    pub fn check_bill(&self) -> Money {
        ELECTRICITY_COST_LEVELS[self.cost_level as usize] * (self.consumed * 1e-3)
    }

    /// emit a bill for the consumed electricity,
    /// and reset the consumed amount to zero
    pub fn emit_bill_for(&mut self, total_cost: Money, time: Time) {
        self.total_due += total_cost;
        self.consumed = 0.;
        self.last_bill_time = time;
    }
}

impl Default for Electricity {
    fn default() -> Self {
        Self {
            cost_level: 0,
            consumed: 0.0,
            total_consumed: 0.0,
            total_due: Money::zero(),
            last_bill_time: 0,
            recent_energy_consumed: 0.,
            energy_consumption_rate: 0.,
        }
    }
}
