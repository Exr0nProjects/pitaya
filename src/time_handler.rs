use std::fmt;
use std::vec::Vec;

use std::time::Duration as StdDuration;

extern crate chrono;
use chrono::{DateTime, Utc};

extern crate nanoid;
use nanoid::nanoid;

use serde::{Serialize, Deserialize};

pub trait HasDuration {
    fn duration(&self) -> StdDuration;
}

#[derive(Serialize, Deserialize, Hash, Debug)]
pub struct TimeSegment {
    pub id: String,
    pub begin: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
}
impl TimeSegment {
    pub fn new() -> Self {
        TimeSegment { begin: Utc::now(), end: None, id: nanoid!() }
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
                   &self.id[0..6],
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
    pub id: String,
    pub segments: Vec<TimeSegment>,
    pub running: bool,
    duration: StdDuration,
}
impl Timer {
    pub fn new() -> Self {
        let id = nanoid!();
        Timer { id, segments: Vec::new(), running: false, duration: StdDuration::new(0, 0) }
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
                   &self.id[0..6],
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

