// http://gameprogrammingpatterns.com/game-loop.html

use std::cmp;
use std::thread::sleep;
use std::time::{Duration, Instant};

static BILLION: u64 = 1_000_000_000;
const DEFAULT_FIXED_UPDATE_PER_SECOND: u64 = 120;
const DEFAULT_UPDATE_PER_SECOND: u64 = 60;

fn ns_to_duration(ns: u64) -> Duration {
    let secs = ns / BILLION;
    let nanos = (ns % BILLION) as u32;
    Duration::new(secs, nanos)
}

#[derive(Debug)]
pub enum LifecycleEvent {
    FixedUpdate(std::time::Duration),
    Update(std::time::Duration),
}

enum State {
    Schedule,
    FixedUpdate,
    Update,
}

pub struct Lifecycle {
    state: State,
    last_fixed_update_time: Instant,
    last_update_time: Instant,
    fixed_update_frequency: Duration,
    update_frequency: Duration,
}

impl Default for Lifecycle {
    fn default() -> Self {
        let start = Instant::now();

        Lifecycle {
            state: State::FixedUpdate,
            last_fixed_update_time: start,
            last_update_time: start,
            fixed_update_frequency: ns_to_duration(BILLION / DEFAULT_FIXED_UPDATE_PER_SECOND),
            update_frequency: ns_to_duration(BILLION / DEFAULT_UPDATE_PER_SECOND),
        }
    }
}

#[allow(clippy::should_implement_trait)]
impl Lifecycle {
    pub fn new() -> Lifecycle {
        Default::default()
    }

    pub fn next(&mut self) -> Option<LifecycleEvent> {
        loop {
            self.state = match self.state {
                State::Schedule => {
                    let current_time = Instant::now();
                    let next_update = self.last_update_time + self.update_frequency;
                    let next_fixed_update =
                        self.last_fixed_update_time + self.fixed_update_frequency;
                    let next_event = cmp::min(next_update, next_fixed_update);

                    if next_event > current_time {
                        sleep(next_event - current_time);
                        State::Schedule
                    } else if next_event == next_update {
                        State::Update
                    } else {
                        // next_event == next_fixed_update
                        State::FixedUpdate
                    }
                }
                State::FixedUpdate => {
                    let current_time = Instant::now();
                    let delta_time = current_time - self.last_fixed_update_time;

                    self.last_fixed_update_time = current_time;
                    self.state = State::Schedule;
                    return Some(LifecycleEvent::FixedUpdate(delta_time));
                }
                State::Update => {
                    let current_time = Instant::now();
                    let delta_time = current_time - self.last_update_time;

                    self.last_update_time = current_time;
                    self.state = State::Schedule;
                    return Some(LifecycleEvent::Update(delta_time));
                }
            };
        }
    }
}
