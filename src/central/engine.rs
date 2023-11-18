//! Module for the active entity in the game,
//! which takes the current state of the program
//! and processes it over time.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{SampleGenerator, Memory, Money, PlayerAction, WorldState, ServiceKind, Ops};

use super::{
    cards::{all::ALL_CARDS, CardEffect, CardSpec},
    queue::{RequestEvent, RequestEventQueue, RequestEventStage, Time},
    state::UsedCard,
};

/// all levels of CPU upgrades
/// (count, base speed, and cost),
/// where the first value is the initial level
pub static CPU_LEVELS: [(u32, u32, Money); 11] = [
    (1, 2, Money::dollars(0)),
    (2, 2, Money::dollars(100)),
    (3, 3, Money::dollars(200)),
    (4, 3, Money::dollars(400)),
    (6, 4, Money::dollars(1_200)),
    (8, 5, Money::dollars(2_500)),
    (16, 7, Money::dollars(4_000)),
    (24, 10, Money::dollars(6_000)),
    (32, 12, Money::dollars(10_000)),
    (48, 15, Money::dollars(20_000)),
    (64, 20, Money::dollars(42_000)),
];

/// all levels of RAM that can be purchased and their base cost,
/// where the first value is the initial level
pub static RAM_LEVELS: [(Memory, Money); 11] = [
    (Memory::mb(256), Money::dollars(0)),
    (Memory::mb(512), Money::dollars(80)),
    (Memory::gb(1), Money::dollars(120)),
    (Memory::mb(2), Money::dollars(220)),
    (Memory::gb(4), Money::dollars(500)),
    (Memory::gb(8), Money::dollars(860)),
    (Memory::gb(16), Money::dollars(1_500)),
    (Memory::gb(24), Money::dollars(2_500)),
    (Memory::gb(16), Money::dollars(4_000)),
    (Memory::gb(32), Money::dollars(6_000)),
    (Memory::gb(64), Money::dollars(8_000)),
];

/// amount of memory that all each cloud node must reserve
/// to provide the base cloud service tier,
/// before modifiers
pub static BASE_MEMORY_RESERVE: Memory = Memory::mb(32);
/// amount of memory that all each cloud node must reserve
/// to provide the super cloud service tier,
/// before modifiers
pub static SUPER_MEMORY_RESERVE: Memory = Memory::mb(128);
/// amount of memory that all each cloud node must reserve
/// to provide the epic cloud service tier,
/// before modifiers
pub static EPIC_MEMORY_RESERVE: Memory = Memory::gb(2);
/// amount of memory that all each cloud node must reserve
/// to provide the awesome cloud service tier,
/// before modifiers
pub static AWESOME_MEMORY_RESERVE: Memory = Memory::gb(16);

/// The main game engine, which processes the game state
/// and produces new events.
#[derive(Debug)]
pub struct GameEngine {
    /// the event queue
    queue: RequestEventQueue,
    /// the number generator
    gen: SampleGenerator,
}

impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            queue: RequestEventQueue::new(),
            gen: SampleGenerator::new(),
        }
    }

    pub fn apply_action(&mut self, state: &mut WorldState, action: PlayerAction) {
        match action {
            PlayerAction::OpClick { kind, amount } => {
                // schedule the operation
                let time = state.time + 1;
                self.queue.push(RequestEvent::new_arrived(
                    time,
                    None,
                    amount,
                    kind,
                    false,
                ));
            }
            PlayerAction::ApplyCost { cost } => {
                state.funds -= cost.money;
                state.base_service.available -= cost.base_ops;
                state.super_service.available -= cost.super_ops;
                state.epic_service.available -= cost.epic_ops;
                state.awesome_service.available -= cost.awesome_ops;
                state.spent += cost.money;
            }
            PlayerAction::Payment { amount } => {
                state.funds -= amount;
                state.spent += amount;
            }
            PlayerAction::ChangePrice { kind, new_price } => {
                // change the price and recalculate demand
                let service = state.service_by_kind_mut(kind);
                service.price = new_price;
            }
            PlayerAction::UpgradeCpu { node } => {
                let funds = state.funds;
                let node = state.node_mut(node).unwrap();
                let next_level = node.cpu_level + 1;
                if next_level as usize >= CPU_LEVELS.len() {
                    return;
                }
                let (num_cores, cpu_speed, cost) = CPU_LEVELS[next_level as usize];
                if funds < cost {
                    return;
                }
                node.cpu_level = next_level;
                node.num_cores = num_cores;
                node.cpu_speed = cpu_speed;
                state.funds -= cost;
                state.spent += cost;
            },
            PlayerAction::UpgradeRam { node } => {
                let funds = state.funds;
                let node = state.node_mut(node).unwrap();
                let next_level = node.ram_level + 1;
                if next_level as usize >= RAM_LEVELS.len() {
                    return;
                }
                let (ram_capacity, cost) = RAM_LEVELS[next_level as usize];
                if funds < cost {
                    return;
                }
                node.ram_level = next_level;
                node.ram_capacity = ram_capacity;
                state.funds -= cost;
                state.spent += cost;
            },
            PlayerAction::AddNode => {
                todo!()
            },
            PlayerAction::UseCard { id } => {
                // 1. find the card
                match ALL_CARDS.binary_search_by_key(&id.as_ref(), |c| &c.id) {
                    Ok(index) => {
                        let card = &ALL_CARDS[index];
                        // 2. apply the card's effects
                        if self.apply_card(state, card) {
                            // 3. add the card to the used cards list
                            // (but only if the card was actually applied)
                            let time = state.time;
                            state.cards_used.push(UsedCard {
                                id: id.clone(),
                                time,
                            });
                        }
                    }
                    Err(_) => {
                        // warn
                        gloo_console::warn!("Bad card identifier ", id.as_ref());
                    }
                }
            }
        }
    }

    fn apply_card(&mut self, state: &mut WorldState, card: &CardSpec) -> bool {
        let effect = &card.effect;
        let cost = &card.cost;
        // check if player can afford the card
        if !state.can_afford(cost) {
            return false;
        }

        match effect {
            CardEffect::Nothing => { /* no op */ }
            CardEffect::PublishService(kind) => {
                let service = state.service_by_kind_mut(*kind);
                service.private = false;
            }
            CardEffect::UnlockService(kind) => todo!(),
            CardEffect::AddFunds(money) => {
                state.funds += *money;
            }
            CardEffect::AddClients(_) => todo!(),
            CardEffect::AddClientsWithPublicity(specs, demand_delta) => {
                state.demand += demand_delta;
                // TODO add clients
            }
            CardEffect::AddPublicity(demand_delta) => {
                state.demand += demand_delta;
            }
            CardEffect::UpgradeServices => todo!(),
        }
        true
    }

    /// Initiate request arrival events based on the current world state
    pub fn bootstrap_events(&mut self, state: &mut WorldState) {
        for (i, user_spec) in state.user_specs.iter().enumerate() {
            // calculate demand based on base demand and cloud service price
            let service = match user_spec.service {
                crate::ServiceKind::Base => &state.base_service,
                crate::ServiceKind::Super => &state.super_service,
                crate::ServiceKind::Epic => &state.epic_service,
                crate::ServiceKind::Awesome => &state.awesome_service,
            };

            let demand = service.calculate_demand(state.demand);
            let duration = self.gen.next_request(demand);
            let timestamp = duration as u64;
            self.queue.push(RequestEvent::new_arrived(
                timestamp,
                Some(i as u32),
                user_spec.amount,
                user_spec.service,
                user_spec.bad,
            ));
        }
    } 

    /// Process the game state and produce new events.
    pub fn update(&mut self, state: &mut WorldState, time: Time) {

        // check whether to do a major update
        let duration = time - state.time;
        if duration > 0 && time / 2_500 - state.time / 2_500 > 0 {
            // do a major update
            self.update_major(state, time);
        }

        // grab upcoming events
        let mut events = self.queue.events_until(time);

        let mut preemptive_events = Vec::new();

        loop {
            // process events until all of them only happen after the given time
            for event in events {
                self.process_event(state, time, event, &mut preemptive_events);
            }
            if !preemptive_events.is_empty() {
                // do another round of processing
                events = preemptive_events.clone();
                preemptive_events.clear();
            } else {
                break;
            }
        }
        // update time
        state.time = time;
    }

    /// Do a major update, which performs heavier stuff periodically.
    fn update_major(&mut self, state: &mut WorldState, time: Time) {
    }

    /// process a single event
    fn process_event(
        &mut self,
        state: &mut WorldState,
        time: Time,
        event: RequestEvent,
        preemptive_events: &mut Vec<RequestEvent>,
    ) {
        // closure to add a new event to the main queue or the preemptive queue
        // (the preemptive queue is for events
        // which need to be processed in this iteration)
        let mut push_event = |event: RequestEvent| {
            if event.timestamp <= time {
                preemptive_events.push(event);
            } else {
                self.queue.push(event);
            }
        };

        match event.kind {
            RequestEventStage::RequestArrived => {
                // route the request if necessary
                let node_count = state.nodes.len() as u32;
                if node_count == 1 {
                    // immediately route to the only node
                    push_event(event.into_routed(0, 0));
                } else {
                    // route the request:
                    // 1. add processing to routing node
                    let node_num = self.gen.gen_range(0, node_count);

                    let node = state.node_mut(node_num).unwrap();
                    node.processing += 1;
                    let duration = node.time_per_request_routing();

                    // 2. push event to request routed
                    push_event(event.into_routed(duration, node_num));
                }

                if let Some(user_spec_id) = event.user_spec_id {
                    // also generate a new request for the upcoming request
                    // from the same client spec
                    if let Some(spec) = &state.user_specs.get(user_spec_id as usize) {
                        // determine demand for the service by this spec
                        let service = match spec.service {
                            crate::ServiceKind::Base => &state.base_service,
                            crate::ServiceKind::Super => &state.super_service,
                            crate::ServiceKind::Epic => &state.epic_service,
                            crate::ServiceKind::Awesome => &state.awesome_service,
                        };
                        let demand = service.calculate_demand(state.demand);
                        let duration = self.gen.next_request(demand);
                        let timestamp = event.timestamp + duration as u64 * event.amount as u64;
                        push_event(RequestEvent::new_arrived(
                            timestamp,
                            event.user_spec_id,
                            spec.amount,
                            spec.service,
                            spec.bad,
                        ));
                    } else {
                        // warn
                    }
                }
            }
            RequestEventStage::RequestRouted { node_num } => {
                let routing_needed = state.nodes.len() > 1;
                let Some(routing_node) = state.node_mut(node_num) else {
                    return
                };

                // 1. decrement processing on the routing node
                if routing_needed {
                    routing_node.processing -= 1;
                }
                
                // 2. pick a request processing node
                let node_id = self.gen.gen_range(0, state.nodes.len() as u32);

                
                // 3. check memory requirement
                let node = state.node_mut(node_id).unwrap();
                let mem_required = node.ram_per_request * event.amount as i32;
                if mem_required > node.ram_capacity - node.ram_usage {
                    // 3.1. if not enough memory, drop the request.
                    state.requests_dropped += event.amount as u64;
                    return;
                }
                // 4. add memory usage to the processing node
                gloo_console::debug!("Increased memory usage by ", mem_required.to_string());
                node.ram_usage += mem_required;

                // 5. if node has a CPU available,
                if node.processing < node.num_cores {
                    // then calculate time to process the request
                    let duration = node.time_per_request(event.service) * event.amount;
                    //  & increment CPU usage
                    node.processing += 1;
                    //  & push request processed event to the queue
                    push_event(event.into_processed(duration, mem_required));
                } else {
                    // add to waiting queue
                    node.requests.push_back(WaitingRequest {
                        amount: event.amount,
                        user_spec_id: event.user_spec_id,
                        service: event.service,
                        mem_required,
                    });
                }
            }
            RequestEventStage::RequestProcessed { node_num, ram_required } => {
                if state.node(node_num).is_none() {
                    return
                };
                // 1. increment op counts (available & total)
                let service = state.service_by_kind_mut(event.service);
                if !event.bad {
                    service.total += Ops(event.amount as i64);
                    service.available += Ops(event.amount as i64);
                }

                // 2. calculate revenue if applicable
                let revenue = if !event.bad && event.user_spec_id.is_some() {
                    service.price * event.amount as i32
                } else {
                    Money::zero()
                };

                let node = state.node_mut(node_num).unwrap();
                // 3. decrement memory usage
                gloo_console::debug!("Reduced memory usage by ", ram_required.to_string());
                node.ram_usage -= ram_required;


                // 4. if there are requests waiting
                if !node.requests.is_empty() {
                    // pop one and schedule a new request processed event
                    let request = node.requests.pop_front().unwrap();
                    let duration = node.time_per_request(event.service) * request.amount;

                    let service = request.service;
                    let bad = if let Some(id) = request.user_spec_id {
                        state.user_specs[id as usize].bad
                    } else {
                        false
                    };
                    push_event(RequestEvent {
                        timestamp: event.timestamp + duration as u64,
                        user_spec_id: request.user_spec_id,
                        amount: request.amount,
                        service,
                        bad,
                        kind: RequestEventStage::RequestProcessed { node_num, ram_required }
                    })
                } else {
                    // decrement processing on the processing node
                    node.processing -= 1;
                }

                // apply revenue
                if revenue > Money::zero() {
                    state.funds += revenue;
                    state.earned += revenue;
                }
            }
        }
    }
}

