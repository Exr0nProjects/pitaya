
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map, Number};

#[derive(Serialize, Deserialize, Hash, PartialEq)]
pub struct Stats {
    data: Map,
}
impl Stats {
    fn combine(&mut self, other: &Stats) -> &mut Self {
        // TODO: combine stats by addition
        &mut self
        // TODO (future): combine stats based on enum (multiply, average, etc)
    }
}
impl Eq for Stats {}

