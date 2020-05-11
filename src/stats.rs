use serde_json::{Value, Map, Number};
use std::{
    sync::Arc,
    marker::Copy,
};

#[derive(PartialEq, Debug)]
pub struct Stats {
    data: Arc<Map<String, Value>>,
}
impl Stats {
    pub fn new() -> Self {
        // TODO
        Stats {
            data: Arc::new(Map::new()),
        }
    }
    fn combine(self, other: &Stats) -> Self {
        // TODO: combine stats by addition
        self
        // TODO (future): combine stats based on enum (multiply, average, etc)
    }
}
impl Eq for Stats {}
impl Clone for Stats {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}