/// A request (or request set) waiting to be processed in a node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaitingRequest {
    /// multiplier for the number of requests
    /// bundled into one
    amount: u32,
    
    /// the cloud user specification ID
    /// (or None if it was requested by the player)
    user_spec_id: Option<u32>,

    /// the request's cloud service kind
    service: ServiceKind,

    /// the amount of memory required to process the request set
    mem_required: Memory,
}

/// A cloud processing node and its state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudNode {
    /// a unique identifier for the node
    pub id: u32,
    /// the node's CPU level (see [`CPU_LEVELS`])
    pub cpu_level: u8,
    /// the node's RAM level (see [`RAM_LEVELS`])
    pub ram_level: u8,

    /// the number of requests that it can fulfill in parallel
    pub num_cores: u32,
    /// the amount of memory that it has
    pub ram_capacity: Memory,

    /// the base speed of the node,
    /// which translates to the number of base requests fulfilled per core
    /// (higher tiered services will have a lower speed)
    pub cpu_speed: u32,
    /// the amount of RAM required to process a request in one of the cores
    pub ram_per_request: Memory,

    /// the number of requests currently being processed right now
    pub processing: u32,
    /// the total amount of RAM in use
    pub ram_usage: Memory,
    /// how much of `ram_usage` is reserved
    pub ram_reserved: Memory,

    /// queue of requests sitting in memory and waiting to be processed
    pub requests: VecDeque<WaitingRequest>,
}

