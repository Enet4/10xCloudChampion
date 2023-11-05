//! The event queue.
//!

use std::collections::VecDeque;

use super::stuff::ServiceKind;

/// type for the time counter
pub type Tick = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct RequestEvent {
    timestamp: Tick,
    amount: u32,
    service: ServiceKind,
    kind: RequestEventKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RequestEventKind {
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

#[derive(Debug)]
pub struct RequestEventQueue {
    /// the last processed tick
    last_tick: Tick,
    queue: VecDeque<RequestEvent>,
}

impl RequestEventQueue {
    pub fn new() -> Self {
        Self {
            last_tick: 0,
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

    /// Retrieve a single event from the queue
    pub fn pop(&mut self) -> Option<RequestEvent> {
        self.queue.pop_front()
    }

    /// get the list of all events until the given tick,
    /// updating the last tick
    pub fn events_until(&mut self, tick: Tick) -> Vec<RequestEvent> {
        let mut events = vec![];
        while let Some(event) = self.pop() {
            if event.timestamp > tick {
                self.push(event);
                break;
            }
            events.push(event);
        }
        self.last_tick = tick;
        events
    }

    pub fn last_tick(&self) -> Tick {
        self.last_tick
    }
}
