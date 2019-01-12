use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct Time {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub min: i32,
}

impl Ord for Time {
    fn cmp(&self, other: &Time) -> Ordering {
        let cmp = self.year.cmp(&other.year);
        if cmp != Ordering::Equal {
            return cmp;
        }

        let cmp = self.month.cmp(&other.month);
        if cmp != Ordering::Equal {
            return cmp;
        }

        let cmp = self.day.cmp(&other.day);
        if cmp != Ordering::Equal {
            return cmp;
        }

        let cmp = self.hour.cmp(&other.hour);
        if cmp != Ordering::Equal {
            return cmp;
        }

        self.min.cmp(&other.min)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Time {
    pub fn new(year: i32, month: i32, day: i32, hour: i32, min: i32) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            min,
        }
    }
}