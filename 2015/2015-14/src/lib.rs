use regex::Regex;
#[derive(Debug, Clone)]
pub struct Deer {
    speed: u64,
    run_time: u64,
    rest_time: u64,
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
                Some(Deer {
                    speed,
                    run_time,
                    rest_time,
                })
            } else {
                None
            }
        })
        .collect()
}

pub mod part1 {
    use crate::*;

    pub fn solve(file: String, time: u64) -> u64 {
        let deers = parse(file);
        let dists: Vec<u64> = deers
            .iter()
            .map(|deer| {
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
            })
            .collect();
        dists.into_iter().max().unwrap()
    }
}

pub mod part2 {
    use crate::*;

    pub struct DeerExt {
        speed: u64,
        run_time: u64,
        rest_time: u64,
        mode: bool,
        dist: u64,
        time: u64,
    }

    pub fn solve(file: String, time: u64) -> u64 {
        let deers = parse(file);
        let mut deers = deers
            .iter()
            .map(|deer| DeerExt {
                speed: deer.speed,
                run_time: deer.run_time,
                rest_time: deer.rest_time,
                mode: true,
                dist: 0,
                time: 0,
            })
            .collect::<Vec<_>>();
        let mut scores = vec![0_u64; deers.len()];

        for _ in 0..time {
            for deer in deers.iter_mut() {
                if deer.mode {
                    // Run
                    deer.dist += deer.speed;
                    deer.time += 1;
                    if deer.time >= deer.run_time {
                        deer.mode = false;
                        deer.time = 0;
                    }
                } else {
                    // Rest
                    deer.time += 1;
                    if deer.time >= deer.rest_time {
                        deer.mode = true;
                        deer.time = 0;
                    }
                }
            }

            let lead_dist = deers.iter().map(|deer| deer.dist).max().unwrap();
            for i in 0..deers.len() {
                if deers[i].dist == lead_dist {
                    scores[i] += 1;
                }
            }
        }
        scores.into_iter().max().unwrap()
    }
}
