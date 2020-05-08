use std::fmt;
use std::vec::Vec;

use std::time::Duration as StdDuration;
use std::sync::RwLock;
use crate::user_handler::{UserSpace, Id};

extern crate chrono;
use chrono::{DateTime, Utc};

use serde::{Serialize, Deserialize};

pub trait HasDuration {
    fn duration(&self) -> StdDuration;
}

#[derive(Serialize, Deserialize, Hash, Debug)]
pub struct TimeSegment {
    pub id: Id,
    pub begin: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
}
impl TimeSegment {
    pub fn new(id: Id) -> Self {
        TimeSegment { begin: Utc::now(), end: None, id }
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


#[derive(Serialize, Deserialize, Hash, Debug)]
pub struct Timer {
    pub id: Id,
    pub segments: Vec<TimeSegment>,
    pub running: bool,
    duration: StdDuration,
}
impl Timer {
    pub fn new(id: Id) -> Self {
        Timer { id, segments: Vec::new(), running: false, duration: StdDuration::new(0, 0) }
    }
    // TODO: rewrite for concurrency, use RwLock?
    pub fn start(&mut self, id: Id) -> Option<&TimeSegment> {
        if !self.running {
            self.segments.push(TimeSegment::new(id));
            self.running = true;
            self.segments.last()
        } else {
            None
        }
    }
    pub fn stop(&mut self) -> Option<StdDuration> {
        if self.running {
            self.running = false;
            Some(self.segments.last_mut()?.stop())
        } else {
            Some(self.segments.last()?.duration())
        }
    }
    pub fn list_timers(&self) {
        println!("TimeSegments in {}:", self);
        for segment in &self.segments {
            println!("    {}", segment);
        }
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

