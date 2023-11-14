//! Module for the active entity in the game,
//! which takes the current state of the program
//! and processes it over time.

use std::collections::VecDeque;

use crate::{EventGenerator, Memory, Money, UserAction, WorldState};

use super::{
    cards::{all::ALL_CARDS, CardEffect, CardSpec},
    queue::{RequestEvent, RequestEventQueue, RequestEventStage, Time},
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
    /// all computational racks
    racks: Vec<CloudRack>,
    /// the event generator
    gen: EventGenerator,
}

impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            queue: RequestEventQueue::new(),
            racks: Vec::new(),
            gen: EventGenerator::new(),
        }
    }

    pub fn apply_action(&mut self, state: &mut WorldState, action: UserAction) {
        match action {
            UserAction::ApplyCost { cost } => {
                state.funds -= cost.money;
                state.base_service.available -= cost.base_ops;
                state.super_service.available -= cost.super_ops;
                state.epic_service.available -= cost.epic_ops;
                state.awesome_service.available -= cost.awesome_ops;
                state.spent += cost.money;
            }
            UserAction::Payment { amount } => {
                state.funds -= amount;
                state.spent += amount;
            }
            UserAction::ChangePrice { kind, new_price } => {
                // change the price and recalculate demand
                let service = state.service_by_kind_mut(kind);
            }
            UserAction::UpgradeCpu { node } => todo!(),
            UserAction::UpgradeRam { node } => todo!(),
            UserAction::AddNode => todo!(),
            UserAction::UseCard { id } => {
                // TODO
                // 1. find the card
                match ALL_CARDS.binary_search_by_key(&id.as_ref(), |c| &c.id) {
                    Ok(index) => {
                        let card = &ALL_CARDS[index];
                        // 2. apply the card's effects
                        if self.apply_card(state, card) {
                            // 3. add the card to the used cards list
                            // (but only if the card was actually applied)
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
            CardEffect::PublishService(_) => todo!(),
            CardEffect::UnlockService(_) => todo!(),
            CardEffect::AddFunds(money) => {
                state.funds += *money;
            }
            CardEffect::AddClients(_) => todo!(),
            CardEffect::AddClientsWithPublicity(_, _) => todo!(),
            CardEffect::UpgradeServices => todo!(),
        }
        true
    }

    /// Process the game state and produce new events.
    pub fn update(&mut self, state: &mut WorldState, time: Time) {
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
                if self.racks.len() == 1 && self.racks[0].nodes.len() == 1 {
                    // immediately route to the only node
                    push_event(event.into_routed(0, 0));
                } else {
                    // TODO route the request:
                    // 1. add processing to routing node
                    // 2. push event to request routed
                }

                if let Some(user_spec_id) = event.user_spec_id {
                    // also generate a new request for the upcoming request
                    // from the same client spec
                    if let Some(spec) = &state.user_specs.get(user_spec_id as usize) {
                        // determine demand for the service by this spec
                        let demand = 1.;
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
                // TODO
                // 1. pick a request processing node
                // 2. decrement processing on the routing node
                // 3.1. if not enough memory, drop the request.
                //      else, add memory usage to the processing node
                // 4.1. if node has a CPU available,
                // 4.1.1. then calculate time to process the request
                //        & increment CPU usage
                //        & push request processed event to the queue
                // 4.1.2. else add to waiting queue
                todo!()
            }
            RequestEventStage::RequestProcessed { node_num } => {
                // TODO
                // 1. decrement processing on the processing node
                // 2. increment op counts (available & total)
                // 3. add funds if applicable
            }
        }
    }
}

/// A request (or request set) waiting to be processed in a node.
#[derive(Debug, Clone, PartialEq)]
pub struct WaitingRequest {
    /// multiplier for the number of requests
    /// bundled into one
    amount: u32,

    /// the amount of memory required to process the request set
    memory: Memory,
}

/// A cloud processing node and its state
#[derive(Debug, Clone, PartialEq)]
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
    /// the amount of RAM currently in use
    pub ram_usage: Memory,

    /// queue of requests sitting in memory and waiting to be processed
    pub requests: VecDeque<WaitingRequest>,
}

impl CloudNode {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            cpu_level: 0,
            ram_level: 0,
            num_cores: CPU_LEVELS[0].0,
            ram_capacity: RAM_LEVELS[0].0,
            cpu_speed: CPU_LEVELS[0].1,
            ram_per_request: Memory::kb(512),
            processing: 0,
            ram_usage: Memory::zero(),
            requests: VecDeque::new(),
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
