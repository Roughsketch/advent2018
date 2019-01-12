use std::cmp::Ordering;
use crate::time::Time;

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    Shift(i32),
    Sleep,
    Wake,
}

#[derive(Debug, Eq, PartialEq)]
pub struct LogEntry {
    pub time: Time,
    pub event: Event,
}

impl Ord for LogEntry {
    fn cmp(&self, other: &LogEntry) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &LogEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl LogEntry {
    pub fn new(entry: &str) -> Self {
        let split = entry.split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let time = Time::new(
            split[0],   //  year
            split[1],   //  month
            split[2],   //  day
            split[3],   //  hour
            split[4],   //  minute
        );

        let event = if split.len() == 6 {
            Event::Shift(split[5])
        } else if entry.contains("wakes up") {
            Event::Wake
        } else if entry.contains("falls asleep") {
            Event::Sleep
        } else {
            unreachable!("Invalid event: {:?}", entry);
        };

        Self {
            time, event
        }
    }
}