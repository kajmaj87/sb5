use std::fmt::Display;
use serde::{Serialize, Deserialize};
use rand::{Rng, distributions::Alphanumeric};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Id(String);

impl Id {
    pub fn new() -> Self {
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}