
use serde_json::{Value, Map, Number};

pub struct Stats {
    data: Map,
}
impl Stats {
    fn combine(&mut self, other: &Stats) -> &mut Self {
        // TODO: combine stats by addition
        // TODO (future): combine stats based on enum (multiply, average, etc)
    }
}

