use std::vec::Vec;

extern crate chrono;
use chrono::{DateTime, Duration, Utc};

extern crate nanoid;
use nanoid::nanoid;

use serde::{Serialize, Deserialize};

pub trait HasDuration {
    fn duration(&self) -> Duration;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeSegment {
    pub begin: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub running: bool,
}
impl TimeSegment {
    pub fn new(begin: DateTime<Utc>, end: DateTime<Utc>) -> TimeSegment {
        TimeSegment { begin, end, running: false }
    }
}
impl HasDuration for TimeSegment {
    fn duration(&self) -> Duration {
        self.end - self.begin
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    pub id: String,
    pub segments: Vec<TimeSegment>,
    pub running: bool,
}
impl Timer {
    pub fn new() -> Timer {
        let id = nanoid!();
        Timer { id, segments: Vec::new(), running: false}
    }
    // TODO: rewrite for concurrency, use RwLock?
    pub fn start(&mut self) -> Option<&TimeSegment> {
        if !self.running {
            self.segments.push(TimeSegment{
                begin: Utc::now(),
                end: Utc::now(),
                running: true
            });
            self.running = true;
            self.segments.last()
        } else {
            None
        }
    }
    pub fn stop(&mut self) -> Option<&TimeSegment> {
        if self.running {
            self.segments.last_mut()?.end = Utc::now();
            self.segments.last_mut()?.running = false;
            self.running = false;
        }
        self.segments.last()
    }
}
impl HasDuration for Timer {
    fn duration(&self) -> Duration {
        let mut duration = Duration::zero();
        for segment in &self.segments {
            duration = duration + segment.duration();
        }
        duration
    }
}

