//! The event queue.
//!

use std::collections::VecDeque;

use crate::Memory;

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
    /// a node finished processing the request (or request set),
    /// and how much RAM in total it was using
    RequestProcessed { node_num: u32, ram_required: Memory },
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

    /// Convert a routed request event
    /// into a processed request event.
    pub fn into_processed(self, node_num: u32, duration: u32, ram_required: Memory) -> Self {
        debug_assert!(
            matches!(self.kind, RequestEventStage::RequestRouted { .. }),
            "cannot process a request that has not been routed"
        );

        Self {
            timestamp: self.timestamp + duration as u64,
            user_spec_id: self.user_spec_id,
            amount: self.amount,
            service: self.service,
            bad: self.bad,
            kind: RequestEventStage::RequestProcessed {
                node_num,
                ram_required,
            },
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

    /// Get the time when the next event will happen,
    /// or `None` if the queue is empty.
    pub fn next_event_time(&self) -> Option<Time> {
        self.queue.front().map(|event| event.timestamp)
    }

    /// Pop the next occurring event from the queue.
    pub fn pop(&mut self) -> Option<RequestEvent> {
        self.queue.pop_front()
    }

    pub fn last_time(&self) -> Time {
        self.last_time
    }
}

#[cfg(test)]
mod tests {
    use super::{RequestEvent, RequestEventQueue};

    #[test]
    fn test_queue() {
        let mut queue = RequestEventQueue::new();

        // add a few events in random order
        queue.push(RequestEvent::new_arrived(
            1000,
            Some(1),
            1,
            crate::ServiceKind::Base,
            false,
        ));

        queue.push(RequestEvent::new_arrived(
            800,
            Some(2),
            1,
            crate::ServiceKind::Base,
            false,
        ));

        queue.push(RequestEvent::new_arrived(
            50,
            Some(3),
            1,
            crate::ServiceKind::Base,
            true,
        ));

        queue.push(RequestEvent::new_arrived(
            2020,
            Some(4),
            1,
            crate::ServiceKind::Super,
            false,
        ));
        queue.push(RequestEvent::new_arrived(
            1620,
            None,
            1,
            crate::ServiceKind::Base,
            false,
        ));

        // check the time of the next event
        assert_eq!(queue.next_event_time(), Some(50));

        // check that all events are popped in order
        let event = queue.pop().unwrap();
        assert_eq!(event.timestamp, 50);
        assert_eq!(event.user_spec_id, Some(3));
        assert_eq!(event.service, crate::ServiceKind::Base);

        let event = queue.pop().unwrap();
        assert_eq!(event.timestamp, 800);
        assert_eq!(event.user_spec_id, Some(2));

        let event = queue.pop().unwrap();
        assert_eq!(event.timestamp, 1000);
        assert_eq!(event.user_spec_id, Some(1));

        let event = queue.pop().unwrap();
        assert_eq!(event.timestamp, 1620);
        assert_eq!(event.user_spec_id, None);

        let event = queue.pop().unwrap();
        assert_eq!(event.timestamp, 2020);
        assert_eq!(event.user_spec_id, Some(4));

        // we're done
        assert_eq!(queue.next_event_time(), None);
    }
}
