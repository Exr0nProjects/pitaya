extern crate nanoid;
use nanoid::nanoid;

extern crate fuzzy_matcher;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

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
    pub fn new_tag(&mut self, name: String) -> &Tag {
        self.tags.push(Tag::new(name));
        self.tags.last().unwrap()
    }
}
