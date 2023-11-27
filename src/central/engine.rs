//! Module for the active entity in the game,
//! which takes the current state of the program
//! and processes it over time.

use std::collections::VecDeque;
use yew::{html::Scope, prelude::*};

use serde::{Deserialize, Serialize};

use crate::{
    CloudUserSpec, Memory, Money, Ops, PlayerAction, SampleGenerator, ServiceKind, WorldState,
};

use super::{
    cards::{all::ALL_CARDS, CardEffect, CardSpec},
    queue::{RequestEvent, RequestEventQueue, RequestEventStage, Time},
    state::{RoutingLevel, UsedCard},
};

/// all levels of CPU upgrades
/// (count, base speed, and cost),
/// where the first value is the initial level
pub static CPU_LEVELS: [(u32, u32, Money); 11] = [
    (1, 2, Money::dollars(0)),
    (2, 2, Money::dollars(60)),
    (3, 2, Money::dollars(160)),
    (4, 3, Money::dollars(360)),
    (6, 3, Money::dollars(850)),
    (8, 4, Money::dollars(1_980)),
    (16, 4, Money::dollars(3_000)),
    (24, 5, Money::dollars(5_400)),
    (32, 6, Money::dollars(8_200)),
    (48, 7, Money::dollars(14_000)),
    (64, 8, Money::dollars(25_000)),
];

/// all levels of RAM that can be purchased and their base cost,
/// where the first value is the initial level
pub static RAM_LEVELS: [(Memory, Money); 11] = [
    (Memory::mb(256), Money::dollars(0)),
    (Memory::mb(512), Money::dollars(40)),
    (Memory::gb(1), Money::dollars(60)),
    (Memory::gb(2), Money::dollars(100)),
    (Memory::gb(4), Money::dollars(180)),
    (Memory::gb(8), Money::dollars(320)),
    (Memory::gb(16), Money::dollars(660)),
    (Memory::gb(24), Money::dollars(900)),
    (Memory::gb(32), Money::dollars(1_200)),
    (Memory::gb(48), Money::dollars(2_000)),
    (Memory::gb(64), Money::dollars(3_600)),
];

/// The cost of a bare node
pub const BARE_NODE_COST: Money = Money::dollars(2_000);

/// The cost of a fully upgraded node
pub const UPGRADED_NODE_COST: Money = BARE_NODE_COST
    .plus(Money::dollars(59_000))
    .plus(Money::dollars(9_000));

/// All levels of caching,
/// namely the memory reserve multiplier (0)
/// and the cache hit rate (1)
pub static CACHE_LEVELS: [(f32, f32); 5] =
    [(1., 0.), (4., 0.25), (16., 0.5), (18., 0.75), (20., 0.875)];

/// Modifiers for the time to process a request and memory required,
/// a number between 0 and 1,
/// where 1 means full cost.
pub static SOFTWARE_LEVELS: [(f64, f64); 5] = [
    (1., 1.),
    (0.9375, 0.96875),
    (0.625, 0.75),
    (0.3125, 0.625),
    (0.0390625, 0.125),
];

/// The electricity cost in Wattever
pub static ELECTRICITY_COST_LEVELS: [Money; 7] = [
    // base cost
    Money::cents(32),
    // renegotiate
    Money::cents(29),
    // repair A/C
    Money::cents(25),
    // buy solar panels
    Money::cents(18),
    // commit to clean power plan
    Money::cents(5),
    // dedicated power plant
    Money::dec_cents(5),
    // free energy
    Money::zero(),
];

/// amount of memory that all each cloud node must reserve
/// to provide the base cloud service tier,
/// before modifiers
pub static BASE_MEMORY_RESERVE: Memory = Memory::mb(32);
/// amount of memory that all each cloud node must reserve
/// to provide the super cloud service tier,
/// before modifiers
pub static SUPER_MEMORY_RESERVE: Memory = Memory::mb(256);
/// amount of memory that all each cloud node must reserve
/// to provide the epic cloud service tier,
/// before modifiers
pub static EPIC_MEMORY_RESERVE: Memory = Memory::gb(2);
/// amount of memory that all each cloud node must reserve
/// to provide the awesome cloud service tier,
/// before modifiers
pub static AWESOME_MEMORY_RESERVE: Memory = Memory::gb(16);

