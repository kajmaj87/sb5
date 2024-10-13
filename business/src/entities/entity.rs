use std::sync::{Arc, Mutex};
use crate::value_objects::location::Location;
use crate::events::event::Event;
use crate::events::location_changed::LocationChanged;
use crate::event_store::EventStore;
use crate::value_objects::id::Id;

pub struct Entity {
    id: Id,
    location: Location,
    event_store: Arc<Mutex<EventStore>>,
}

impl Entity {
    pub fn new(id: Id, location: Location, event_store: Arc<Mutex<EventStore>>) -> Self {
        Self {
            id,
            location,
            event_store,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
    pub fn change_location(&mut self, new_location: Location) {
        let event = Event::LocationChanged(LocationChanged {
            entity_id: self.id.clone(),
            old_location: self.location.clone(),
            new_location: new_location.clone(),
        });

        self.location = new_location;
        self.event_store.lock().unwrap().add_event(event);
    }
}
