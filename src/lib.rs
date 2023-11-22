//! 10x Cloud Champion component library

pub mod audio;
pub mod central;
pub mod components;
pub mod display;

use std::fmt;

use gloo_timers::callback::Interval;
use rand::SeedableRng;
use rand_distr::Distribution;
use rand_pcg::Pcg32;

pub use crate::central::action::PlayerAction;
pub use crate::central::cloud_user::{CloudClientSpec, CloudUserSpec};
pub use crate::central::queue::Time;
pub use crate::central::state::WorldState;
pub use crate::central::stuff::{Cost, Memory, Money, Ops, ServiceKind};

/// the global period of the game watch interval
pub const MILLISECONDS_PER_CYCLE: u32 = 50;

/// how many time units are in a single millisecond
pub const TIME_UNITS_PER_MILLISECOND: u32 = 10;

/// how many time units are in a single game update cycle
pub const TIME_UNITS_PER_CYCLE: u32 = TIME_UNITS_PER_MILLISECOND * MILLISECONDS_PER_CYCLE;

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

    pub fn stop(&mut self) {
        if let Some(interval) = self.interval.take() {
            interval.cancel();
        }
    }
}

/// Top level game message for the game loop and reacting to player actions.
#[derive(Debug, Clone, PartialEq)]
pub enum GameMsg {
    /// the player performed an action
    Action(PlayerAction),
    /// the game watch ticked,
    /// so the game loop should advance
    Tick,
    /// the game loop should stop
    Pause,
    /// the game loop should resume
    Resume,
}

impl From<PlayerAction> for GameMsg {
    fn from(action: PlayerAction) -> Self {
        GameMsg::Action(action)
    }
}

/// Game construct that produces timed events on demand.
#[derive(Debug)]
pub struct SampleGenerator {
    /// the random number generator
    rng: Pcg32,
}

impl SampleGenerator {
    pub fn new() -> Self {
        SampleGenerator {
            rng: Pcg32::from_entropy(),
        }
    }

    /// Sample when the next request to cloud service is going to be made
    /// based on the given demand for that service.
    ///
    /// Demand is approximately the number of requests per second.
    pub fn next_request(&mut self, demand: f32) -> Time {
        let distribution = rand_distr::Exp::new(demand).unwrap();
        (distribution.sample(&mut self.rng) * 1_000. * TIME_UNITS_PER_MILLISECOND as f32) as Time
    }

    /// Pick a number in the `(low..high)` range (excluding `high`).
    pub fn gen_range(&mut self, low: u32, high: u32) -> u32 {
        rand_distr::Uniform::new(low, high).sample(&mut self.rng)
    }

    /// Pick `true` with the given probability.
    pub fn gen_bool(&mut self, chance: f32) -> bool {
        rand_distr::Uniform::new_inclusive(0., 1.).sample(&mut self.rng) < chance
    }
}

impl Default for SampleGenerator {
    fn default() -> Self {
        Self::new()
    }
}
