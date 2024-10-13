use crate::events::location_changed::LocationChanged;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    LocationChanged(LocationChanged),
}