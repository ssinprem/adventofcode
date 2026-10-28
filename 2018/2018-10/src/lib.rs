pub fn parse(file: String) -> Vec<((isize,isize),(isize,isize))> {
    use regex::Regex;

    let regex = Regex::new(r"position=<\s*([-0-9]+),\s*([-0-9]+)> velocity=<\s*([-0-9]+),\s*([-0-9]+)>").unwrap();
    file.lines().map(|line| {
        if let Some(caps) = regex.captures(line)
        && let Some(x) = caps.get(1)
        && let Some(y) = caps.get(2)
        && let Some(vx) = caps.get(3)
        && let Some(vy) = caps.get(4)
        && let Ok(x) = x.as_str().parse::<isize>()
        && let Ok(y) = y.as_str().parse::<isize>()
        && let Ok(vx) = vx.as_str().parse::<isize>()
        && let Ok(vy) = vy.as_str().parse::<isize>()
        {
            ((x,y),(vx,vy))
        } else {
            unreachable!()
        }
    }).collect()
}

pub fn get_size(points: &Vec<((isize,isize),(isize,isize))>) -> isize {
    let min_x = points.iter().map(|((x,_),(_,_))| *x).min().unwrap();
    let max_x = points.iter().map(|((x,_),(_,_))| *x).max().unwrap();
    let min_y = points.iter().map(|((_,y),(_,_))| *y).min().unwrap();
    let max_y = points.iter().map(|((_,y),(_,_))| *y).max().unwrap();
    (max_x - min_x) * (max_y - min_y)
}

pub fn _display(points: &Vec<((isize,isize),(isize,isize))>) -> String {
    let min_x = points.iter().map(|((x,_),(_,_))| *x).min().unwrap();
    let max_x = points.iter().map(|((x,_),(_,_))| *x).max().unwrap();
    let min_y = points.iter().map(|((_,y),(_,_))| *y).min().unwrap();
    let max_y = points.iter().map(|((_,y),(_,_))| *y).max().unwrap();
    let mut string = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.iter().any(|((px,py),(_,_))| {
                *px == x && *py == y
            }) {
                string += "#";
            } else {
                string += ".";
            }
        }
        string += "\n";
    }

    string
}

pub fn process(points: &mut Vec<((isize,isize),(isize,isize))>) {
    for ((px,py),(vx,vy)) in points {
        *px += *vx;
        *py += *vy;
    }
}

pub mod part1 {
    use crate::{_display, get_size, parse, process};
    use utility::_pause;

    pub fn solve(file: String) -> usize {
        let mut points = parse(file);
        
        let mut last_point = points.clone();
        let mut last_size = isize::MAX;
        for s in 0.. {
            process(&mut points);
            let size = get_size(&points);
            
            if size > last_size {
                println!("{}", _display(&last_point));
                return s;
            }
            last_size = size;
            last_point = points.clone()
        }
        0
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
