extern crate chrono;
use std::time::Duration as StdDuration;

use crate::time_handler::HasDuration;

pub struct Tag {
    /* TODO
     * implement all the tag stuff
     */
}
impl HasDuration for Tag {
    fn duration(&self) -> StdDuration {
        // TODO
        StdDuration::new(0, 0)
    }
}
