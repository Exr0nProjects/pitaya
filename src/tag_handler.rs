extern crate chrono;
use chrono::Duration;

use crate::time_handler::HasDuration;

pub struct Tag {
    /* TODO
     * implement all the tag stuff
     */
}
impl HasDuration for Tag {
    fn duration(&self) -> Duration {
        // TODO
        Duration::zero()
    }
}
