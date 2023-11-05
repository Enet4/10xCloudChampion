//! 10x Cloud Champion component library

pub mod central;
pub mod components;
pub mod display;

use std::fmt;

use gloo_timers::callback::Interval;

pub use crate::central::cloud_user::CloudClientSpec;
pub use crate::central::queue::Tick;
pub use crate::central::state::WorldState;
pub use crate::central::stuff::{Cost, Memory, Money, Ops, ServiceKind};

/// the global period of the game watch interval
pub const MILLISECONDS_PER_CYCLE: u32 = 50;
// how many in-game ticks advance per interval cycle
pub const TICKS_PER_CYCLE: u32 = 5;

/// The time watch service, emits ticks at a fixed interval when started.
pub struct GameWatch {
    interval: Option<Interval>,
}

impl fmt::Debug for GameWatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameWatch")
            .field("interval", &self.interval)
            .finish()
    }
}

impl GameWatch {
    pub fn new() -> Self {
        GameWatch { interval: None }
    }

    pub fn start_with<F>(&mut self, tick_fn: F)
    where
        F: 'static + FnMut() + Clone,
    {
        if self.interval.is_some() {
            return;
        }

        let interval = Interval::new(MILLISECONDS_PER_CYCLE, tick_fn);
        self.interval = Some(interval);
    }
}
