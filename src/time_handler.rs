use std::{
    fmt,
    vec::Vec,
    collections::HashSet,
    sync::mpsc::Sender,
};

use std::time::Duration as StdDuration;
use std::sync::RwLock;
use crate::user_handler::{UserSpace, Id};
use crate::stats::Stats;

extern crate chrono;
use chrono::{DateTime, Utc};

use serde::{Serialize, Deserialize};
use serde_json;

pub trait HasDuration {
    fn duration(&self) -> StdDuration;
}

#[derive(Serialize, Deserialize, Hash, Debug)]
pub struct TimeSegment {
    pub id: Id,
    pub begin: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    stats: Stats,
}
impl TimeSegment {
    pub fn new() -> Self {
        TimeSegment {
            begin: Utc::now(),
            end: None,
            id: Id::new(),
            stats: Stats::new(),
        }
    }
    pub fn stop(&mut self) -> StdDuration {
        self.end = Some(Utc::now());
        self.duration()
    }
}
    // trait impls
    impl HasDuration for TimeSegment {
        fn duration(&self) -> StdDuration {
            ((match self.end {
                Some(end) => end,
                None => Utc::now(),
            }) - self.begin).to_std().expect("Duration overflowed!")
        }
    }
    impl fmt::Display for TimeSegment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TimeSegment {{ {} : {} -> {} }}",
                   self.id.to_string(),
                   self.begin.to_rfc2822(),
                   match self.end {
                       Some(end) => end.to_rfc2822(),
                       None => "None".to_string(),
                    }
               )
        }
    }
    impl PartialEq for TimeSegment {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }
    impl Eq for TimeSegment {}


#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    id: Id,
    pub name: String,
    pub tags: HashSet<Id>,  // TODO: how to hash?
    segments: Vec<TimeSegment>,
    running: bool,
    tag_tx: Sender<(&Id, &Stats)>,
}
impl Timer {
    pub fn new(name: String, tag_tx: Sender<(&Id, &Stats)>) -> Self {
        Timer {
            id: Id::new(),
            name,
            tags: HashSet::new(),
            segments: Vec::new(),
            running: false,
            tag_tx,
        }
    }
    // TODO: rewrite for concurrency, use RwLock?
    pub fn start(&mut self) -> Option<&TimeSegment> {
        if !self.running {
            self.segments.push(TimeSegment::new());
            self.running = true;
            self.segments.last()
        } else {
            None
        }
    }
    pub fn stop(&mut self) -> Option<StdDuration> {
        if self.running {
            self.running = false;
            let last = self.segments.last()?;
            let dura = last.stop();

            for tag_id in self.tags.iter() {
                self.tag_tx.send((tag_id, &last.stats));
            }

            Some(dura)
        } else {
            None
        }
    }
    pub fn list_timers(&self) {
        println!("TimeSegments in {}:", self);
        for segment in &self.segments {
            println!("    {}", segment);
        }
    }
    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn segments(&self) -> &Vec<TimeSegment> {
        &self.segments
    }
    pub fn running(&self) -> &bool {
        &self.running
    }
}
    // trait impls
    impl HasDuration for Timer {
        fn duration(&self) -> StdDuration {
            let mut duration = StdDuration::new(0, 0);
            for segment in &self.segments {
                duration = duration + segment.duration();
            }
            duration
        }
    }
    impl fmt::Display for Timer {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Timer {{ {} for {}{}ms }}",
                   self.id.to_string(),
                   self.duration().as_millis(),
                   if self.running { ".." } else { "" }
               )
        }
    }
    impl PartialEq for Timer {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }
    impl Eq for Timer {}

