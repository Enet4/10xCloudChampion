//! Module for the active entity in the game,
//! which takes the current state of the program
//! and processes it over time.

use crate::Memory;

use super::queue::RequestEventQueue;

#[derive(Debug)]
pub struct Engine {
    /// the event queue
    queue: RequestEventQueue,
    /// all computational racks
    racks: Vec<CloudRack>,
}

/// A cloud processing node and its state
#[derive(Debug)]
pub struct CloudNode {
    /// the number of requests that it can fulfill in parallel
    pub num_cores: u32,
    /// the amount of memory that it has
    pub memory: Memory,

    /// the ticks required to process a request in one of the cores
    pub ticks_per_request: u32,

    /// the number of requests currently being processed right now
    pub processing: u32,
    /// the number of requests that can be processed at once
    pub capacity: u32,
}

/// A rack containing multiple nodes
#[derive(Debug)]
pub struct CloudRack {
    pub nodes: Vec<CloudNode>,
    /// the maximum capacity of a single rack
    pub capacity: u8,
}