impl CloudNode {
    pub fn new(id: u32, ram_reserved: Memory) -> Self {
        Self {
            id,
            cpu_level: 0,
            ram_level: 0,
            num_cores: CPU_LEVELS[0].0,
            ram_capacity: RAM_LEVELS[0].0,
            cpu_speed: CPU_LEVELS[0].1,
            ram_per_request: Memory::kb(512),
            processing: 0,
            ram_usage: ram_reserved,
            ram_reserved,
            requests: VecDeque::new(),
        }
    }

    pub(crate) fn time_per_request(&self, service: ServiceKind) -> u32 {
        let factor = match service {
            ServiceKind::Base => 1,
            ServiceKind::Super => 2,
            ServiceKind::Epic => 8,
            ServiceKind::Awesome => 32,
        };

        1_000 * factor / self.cpu_speed
    }

    pub(crate) fn time_per_request_routing(&self) -> u32 {
        200 / self.cpu_speed
    }

    pub fn next_cpu_upgrade_cost(&self) -> Option<Money> {
        if self.cpu_level as usize + 1 < CPU_LEVELS.len() {
            Some(CPU_LEVELS[self.cpu_level as usize + 1].2)
        } else {
            None
        }
    }

    pub fn next_ram_upgrade_cost(&self) -> Option<Money> {
        if self.ram_level as usize + 1 < RAM_LEVELS.len() {
            Some(RAM_LEVELS[self.ram_level as usize + 1].1)
        } else {
            None
        }
    }
}

/// A rack containing multiple nodes
#[derive(Debug)]
pub struct CloudRack {
    pub nodes: Vec<CloudNode>,
    /// the maximum capacity of a single rack
    pub capacity: u8,
}
