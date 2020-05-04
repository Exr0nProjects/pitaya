extern crate nanoid;
use nanoid::nanoid;

use std::vec::Vec;

use crate::time_handler::Timer;
use crate::tag_handler::Tag;

pub struct UserSpace {
    id: String,
    timers: Vec<Timer>,
    tags: Vec<Tag>,
}
impl UserSpace {
    pub fn new() -> Self {
        UserSpace { id: nanoid!(), timers: Vec::new(), tags: Vec::new() }
    }
    pub fn new_timer(&mut self) -> &Timer {
        self.timers.push(Timer::new());
        self.timers.last().unwrap()
    }
}
