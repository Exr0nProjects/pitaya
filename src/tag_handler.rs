extern crate nanoid;
use nanoid::nanoid;

use std::time::Duration as StdDuration;

use crate::time_handler::HasDuration;

pub struct Tag {
    /* TODO
     * implement all the tag stuff
     */
    pub id: String,
    pub name: String,
}
impl Tag {
    pub fn new(name: String) -> Self {
        Tag { id: nanoid!(), name }
    }
}
impl HasDuration for Tag {
    fn duration(&self) -> StdDuration {
        // TODO
        StdDuration::new(0, 0)
    }
}
