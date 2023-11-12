//! The event queue.
//!

use std::collections::VecDeque;

use super::stuff::ServiceKind;

/// type for an absolute time measure.
pub type Time = u64;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RequestEvent {
    pub timestamp: Time,
    /// unique identifier (index) to the cloud user specification
    /// (or `None` if the request was triggered by the player)
    pub user_spec_id: Option<u32>,
    pub amount: u32,
    pub service: ServiceKind,
    /// whether it was a bad request that will not fulfill anything
    pub bad: bool,
    /// the request event stage
    pub kind: RequestEventStage,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RequestEventStage {
    /// a request or request set has just arrived at the system.
    /// if more than one node is available,
    /// routing may still be necessary
    RequestArrived,
    /// the request (or request set) has been routed to a node
    /// and is now being processed
    RequestRouted { node_num: u32 },
    /// a node finished processing the request (or request set)
    RequestProcessed { node_num: u32 },
}

impl RequestEvent {
    pub fn new_arrived(
        timestamp: Time,
        user_spec_id: Option<u32>,
        amount: u32,
        service: ServiceKind,
        bad: bool,
    ) -> Self {
        Self {
            timestamp,
            user_spec_id,
            amount,
            service,
            bad,
            kind: RequestEventStage::RequestArrived,
        }
    }

    pub fn into_routed(self, duration: u32, node_num: u32) -> Self {
        Self {
            timestamp: self.timestamp + duration as u64,
            user_spec_id: self.user_spec_id,
            amount: self.amount,
            service: self.service,
            bad: self.bad,
            kind: RequestEventStage::RequestRouted { node_num },
        }
    }
}

#[derive(Debug)]
pub struct RequestEventQueue {
    /// the time of the last process tick
    last_time: Time,
    queue: VecDeque<RequestEvent>,
}

impl RequestEventQueue {
    pub fn new() -> Self {
        Self {
            last_time: 0,
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, event: RequestEvent) {
        // sorted insertion using binary search
        let index = self
            .queue
            .binary_search_by(|probe| probe.timestamp.cmp(&event.timestamp));
        match index {
            Ok(index) => self.queue.insert(index, event),
            Err(index) => self.queue.insert(index, event),
        }
    }

    /// get the list of all events until the given tick,
    /// updating the last tick
    pub fn events_until(&mut self, tick: Time) -> Vec<RequestEvent> {
        let mut events = Vec::new();
        while let Some(event) = self.queue.front() {
            if event.timestamp > tick {
                break;
            }
            events.push(self.queue.pop_front().unwrap());
        }
        self.last_time = tick;
        events
    }

    pub fn last_time(&self) -> Time {
        self.last_time
    }
}
