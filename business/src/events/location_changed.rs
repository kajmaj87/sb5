use crate::value_objects::location::Location;
use serde::{Serialize, Deserialize};
use crate::value_objects::id::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationChanged {
    pub entity_id: Id,
    pub old_location: Location,
    pub new_location: Location,
}