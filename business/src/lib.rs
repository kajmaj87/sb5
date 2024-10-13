mod entities;
mod value_objects;
mod events;
mod event_store;
pub mod api;
mod scripting;

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use super::entities::entity::Entity;
    use super::value_objects::location::Location;
    use super::value_objects::id::Id;
    use super::event_store::EventStore;

    #[test]
    fn test_entity_movement_and_event_store() {
        let entity_id = Id::new();
        let initial_location = Location::new(0, 0);
        let new_location = Location::new(10, 10);

        let event_store = Arc::new(Mutex::new(EventStore::new()));
        let mut entity = Entity::new(entity_id.clone(), initial_location.clone(), event_store.clone());

        assert_eq!(entity.location(), &initial_location);

        entity.change_location(new_location.clone());
        assert_eq!(entity.location(), &new_location);

        // Check events in the event store
        let binding = event_store.lock().unwrap();
        let stored_events = binding.get_events();
        assert_eq!(stored_events.len(), 1);
        if let Some(event) = stored_events.front() {
            if let super::events::event::Event::LocationChanged(e) = event {
                assert_eq!(e.old_location, initial_location);
                assert_eq!(e.new_location, new_location);
                assert_eq!(e.entity_id, entity_id);
            } else {
                panic!("Expected LocationChanged event");
            }
        } else {
            panic!("Expected one event in the event store");
        }
    }
}