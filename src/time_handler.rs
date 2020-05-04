extern crate chrono;
extern crate nanoid;
use chrono::{DateTime, Duration, Utc};

use nanoid::*;

use serde::{Serialize, Deserialize};

pub trait HasDuration {
    fn duration(&self) -> Duration;
}

#[derive(Debug)]
pub struct TimeSegment {
    pub begin: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
impl TimeSegment {
    pub fn new(begin: DateTime<Utc>, end: DateTime<Utc>) -> TimeSegment {
        TimeSegment { begin, end }
    }
}
impl HasDuration for TimeSegment {
    fn duration(&self) -> Duration {
        println!("Duration goes here.");
        self.end - self.begin
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    pub id: i32,
}
impl Timer {
    pub fn print(&self) /*-> Duration*/ {
        //nanoid()
    }
}
