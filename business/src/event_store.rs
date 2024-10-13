use crate::events::event::Event;
use std::collections::VecDeque;

pub struct EventStore {
    events: VecDeque<Event>,
}

impl EventStore {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn get_events(&self) -> &VecDeque<Event> {
        &self.events
    }
}