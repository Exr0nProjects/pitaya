use serde::{Serialize, Deserialize};

extern crate fuzzy_matcher;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use std::vec::Vec;
use std::sync::{Arc, RwLock};

use crate::time_handler::Timer;
use crate::tag_handler::Tag;

use std::fmt;
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq)]
pub struct Id(pub u64);
impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Id(disp) = self;
        write!(f, "{}", disp % 10000)
    }
}
use rand_chacha::{ChaCha20Rng, rand_core::{SeedableRng, RngCore}};
pub struct IdGen {
    generator: ChaCha20Rng,
}
impl IdGen {
    pub fn new() -> Self {
        IdGen { generator: ChaCha20Rng::from_entropy() }
    }
    pub fn next(&mut self) -> Id {
        Id(self.generator.next_u64())
    }
}
impl std::default::Default for IdGen {
    fn default() -> Self {
        Self::new()
    }
}
impl core::ops::FnMut<()> for IdGen {
    extern "rust-call" fn call_mut(&mut self, args: ()) -> () {
        self.next()
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserSpace {
    #[serde(skip)]
    id_gen: IdGenerator,
    timers: Vec<Timer>,
    tags: Vec<Tag>,
}
impl UserSpace {
    pub fn new() -> Self {
        UserSpace {
            id_gen: IdGenerator::new(),
            timers: Vec::new(),
            tags: Vec::new(),
        }
    }
    pub fn timers(&self) -> &Vec<Timer> { &self.timers }
    pub fn tags(&self) -> &Vec<Tag> { &self.tags }
    pub fn new_timer(&mut self) -> &Timer {
        self.timers.push(Timer::new(self.id_gen.next()));
        self.timers.last().unwrap()
    }
    pub fn new_tag(&mut self, name: String) -> &Tag {
        self.tags.push(Tag::new(self.id_gen.next(), name));
        self.tags.last().unwrap()
    }
    pub fn get_stats(&mut self) {
        /* TODO
         * 1. store two vectors or deques of timers, one that needs syncing and one that doesn't
         * 2. go through timers that need syncing, traverse tag tree and add to their deque
         * 3. parallelize: for each tag add all the accumulatables (time and count for now)
         */
    }
}
impl std::default::Default for UserSpace {
    fn default() -> Self {
        Self::new()
    }
}
