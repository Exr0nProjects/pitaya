use serde::{Serialize, Deserialize};

use std::time::Duration as StdDuration;

use crate::time_handler::HasDuration;

pub trait Collectable {
    type Attr;
    fn new() -> Self;
    fn collect(&mut self, other: Collectable);
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    /* TODO
     * implement all the tag stuff
     */
    pub id: u64,
    pub name: String,
}
impl Tag {
    pub fn new(id: u64, name: String) -> Self {
        Tag { id, name }
    }
}
impl HasDuration for Tag {
    fn duration(&self) -> StdDuration {
        // TODO
        StdDuration::new(0, 0)
    }
}
