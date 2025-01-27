use std::any::Any;
use std::collections::{BTreeMap, BinaryHeap};

use crate::worlds::{Event, State};

#[derive(Clone)]
pub struct Snapshot {
    timestamp: f64,
    shared_state: Option<State>,
    agent_states: BTreeMap<usize, Option<State>>,
}

impl PartialEq for Snapshot {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for Snapshot {}

impl PartialOrd for Snapshot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Snapshot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.partial_cmp(&other.timestamp).unwrap()
    }
}

pub struct Logger {
    snapshots: BinaryHeap<Snapshot>,
    events: BinaryHeap<Event>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            snapshots: BinaryHeap::new(),
            events: BinaryHeap::new(),
        }
    }

    pub fn log(
        &mut self,
        timestamp: f64,
        shared_state: Option<State>,
        agent_states: BTreeMap<usize, Option<State>>,
        event: Event,
    ) {
        self.snapshots.push(Snapshot {
            timestamp,
            shared_state,
            agent_states,
        });
        self.events.push(event);
    }

    pub fn get_snapshots(&self) -> BinaryHeap<Snapshot> {
        self.snapshots.clone()
    }
}
