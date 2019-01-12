mod entry;
mod time;

use crate::entry::{Event, LogEntry};

pub fn part_1() -> Result<i32, std::io::Error> {
    use std::collections::HashMap;

    let content = std::fs::read_to_string("input/4.txt")?;

    //  Turn challenge input into vec of log entries
    let mut logs = content.lines()
        .map(|line| LogEntry::new(line))
        .collect::<Vec<LogEntry>>();

    //  Sort entries based on date
    logs.sort();
    
    let mut tracker = HashMap::new();
    let mut current_id = 0;
    let mut sleep_start = 0;

    for entry in logs {
        match entry.event {
            //  If new shift starts, change current_id
            Event::Shift(id) => current_id = id,
            //  If sleep starts, mark the minute counter
            Event::Sleep => sleep_start = entry.time.min,
            //  If waking up, insert an entry for every minute slept for that guard id
            Event::Wake => {
                //  Each guard entry is a histogram of time slept
                let guard_entry = tracker.entry(current_id).or_insert(HashMap::new());
                
                //  Insert every minute slept, or add 1 if already exists
                for min in sleep_start..entry.time.min {
                    let e = guard_entry.entry(min).or_insert(0);
                    *e += 1;
                }
            }
        }
    }

    //  Find the guard who slept the longest
    let max = tracker.iter()
        .max_by(|(_, a), (_, b)| {
            //  Compare the total sum of values for each (minutes slept)
            let a_sum: usize = a.iter()
                .map(|(_, v)| v)
                .sum();

            let b_sum: usize = b.iter()
                .map(|(_, v)| v)
                .sum();

            a_sum.cmp(&b_sum)
        });

    if let Some((id, times)) = max {
        //  Get the minute corresponding to the maximum time slept
        let (min, _) = times.iter()
            .max_by(|(_, a), (_, b)| a.cmp(&b))
            .unwrap_or((&0, &0));
        
        return Ok(min * id);
    }

    Ok(0)
}

pub fn part_2() -> Result<i32, std::io::Error> {
    use std::collections::HashMap;

    let content = std::fs::read_to_string("input/4.txt")?;

    //  Turn challenge input into vec of log entries
    let mut logs = content.lines()
        .map(|line| LogEntry::new(line))
        .collect::<Vec<LogEntry>>();

    //  Sort entries based on date
    logs.sort();
    
    let mut tracker = HashMap::new();
    let mut current_id = 0;
    let mut sleep_start = 0;

    for entry in logs {
        match entry.event {
            //  If new shift starts, change current_id
            Event::Shift(id) => current_id = id,
            //  If sleep starts, mark the minute counter
            Event::Sleep => sleep_start = entry.time.min,
            //  If waking up, insert an entry for every minute slept for that guard id
            Event::Wake => {
                //  Each guard entry is a histogram of time slept
                let guard_entry = tracker.entry(current_id).or_insert(HashMap::new());
                
                //  Insert every minute slept, or add 1 if already exists
                for min in sleep_start..entry.time.min {
                    let e = guard_entry.entry(min).or_insert(0);
                    *e += 1;
                }
            }
        }
    }

    let result = tracker.iter()
        .map(|(id, times)| {
            let (min, count) = times.iter()
                .max_by(|(_, a), (_, b)| a.cmp(&b))
                .unwrap_or((&0, &0));

            (id, min, count)
        })
        .max_by(|(_, _, count1), (_, _, count2)| count1.cmp(count2))
        .unwrap_or((&0, &0, &0));

    Ok(result.0 * result.1)
}
