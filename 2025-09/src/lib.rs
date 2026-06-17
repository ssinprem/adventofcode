#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Seat {
    x: u64,
    y: u64,
}

impl Seat {
    pub fn new_xy(x: u64,y: u64) -> Self {
        Self { x, y }
    }

    pub fn new(c: Vec<String>) -> Self {
        Self {
            x: c.first().unwrap_or(&"0".to_string()).parse::<u64>().unwrap(),
            y: c.get(1).unwrap_or(&"0".to_string()).parse::<u64>().unwrap(),
        }
    }

    pub fn area(&self, b: Seat) -> u64 {
        (
            ((self.x as i64 - b.x as i64 ).abs() +1)  * 
            ((self.y as i64 - b.y as i64 ).abs() +1)
        ) as u64
    }
}


pub mod part1 {
    use std::collections::HashMap;
    use crate::Seat;

    pub fn solve(file: String) -> u64 {
        let mut areas = HashMap::<(Seat, Seat), u64>::new();
        let seats = file.lines().filter(|line| !line.is_empty())
            .map(|line| {
                let str_arr = line.split(",")
                    .map(|str| str.to_string())
                    .collect();
                Seat::new(str_arr)
            }).collect::<Vec<Seat>>();

        for i in 0..seats.len() {
            let iseat = seats.get(i).unwrap();
            for j in i+1..seats.len() {
                let jseat = seats.get(j).unwrap();
                areas.insert((*iseat,*jseat), iseat.area(*jseat));
            }
        }

        if let Some(max_entry) = areas.iter().max_by_key(|item| item.1) {
            *max_entry.1
        } else {
            0
        }
    }
}

pub mod part2 {
    use std::collections::{HashMap, HashSet};
    use crate::Seat;

    pub fn solve(file: String) -> u64 {
        let mut areas = HashMap::<(Seat, Seat), u64>::new();
        let reds = file.lines().filter(|line| !line.is_empty())
            .map(|line| {
                let str_arr = line.split(",")
                    .map(|str| str.to_string())
                    .collect();
                Seat::new(str_arr)
            }).collect::<Vec<Seat>>();
        println!("initial Red point {}", reds.len());
        for i in 0..reds.len() {
            let iseat = reds.get(i).unwrap();
            for j in i+1..reds.len() {
                let jseat = reds.get(j).unwrap();
                areas.insert((*iseat,*jseat), iseat.area(*jseat));
            }
        }
        println!("calulate possible areas {} max-> {:?}", areas.len(), areas.iter().max_by_key(|item| item.1));

        let mut greens = HashSet::<(u64,u64)>::new();
        for i in 0..reds.len() {
            let iseat = reds.get(i).unwrap();
            for j in i+1..reds.len() {
                let jseat = reds.get(j).unwrap();
                if iseat.x == jseat.x {
                    for y in iseat.y.min(jseat.y)..=iseat.y.max(jseat.y) {
                        greens.insert((iseat.x as u64, y as u64));
                    } 
                } else if iseat.y == jseat.y {
                    for x in iseat.x.min(jseat.x)..=iseat.x.max(jseat.x) {
                        greens.insert((x as u64, iseat.y as u64));
                    }
                }
            }
        }
        println!("initial Green line for Red point  {}", greens.len());

        // fill inside the with odd-even rules
        let start = (
            greens.iter().min_by_key(|item| item.0 ).unwrap().0,
            greens.iter().min_by_key(|item| item.1 ).unwrap().1
        );
        let end = (
            greens.iter().max_by_key(|item| item.0 ).unwrap().0,
            greens.iter().max_by_key(|item| item.1 ).unwrap().1
        );

        // for y in start.1..=end.1 {
        //     println!("{y}");
        //     let mut cnt = 0;
        //     for x in start.0..=end.0 {
        //         if greens.contains(&(x,y)) && !greens.contains(&(x+1,y)) {
        //             cnt+=1;
        //         } else if !greens.contains(&(x,y)) && greens.contains(&(x+1,y)) {
        //             cnt+=1;
        //         }
        //         if cnt % 2 == 1 {
        //             greens.insert((x,y));
        //         }
        //     }
        // }
        // println!("fill the Green areas in Green line {}",greens.len());

        // let mut ignore_list = Vec::new();
        let mut rank: Vec<(((u64,u64), (u64,u64)), u64)> = areas.iter()
            .map(|item| {
                (
                    (
                        (item.0.0.x, item.0.0.y),
                        (item.0.1.x, item.0.1.y)
                    ),
                    *item.1
                )
            })
            .collect();
        rank.sort_by_key(|item| std::cmp::Reverse(item.1));
        'find_outring: for max in rank.iter() {
                println!("max {:?}",max);
                let seat_a = max.0.1;
                let seat_b = max.0.0;
                for x in seat_a.0.min(seat_b.0)..=seat_a.0.max(seat_b.0) {
                    let mut cnt = 0;
                    for y in seat_a.1.min(seat_b.1)..=seat_a.1.max(seat_b.1) {
                        if greens.get(&(x,y)).is_none() && greens.get(&(x,y+1)).is_some() {
                            cnt += 1;
                        } else if greens.get(&(x,y)).is_some() && greens.get(&(x,y+1)).is_none() {
                            cnt += 1;
                        }
                        if cnt > 1 {
                            continue 'find_outring;
                        }
                    }
                }
            return max.1;
        } 
        return 0;
    }
}

