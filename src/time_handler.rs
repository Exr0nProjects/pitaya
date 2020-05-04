use std::fmt;
use std::vec::Vec;

extern crate chrono;
use chrono::{DateTime, Duration, Utc};

extern crate nanoid;
use nanoid::nanoid;

use serde::{Serialize, Deserialize};

use crate::tag_handler::Tag;

pub trait HasDuration {
    fn duration() -> &Duration;
    fn start(&mut self) -> &DateTime<Utc>;
    fn stop(&mut self) -> &Duration;
}

#[derive(Serialize, Deserialize, Hash, Debug)]
pub struct TimeSegment {
    pub id: String,
    pub begin: DateTime<Utc>,
    pub end: DateTime<Utc>,
    dura: Duration,
}
impl TimeSegment {
    pub fn new(begin: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        TimeSegment { begin, end, id: nanoid!(), duration: Duration::zero() }
    }
}
impl HasDuration for TimeSegment {
    fn duration(&self) -> Duration {
        self.end - self.begin
    }
    fn start(&mut self) -> &DateTime<Utc> {
        self.running = true;
        self.begin = Utc::now();
        &self.begin
    }
    fn stop(&mut self) -> &Duration {
        // TODO
    }
}
impl fmt::Display for TimeSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TimeSegment {{ {} : {} -> {} }}",
               &self.id[0..6],
               self.begin.to_rfc2822(),
               self.end.to_rfc2822()
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
}
impl Timer {
    pub fn new() -> Self {
        let id = nanoid!();
        Timer { id, segments: Vec::new(), running: false}
    }
    // TODO: rewrite for concurrency, use RwLock?
    // TODO: move these methods to impl HasDuration
    pub fn start(&mut self) -> Option<&TimeSegment> {
        if !self.running {
            self.segments.push(TimeSegment{
                id: nanoid!(),
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
    pub fn list_timers(&self) {
        println!("TimeSegments in {}:", self);
        for segment in &self.segments {
            println!("    {}", segment);
        }
    }
}
impl HasDuration for Timer {
    fn duration(&self) -> Duration {
        let mut duration = Duration::zero();
        for segment in &self.segments {
            duration = duration + segment.duration();
        }
        if self.segments.last().unwrap().running {
            let pending = Utc::now() - self.segments.last().unwrap().begin;
            duration = duration + pending;
        }
        duration
    }
}
impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Timer {{ {} for {}{}ms }}",
               &self.id[0..6],
               self.duration().num_milliseconds(),
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

