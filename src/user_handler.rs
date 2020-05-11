use serde::{Serialize, Deserialize};

extern crate fuzzy_matcher;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use std::{
    vec::Vec,
    sync::{
        RwLock,
        mpsc::{channel, Sender, Receiver},
    },
    thread,
};

use crate::time_handler::Timer;
use crate::tag_handler::Tag;
use crate::stats::Stats;

use std::fmt;
use rand_chacha::{ChaCha8Rng, rand_core::{SeedableRng, RngCore}};
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq)]
pub struct Id(pub u64);
impl Id {
    pub fn new() -> Self {
        Self(ChaCha8Rng::from_entropy().next_u64()) // costs roughly 600ns over constant
    }
}
impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Id(disp) = self;
        write!(f, "{}", disp % 10000)
    }
}
impl Eq for Id {}

fn tag_worker(rx: Receiver<(Id, Stats)>)
{
    // TODO: write processor thread logic: recieve requests, update tag stats
    // TODO: buffer incoming requests
    // TODO: refactor elsewhere
    thread::spawn(move|| {
        for (tag_id, stats) in rx.iter() {
            println!("Requested to add stats {:?} to tag {}", stats, tag_id);
        }
    });
}

pub struct UserSpace {
    timers: Vec<Timer>,
    tags: Vec<Tag>,
    tag_tx: Sender<(Id, Stats)>,
}
impl UserSpace {
    pub fn new() -> Self {
        let (tx, rx) = channel();

        tag_worker(rx);

        UserSpace {
            timers: Vec::new(),
            tags: Vec::new(),
            tag_tx: tx,
        }
    }
    pub fn timers(&self) -> &Vec<Timer> { &self.timers }
    pub fn tags(&self) -> &Vec<Tag> { &self.tags }
    pub fn new_timer(&mut self, name: String) -> &mut Timer {
        self.timers.push(Timer::new(name, self.tag_tx.clone()));
        self.timers.last_mut().unwrap()
    }
    pub fn new_tag(&mut self, name: String) -> &Tag {
        self.tags.push(Tag::new(name));
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
