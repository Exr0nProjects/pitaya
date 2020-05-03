extern crate chrono;
use chrono::{DateTime, Duration, Utc};

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
    //pub fn duration(&self) {
    //    println!("Direct duration called");
    //}
}

impl HasDuration for TimeSegment {
    fn duration(&self) -> Duration {
        println!("Duration goes here.");
        self.end - self.begin
    }
}
