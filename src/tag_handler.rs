use serde::{Serialize, Deserialize};

use std::time::Duration as StdDuration;

use crate::time_handler::HasDuration;
use crate::user_handler::Id;

use std::collections::BTreeMap;
pub struct Collection {
    attrs: BTreeMap::<Id, f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    /* TODO
     * implement all the tag stuff
     */
    pub id: Id,
    pub name: String,
}
impl Tag {
    pub fn new(name: String) -> Self {
        Tag { id: Id::new(), name }
    }
}
impl HasDuration for Tag {
    fn duration(&self) -> StdDuration {
        // TODO
        StdDuration::new(0, 0)
    }
}