/// the threshold of base demand at which DoS attacks will emerge
pub static DEMAND_DOS_THRESHOLD: f32 = 2500.0;

/// time period after which base demand increases a small bit
pub static INCREASE_DEMAND_PERIOD: u64 = 150_000;

/// time period after which the user is given electricity bills to pay
pub static ELECTRICITY_BILL_PERIOD: u64 = 2_500_000;

/// time period after which a major update is performed
/// (also subtle but can do more expensive things)
pub static MAJOR_UPDATE_PERIOD: u64 = 3_000;

/// time period after which the game is automatically saved to local storage
pub static GAME_SAVE_PERIOD: u64 = 360_000;

/// The main game engine, which processes the game state
/// and produces new events.
#[derive(Debug)]
pub struct GameEngine<C: Component> {
    /// the event queue
    queue: RequestEventQueue,
    /// the number generator
    gen: SampleGenerator,
    /// link to the main game component's context,
    /// so that we can send messages back to it
    link: Scope<C>,
    /// a waiting queue where requests are placed
    /// when no node is available to process them
    waiting_queue: VecDeque<WaitingRouteRequest>,
}

impl<C> GameEngine<C>
where
    C: Component,
{
    pub fn new(link: Scope<C>) -> Self
    where
        C: BaseComponent,
    {
        GameEngine {
            queue: RequestEventQueue::new(),
            gen: SampleGenerator::new(),
            waiting_queue: VecDeque::new(),
            link,
        }
    }

    pub fn apply_action(&mut self, state: &mut WorldState, action: PlayerAction) {
        match action {
            PlayerAction::OpClick { kind, amount } => {
                // schedule the operation
                let time = state.time + 1;
                self.queue
                    .push(RequestEvent::new_arrived(time, None, amount, kind, false));
            }
            PlayerAction::Payment { amount } => {
                state.funds -= amount;
                state.spent += amount;
            }
            PlayerAction::PayElectricityBill => {
                self.apply_action(
                    state,
                    PlayerAction::Payment {
                        amount: state.electricity.total_due,
                    },
                );
                state.electricity.total_due = Money::zero();
                state.electricity.last_bill_time = state.time;
            }
            PlayerAction::ChangePrice { kind, new_price } => {
                //let demand = state.demand;
                // change the price and recalculate demand
                let service = state.service_by_kind_mut(kind);
                service.price = new_price;

                /*
                gloo_console::debug!(
                    "Demand based on price changed to ",
                    service.calculate_demand(demand)
                );
                */
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
            }
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
            }
            PlayerAction::AddNode => {
                // check cost
                if state.funds < BARE_NODE_COST {
                    gloo_console::warn!("Not enough funds to purchase a new node");
                    return;
                }
                // note: whether there is space for the new node
                // is determined elsewhere

                state.funds -= BARE_NODE_COST;

                let id = state.nodes.len() as u32;
                state.nodes.push(CloudNode::new(id));
            }
            PlayerAction::AddUpgradedNode => {
                // check cost
                if state.funds < UPGRADED_NODE_COST {
                    gloo_console::warn!("Not enough funds to purchase a new node");
                    return;
                }
                // note: whether there is space for the new node
                // is determined elsewhere

                state.funds -= UPGRADED_NODE_COST;

                let id = state.nodes.len() as u32;
                state.nodes.push(CloudNode::new_fully_upgraded(id));
            }
            PlayerAction::UseCard { id } => {
                // 1. find the card
                match ALL_CARDS.binary_search_by_key(&id.as_ref(), |c| &c.id) {
                    Ok(index) => {
                        let card = &ALL_CARDS[index];
                        // 2. deduct its cost
                        let cost = &card.cost;
                        if !state.can_afford(cost) {
                            gloo_console::warn!("Invalid card purchase attempted:", card.id);
                            return;
                        }
                        state.apply_cost(cost);
                        // 3. apply the card's effects
                        self.apply_card(state, card);
                        // 4. add the card to the used cards list
                        // (but only if the card was actually applied)
                        let time = state.time;
                        state.cards_used.push(UsedCard {
                            id: id.clone(),
                            time,
                        });
                        // keep cards_used sorted by ID
                        state
                            .cards_used
                            .sort_unstable_by(|c1, c2| c1.id.cmp(&c2.id));
                    }
                    Err(_) => {
                        // warn
                        gloo_console::warn!("Bad card identifier ", id.as_ref());
                    }
                }
            }
        }
    }

    fn apply_card(&mut self, state: &mut WorldState, card: &CardSpec) {
        self.apply_card_effect(state, &card.effect)
    }

    fn apply_card_effect(&mut self, state: &mut WorldState, effect: &CardEffect) {
        match effect {
            CardEffect::Nothing => { /* no op */ }
            CardEffect::UnlockDemandEstimate => {
                state.can_see_demand = true;
            }
            CardEffect::UnlockService(kind) => {
                let service = state.service_by_kind_mut(*kind);
                service.unlocked = true;
                service.private = true;
            }
            CardEffect::PublishService(kind) => {
                let service = state.service_by_kind_mut(*kind);
                service.private = false;

                // if service is base, add base publicity
                // (it means that the game has just started)
                if *kind == ServiceKind::Base {
                    state.demand = 1.;
                }

                // add user specification for this service
                if state.user_specs.iter().all(|spec| spec.service != *kind) {
                    state.user_specs.push(CloudUserSpec {
                        id: state.next_user_spec_id(),
                        service: *kind,
                        bad: false,
                        trial_time: 0,
                    });

                    let user_spec = &state.user_specs[state.user_specs.len() - 1];
                    self.bootstrap_events_for(state, user_spec);
                }
                // add DoS specification for this service
                // if there is high demand
                if state.demand > DEMAND_DOS_THRESHOLD {
                    if state
                        .user_specs
                        .iter()
                        .all(|spec| spec.service != *kind && spec.bad)
                    {
                        state.user_specs.push(CloudUserSpec {
                            id: state.next_user_spec_id(),
                            service: *kind,
                            bad: true,
                            trial_time: 0,
                        });
                        let user_spec = &state.user_specs[state.user_specs.len() - 1];
                        self.bootstrap_events_for(state, user_spec);
                    }
                }
            }
            CardEffect::UpgradeEntitlements(service, money) => {
                let service = state.service_by_kind_mut(*service);
                service.entitlement = service.entitlement.max(*money);
            }
            CardEffect::SetElectricityCostLevel(level) => {
                state.electricity.cost_level = state.electricity.cost_level.max(*level);
                if *level == 4 {
                    // bonus ads at level 4 (clean energy plan)
                    state.demand += 5_000.;
                    state.demand_rate += 32.;
                } else if *level == 6 {
                    // bonus ads at level 6 (free energy)
                    state.demand += 50_000.;
                    state.demand_rate += 64.;
                }
            }
            CardEffect::UpgradeOpsPerClick(amount) => {
                state.ops_per_click = state.ops_per_click.max(*amount);
            }
            CardEffect::AddFunds(money) => {
                state.funds += *money;
            }
            CardEffect::AddClients(spec) => {
                state.user_specs.push(CloudUserSpec {
                    id: state.next_user_spec_id(),
                    service: spec.service,
                    trial_time: state.time + spec.trial_duration as u64,
                    bad: false,
                });
                let user_spec = &state.user_specs[state.user_specs.len() - 1];
                self.bootstrap_events_for(state, user_spec);
            }
            CardEffect::AddClientsWithPublicity(spec, demand_delta) => {
                state.demand += demand_delta;

                state.user_specs.push(CloudUserSpec {
                    id: state.next_user_spec_id(),
                    service: spec.service,
                    trial_time: if spec.trial_duration > 0 {
                        state.time + spec.trial_duration as u64
                    } else {
                        0
                    },
                    bad: false,
                });
                let user_spec = &state.user_specs[state.user_specs.len() - 1];
                self.bootstrap_events_for(state, user_spec);
            }
            CardEffect::AddPublicityRate(demand_delta, demand_rate_delta) => {
                let was_high_demand = state.demand > DEMAND_DOS_THRESHOLD;
                state.demand += demand_delta;
                state.demand_rate += demand_rate_delta;
                // if demand increased a lot,
                // insert DoS users if not added already
                if !was_high_demand && state.demand > DEMAND_DOS_THRESHOLD {
                    for service in [
                        ServiceKind::Base,
                        ServiceKind::Super,
                        ServiceKind::Epic,
                        ServiceKind::Awesome,
                    ] {
                        if state
                            .user_specs
                            .iter()
                            .all(|spec| spec.service != service && spec.bad)
                        {
                            state.user_specs.push(CloudUserSpec {
                                id: state.next_user_spec_id(),
                                service,
                                bad: true,
                                trial_time: 0,
                            });
                            let user_spec = &state.user_specs[state.user_specs.len() - 1];
                            self.bootstrap_events_for(state, user_spec);
                        }
                    }
                }
            }
            CardEffect::UpgradeServices => {
                state.software_level += 1;
                // refresh memory reserves
                // might be reserving too much
                let maximum_reserve = state.expected_ram_reserved();
                for node in state.nodes.iter_mut() {
                    node.release_excess_reserve(maximum_reserve);
                }
            }
            CardEffect::MoreCaching => {
                state.cache_level += 1;
            }
            CardEffect::UnlockMultiNodes => {
                state.can_buy_nodes = true;
            }
            CardEffect::UnlockMultiRacks => {
                state.can_buy_racks = true;
            }
            CardEffect::UnlockMultiDatacenters => {
                state.can_buy_datacenters = true;
            }
            CardEffect::UpgradeSpamProtection(rate) => {
                state.spam_protection = state.spam_protection.max(*rate);
            }
            CardEffect::UpgradeRoutingLevel(level) => {
                state.routing_level = state.routing_level.max(*level);
            }
        }
    }

    /// Initiate request arrival events based on the current world state
    pub fn bootstrap_events(&mut self, state: &WorldState) {
        let time = state.time;
        for user_spec in state.user_specs.iter() {
            // calculate demand based on base demand and cloud service price
            let service = match user_spec.service {
                crate::ServiceKind::Base => &state.base_service,
                crate::ServiceKind::Super => &state.super_service,
                crate::ServiceKind::Epic => &state.epic_service,
                crate::ServiceKind::Awesome => &state.awesome_service,
            };

            let demand = service.calculate_demand(state.demand);
            let duration = self.gen.next_request(demand);
            let timestamp = time + duration as u64;
            let amount = 1;
            self.queue.push(RequestEvent::new_arrived(
                timestamp,
                Some(user_spec.id),
                amount,
                user_spec.service,
                user_spec.bad,
            ));
        }
    }

    /// Initiate request arrival events for the given cloud user specification
    pub fn bootstrap_events_for(&mut self, state: &WorldState, user_spec: &CloudUserSpec) {
        let time = state.time;
        // calculate demand based on base demand and cloud service price
        let service = match user_spec.service {
            crate::ServiceKind::Base => &state.base_service,
            crate::ServiceKind::Super => &state.super_service,
            crate::ServiceKind::Epic => &state.epic_service,
            crate::ServiceKind::Awesome => &state.awesome_service,
        };

        let demand = service.calculate_demand(state.demand);
        let amount = 1;
        let duration = self.gen.next_request(demand);
        let timestamp = time + duration as u64;
        self.queue.push(RequestEvent::new_arrived(
            timestamp,
            Some(user_spec.id),
            amount,
            user_spec.service,
            user_spec.bad,
        ));
    }

    /// Process the game state and produce new events.
    pub fn update(&mut self, state: &mut WorldState, time: Time) {
        // process events until the given time
        while let Some(next_event_time) = self.queue.next_event_time() {
            // add a safety net from events
            // which are in the past but were not consumed
            if next_event_time < state.time {
                gloo_console::debug!(
                    "Dropped past event with timestamp",
                    next_event_time,
                    "(this could be a bug)"
                );
                continue;
            }
            if next_event_time > time {
                break;
            }
            let event = self.queue.pop().unwrap();
            self.process_event(state, time, event);
        }

        let duration = time - state.time;

        // check whether to do a major update
        if duration > 0 && time / 2_500 - state.time / 2_500 > 0 {
            // do a major update
            self.update_major(state, time);
        }

        // update time
        state.time = time;
    }

    /// Do a major update, which performs heavier stuff periodically.
    fn update_major(&mut self, state: &mut WorldState, time: Time) {
        // check whether to increase demand from time passing by
        if time / INCREASE_DEMAND_PERIOD - state.time / INCREASE_DEMAND_PERIOD > 0 {
            // increase demand a tiny bit
            state.demand += state.demand_rate;
            gloo_console::debug!("Demand increased to ", state.demand);
        }

        // check whether to issue an electricity bill
        if time / ELECTRICITY_BILL_PERIOD - state.time / ELECTRICITY_BILL_PERIOD > 0 {
            // check whether we have enough costs to worth issuing a bill
            let total_cost = state.electricity.check_bill();
            if total_cost > Money::cents(50) {
                // issue an electricity bill
                state.electricity.emit_bill_for(total_cost, time);
            }
        }

        // check whether to save the game
        if time / GAME_SAVE_PERIOD - state.time / GAME_SAVE_PERIOD > 0 {
            // save the game
            state
                .save_game()
                .unwrap_or_else(|e| gloo_console::error!(e));
        }
    }

    /// process a single request event
    fn process_event(&mut self, state: &mut WorldState, time: Time, event: RequestEvent) {
        // closure to add a new event to the main queue
        let mut push_event = |event: RequestEvent| {
            self.queue.push(event);
        };

        match event.kind {
            RequestEventStage::RequestArrived => {
                let powersave = state.is_powersaving();
                // route the request if necessary
                let node_count = state.nodes.len() as u32;
                if node_count == 1 {
                    // immediately route to the only node
                    push_event(event.into_routed(0, 0));
                } else if state.routing_level == RoutingLevel::NoRoutingCost {
                    // immediately route to a random node
                    let node_num = self.gen.gen_range(0, node_count);
                    push_event(event.into_routed(0, node_num));
                } else {
                    // route the request:

                    // check if any node is not busy
                    if state.nodes.iter().all(|node| node.is_busy(powersave)) {
                        // enqueue it unless the waiting queue is too large already
                        if self.waiting_queue.len() > 2_000 {
                            // drop the request
                            state.requests_dropped += event.amount as u64;
                        } else {
                            // enqueue it
                            self.waiting_queue.push_back(WaitingRouteRequest {
                                amount: event.amount,
                                user_spec_id: event.user_spec_id,
                                service: event.service,
                                bad: event.bad,
                            });
                        }
                    } else {
                        // pick a routing node
                        let node_num = if state.routing_level == RoutingLevel::Distributed {
                            loop {
                                let n = self.gen.gen_range(0, node_count);
                                let picked_node = state.node(n).unwrap();
                                // if node is not busy, use it
                                if !picked_node.is_busy(powersave) {
                                    break n;
                                }
                                // otherwise put it on the waiting queue
                            }
                        } else {
                            // always use the first one
                            // until the player gets the upgrade
                            0
                        };
                        // add processing to the routing node
                        let node = state.node_mut(node_num).unwrap();
                        // drop request if node is busy
                        if node.is_busy(powersave) {
                            gloo_console::debug!("node ", node_num, " dropped a request");

                            state.requests_dropped += event.amount as u64;
                        } else {
                            gloo_console::debug!(
                                "node ",
                                node_num,
                                " is routing. processing: ",
                                node.processing
                            );

                            node.processing += 1;
                            let duration = node.time_per_request_routing() * event.amount;

                            // 2. push event to request routed
                            push_event(event.into_routed(duration, node_num));
                        }
                    }
                }

                let mut should_drop_spec = false;
                if let Some(user_spec_id) = event.user_spec_id {
                    // also generate a new request for the upcoming request
                    // from the same client spec
                    if let Some(spec) = &state.user_spec(user_spec_id) {
                        // check trial period
                        if spec.trial_time > time || spec.trial_time == 0 {
                            // determine demand for the service by this spec
                            let service = match spec.service {
                                crate::ServiceKind::Base => &state.base_service,
                                crate::ServiceKind::Super => &state.super_service,
                                crate::ServiceKind::Epic => &state.epic_service,
                                crate::ServiceKind::Awesome => &state.awesome_service,
                            };
                            let demand = service.calculate_demand(state.demand);
                            let amount = 1;
                            let duration = self.gen.next_request(demand);
                            let timestamp = event.timestamp + duration as u64 * event.amount as u64;
                            push_event(RequestEvent::new_arrived(
                                timestamp,
                                event.user_spec_id,
                                amount,
                                spec.service,
                                spec.bad,
                            ));
                        } else if spec.trial_time > 0 {
                            // trial period over
                            should_drop_spec = true;
                        }

                        if should_drop_spec {
                            // clean up unused user spec
                            // (this is safe because the user spec ID is unique)
                            state.user_specs.retain(|spec| spec.id != user_spec_id);
                        }
                    } else {
                        gloo_console::warn!("Invalid user specification ID ", user_spec_id);
                    }
                }
            }
            RequestEventStage::RequestRouted { node_num } => {
                let software_level = state.software_level;
                let cache_level = state.cache_level;
                let powersave = state.is_powersaving();
                let routing_needed =
                    state.nodes.len() > 1 && state.routing_level != RoutingLevel::NoRoutingCost;
                let Some(routing_node) = state.node_mut(node_num) else {
                    return;
                };

                // 1. if required, decrement processing on the routing node
                if routing_needed {
                    if routing_node.processing == 0 {
                        gloo_console::warn!(
                            "Processing count of routing node",
                            routing_node.id,
                            "is zero, there is probably a bug"
                        );
                    } else {
                        routing_node.processing -= 1;
                    }
                    // add small electricity cost
                    if !powersave {
                        state.electricity.add_consumption(0.01);
                    }
                }

                // spam detection
                if event.bad && state.spam_protection > 0. {
                    let detected = self.gen.gen_bool(state.spam_protection);
                    if detected {
                        // dropped intentionally
                        return;
                    }
                };

                // 2. pick a request processing node
                let node_id = self.gen.gen_range(0, state.nodes.len() as u32);

                // 3. check memory reserve requirement
                let mem_reserve_required = Self::calculate_memory_reserve_required(
                    event.service,
                    state.cache_level,
                    state.software_level,
                );
                let node = state.node_mut(node_id).unwrap();

                if !node.reserve_for(mem_reserve_required) {
                    // can't reserve, drop the request
                    state.requests_dropped += event.amount as u64;
                    return;
                }

                // 4. check memory requirement for request
                let mem_required = node.ram_per_request * event.amount as i32;
                if mem_required > node.ram_capacity - node.ram_usage {
                    // 4.1. if not enough memory, drop the request.
                    state.requests_dropped += event.amount as u64;
                    return;
                }
                // 5. add memory usage to the processing node
                node.ram_usage += mem_required;

                // 6. if node has a CPU available,
                if !node.is_busy(powersave) {
                    // calculate time to process the request
                    let mut duration =
                        node.time_per_request(event.service, software_level) * event.amount;

                    // if in powersave mode, make it slower
                    if powersave {
                        duration *= 4;
                    }

                    // test whether this request will hit the cache
                    let cache_rate = CACHE_LEVELS[cache_level as usize].1;
                    let cache_hit = self.gen.gen_bool(cache_rate);
                    if cache_hit {
                        // make it much faster
                        duration = (duration / 20).min(1);
                    }

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
            RequestEventStage::RequestProcessed {
                node_num,
                ram_required,
            } => {
                let routing_level = state.routing_level;
                let software_level = state.software_level;
                if state.node(node_num).is_none() {
                    return;
                };

                // 1. add electricity consumption
                if !state.is_powersaving() {
                    state.electricity.add_consumption(1.);
                }

                // 2. increment op counts (available & total)
                let service = state.service_by_kind_mut(event.service);
                if !event.bad {
                    service.total += Ops(event.amount as i64);
                    service.available += Ops(event.amount as i64);
                }
                let service_price = service.price;
                let service_entitlement = service.entitlement;

                // 3. calculate revenue if applicable
                let revenue = if !event.bad {
                    if let Some(id) = event.user_spec_id {
                        if let Some(spec) = &state.user_spec(id) {
                            if spec.is_paying(time) {
                                service_price * event.amount as i32 + service_entitlement
                            } else {
                                // within trial period
                                service_entitlement
                            }
                        } else {
                            // specification was deleted,
                            // treat it as clicked by player
                            service_entitlement
                        }
                    } else {
                        // player request
                        service_entitlement
                    }
                } else {
                    // bad request
                    Money::zero()
                };

                let node = state.node_mut(node_num).unwrap();
                // 4. decrement memory usage
                node.ram_usage -= ram_required;

                let node_num = node.id;

                // 5. if there are routing requests waiting
                if !self.waiting_queue.is_empty()
                    && (node_num == 0 || routing_level == RoutingLevel::Distributed)
                {
                    let request = self.waiting_queue.pop_front().unwrap();
                    // pop one and route the request now using this node
                    let duration = node.time_per_request_routing() * request.amount;

                    // push event to request routed
                    push_event(RequestEvent {
                        timestamp: event.timestamp + duration as Time,
                        user_spec_id: request.user_spec_id,
                        amount: request.amount,
                        service: request.service,
                        bad: request.bad,
                        kind: RequestEventStage::RequestRouted { node_num },
                    });
                } else if let Some(request) = node.requests.pop_front() {
                    // pop one and schedule a new request processed event
                    let duration =
                        node.time_per_request(event.service, software_level) * request.amount;

                    let service = request.service;
                    let bad = if let Some(id) = request.user_spec_id {
                        state.user_spec(id).map(|spec| spec.bad).unwrap_or(false)
                    } else {
                        false
                    };
                    push_event(RequestEvent {
                        timestamp: event.timestamp + duration as u64,
                        user_spec_id: request.user_spec_id,
                        amount: request.amount,
                        service,
                        bad,
                        kind: RequestEventStage::RequestProcessed {
                            node_num,
                            ram_required,
                        },
                    })
                } else {
                    // decrement processing on the processing node
                    if node.processing == 0 {
                        gloo_console::warn!(
                            "Processing count of node",
                            node.id,
                            "is zero, there is probably a bug"
                        );
                    } else {
                        node.processing -= 1;
                    }
                }

                // apply revenue
                if revenue > Money::zero() {
                    state.funds += revenue;
                    state.earned += revenue;
                }
                // apply bad request count
                if event.bad {
                    state.requests_failed += event.amount as u64;
                }
            }
        }
    }

    fn calculate_memory_reserve_required(
        service: ServiceKind,
        cache_level: u8,
        software_level: u8,
    ) -> Memory {
        (match service {
            ServiceKind::Base => BASE_MEMORY_RESERVE,
            ServiceKind::Super => SUPER_MEMORY_RESERVE,
            ServiceKind::Epic => EPIC_MEMORY_RESERVE,
            ServiceKind::Awesome => AWESOME_MEMORY_RESERVE,
        }) * CACHE_LEVELS[cache_level as usize].0
            * SOFTWARE_LEVELS[software_level as usize].1
    }
}

/// A request (or request set) waiting to be routed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaitingRouteRequest {
    /// multiplier for the number of requests
    /// bundled into one
    amount: u32,

    /// the cloud user specification ID
    /// (or None if it was requested by the player)
    user_spec_id: Option<u32>,

    /// the request's cloud service kind
    service: ServiceKind,

    /// whether the request is bad
    bad: bool,
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
    /// the amount of RAM required to process
    /// a base request in one of the cores
    pub ram_per_request: Memory,

    /// the number of requests currently being processed right now
    ///
    /// Transient.
    #[serde(skip)]
    pub processing: u32,

    /// the total amount of RAM in use
    ///
    /// Transient.
    #[serde(skip, default = "Memory::zero")]
    pub ram_usage: Memory,

    /// how much of `ram_usage` is reserved
    ///
    /// Transient.
    #[serde(skip, default = "Memory::zero")]
    pub ram_reserved: Memory,

    /// queue of requests sitting in memory and waiting to be processed
    ///
    /// Transient.
    #[serde(skip)]
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
            ram_reserved: Memory::zero(),
            requests: VecDeque::new(),
        }
    }

    pub fn new_fully_upgraded(id: u32) -> Self {
        Self {
            id,
            cpu_level: CPU_LEVELS.len() as u8 - 1,
            ram_level: RAM_LEVELS.len() as u8 - 1,
            num_cores: CPU_LEVELS[CPU_LEVELS.len() - 1].0,
            ram_capacity: RAM_LEVELS[RAM_LEVELS.len() - 1].0,
            cpu_speed: CPU_LEVELS[CPU_LEVELS.len() - 1].1,
            ram_per_request: Memory::kb(512),
            processing: 0,
            ram_usage: Memory::zero(),
            ram_reserved: Memory::zero(),
            requests: VecDeque::new(),
        }
    }

    /// Calculate the time units needed to process the request,
    /// based on service kind and other global parameters
    pub(crate) fn time_per_request(&self, service: ServiceKind, software_level: u8) -> u32 {
        let factor = match service {
            ServiceKind::Base => 1,
            ServiceKind::Super => 4,
            ServiceKind::Epic => 16,
            ServiceKind::Awesome => 64,
        };

        let software = software_level as u32;
        2_500 * factor / self.cpu_speed + (4_500 / (software * software + 1))
    }

    pub(crate) fn time_per_request_routing(&self) -> u32 {
        256 / self.cpu_speed
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

    /// Ensure that the node has enough memory reserved,
    /// and update `ram_reserved` if possible.
    ///
    /// This will not release memory already reserved.
    ///
    /// Returns false if the node does not have enough memory,
    /// in which case no changes are made.
    pub fn reserve_for(&mut self, memory: Memory) -> bool {
        let available = self.ram_capacity + self.ram_reserved - self.ram_usage;
        if available < memory {
            false
        } else {
            if self.ram_reserved < memory {
                self.release_reserved();
                self.ram_reserved = memory;
                self.ram_usage += self.ram_reserved;
            }
            true
        }
    }

    /// Release reserved memory,
    /// reclaiming it back as available.
    pub(crate) fn release_reserved(&mut self) {
        self.ram_usage -= self.ram_reserved;
        self.ram_reserved = Memory::zero();
    }

    /// Release some memory so that
    /// the node has at most `maximum_reserve` reserved.
    pub(crate) fn release_excess_reserve(&mut self, maximum_reserve: Memory) -> bool {
        if self.ram_reserved > maximum_reserve {
            // check difference
            let mem_diff = self.ram_reserved - maximum_reserve;
            self.ram_reserved -= mem_diff;
            self.ram_usage -= mem_diff;
            true
        } else {
            false
        }
    }

    /// Check whether this node cannot process any more requests in parallel
    /// at this time.
    pub(crate) fn is_busy(&self, powersave: bool) -> bool {
        if self.processing > self.num_cores {
            gloo_console::warn!("Cloud node ", self.id, " is over its capacity!");
        }

        if powersave {
            self.processing >= self.num_cores / 4
        } else {
            self.processing >= self.num_cores
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
