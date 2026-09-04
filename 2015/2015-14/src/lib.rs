use regex::Regex;
#[derive(Debug)]
pub struct Deer {
    speed: u64,
    run_time: u64,
    rest_time: u64
}

pub fn parse(file: String) -> Vec<Deer> {
    let regex = Regex::new(r"^.* (\d+) km/s for (\d+) seconds.* (\d+) seconds.$").unwrap();
    file.lines()
    .filter_map(|line| {
        if let Some(cap) = regex.captures(line)
        && let Some(speed) = cap.get(1)
        && let Ok(speed) = speed.as_str().parse::<u64>()
        && let Some(run_time) = cap.get(2)
        && let Ok(run_time) = run_time.as_str().parse::<u64>()
        && let Some(rest_time) = cap.get(3)
        && let Ok(rest_time) = rest_time.as_str().parse::<u64>()
        {
            Some( Deer {
                speed,
                run_time,
                rest_time
            })
        } else {
            None
        }
    }).collect()
}

pub mod part1 {
    use crate::*;

    pub fn solve(file: String, time: u64) -> u64 {
        let deers = parse(file);
        let dists: Vec<u64> = deers.iter().map(|deer| {
            let mut dist = 0;
            let mut t = time;
            let mut run = true;
            while t != 0 {
                if run {
                    if t > deer.run_time {
                        dist += deer.speed * deer.run_time;
                        t -= deer.run_time;
                    } else {
                        dist += deer.speed * t;
                        break;
                    }
                } else {
                    if t > deer.rest_time {
                        t -= deer.rest_time;
                    } else {
                        break;
                    }
                }
                run = !run;
            }
            dist
        }).collect();
        dists.into_iter().max().unwrap() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
