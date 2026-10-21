use chrono::NaiveDateTime;
use regex::Regex;

#[derive(Debug)]
pub enum Event {
    Start(usize),
    Fall,
    Wake,
}

pub fn parse(file: String) -> Vec<(NaiveDateTime, Event)> {
    let regex_line = Regex::new(r"\[(\d+\-\d+\-\d+ \d\d:\d\d)\] (.*)").unwrap();
    let regex_start = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    file.lines()
        .map(|line| {
            let matched = regex_line.captures(line).unwrap();
            let date_str = matched.get(1).unwrap().as_str().to_string();
            let evt_str = matched.get(2).unwrap().as_str().to_string();
            let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M").unwrap();

            let event = if let Some(cap) = regex_start.captures(&evt_str)
                && let Some(id) = cap.get(1)
                && let Ok(id) = id.as_str().parse::<usize>()
            {
                Event::Start(id)
            } else if evt_str == "falls asleep" {
                Event::Fall
            } else if evt_str == "wakes up" {
                Event::Wake
            } else {
                unimplemented!();
            };

            (date, event)
        })
        .collect()
}

pub mod part1 {
    use std::collections::HashMap;
    use chrono::Timelike;
    use crate::Event;
    use crate::parse;

    pub fn solve(file: String) -> u32 {
        let mut events = parse(file);
        events.sort_by_key(|(date, _evt)| *date);

        let mut falls = vec![];
        let mut id = 0;
        let mut start_min = 0;
        for (date, evt) in events {
            match evt {
                Event::Start(gid) => id = gid,
                Event::Fall => start_min = date.minute(),
                Event::Wake => {
                    let duration = date.minute() - start_min;
                    falls.push((id, start_min, duration));
                }
            }
        }

        let mut guards = HashMap::new();
        falls.iter().for_each(|(id, _start, duration)| {
            guards
                .entry(id)
                .and_modify(|d| *d += *duration)
                .or_insert(*duration);
        });

        let id = **guards
            .iter()
            .max_by_key(|(_id, duration)| **duration)
            .unwrap()
            .0;

        let mut minutes = HashMap::new();
        falls
            .iter()
            .filter(|(gid, _, _)| id == *gid)
            .for_each(|(_, start, duration)| {
                for i in 0..*duration {
                    minutes
                        .entry(start + i)
                        .and_modify(|min| {
                            *min += 1;
                        })
                        .or_insert(1);
                }
            });

        let min = minutes.iter().max_by_key(|(_id, count)| **count).unwrap();
        id as u32 * min.0
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
