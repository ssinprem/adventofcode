use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    Source,
    Flow,
    Full,
    None,
}

pub fn parse(file: String) -> HashSet<(usize, usize)> {
    use regex::Regex;

    let regex_xy = Regex::new(r"x=(\d+), y=(\d+)..(\d+)").unwrap();
    let regex_yx = Regex::new(r"y=(\d+), x=(\d+)..(\d+)").unwrap();
    let mut map = HashSet::new();

    file.lines().for_each(|line| {
        if let Some(cap) = regex_xy.captures(line)
            && let Some(x) = cap.get(1)
            && let Ok(x) = x.as_str().parse::<usize>()
            && let Some(y1) = cap.get(2)
            && let Ok(y1) = y1.as_str().parse::<usize>()
            && let Some(y2) = cap.get(3)
            && let Ok(y2) = y2.as_str().parse::<usize>()
        {
            for y in y1..=y2 {
                map.insert((x, y));
            }
        } else if let Some(cap) = regex_yx.captures(line)
            && let Some(y) = cap.get(1)
            && let Ok(y) = y.as_str().parse::<usize>()
            && let Some(x1) = cap.get(2)
            && let Ok(x1) = x1.as_str().parse::<usize>()
            && let Some(x2) = cap.get(3)
            && let Ok(x2) = x2.as_str().parse::<usize>()
        {
            for x in x1..=x2 {
                map.insert((x, y));
            }
        }
    });

    map
}

pub fn _display(clays: &HashSet<(usize, usize)>, waters: &HashMap<(usize, usize), Cell>) {
    let minx = clays.iter().min_by_key(|(x, _y)| *x).unwrap().0 - 1;
    let maxx = clays.iter().max_by_key(|(x, _y)| *x).unwrap().0 + 1;
    let maxy = clays.iter().max_by_key(|(_x, y)| *y).unwrap().1 + 1;

    for y in 0..=maxy {
        for x in minx..=maxx {
            if clays.contains(&(x, y)) {
                print!("#");
            } else if let Some(c) = waters.get(&(x, y)) {
                print!(
                    "{}",
                    match c {
                        Cell::Source => "+",
                        Cell::Flow => "|",
                        Cell::Full => "~",
                        _ => "?",
                    }
                );
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

pub mod part1 {
    use std::collections::HashMap;

    use crate::{_display, Cell, parse};

    pub fn solve(file: String) -> usize {
        let clays = parse(file);
        let miny = clays.iter().min_by_key(|(_x, y)| *y).unwrap().1;
        let maxy = clays.iter().max_by_key(|(_x, y)| *y).unwrap().1;
        let mut waters = HashMap::new();
        waters.insert((500, 0), Cell::Source);
        let mut last_waters = HashMap::new();

        _display(&clays, &waters);
        while last_waters != waters {
            last_waters = waters.clone();

            for (&(x, y), &c) in waters
                .clone()
                .iter()
                .filter(|(_, c)| [Cell::Source, Cell::Flow].contains(*c))
            {
                if (c == Cell::Source || c == Cell::Flow)
                    && y < maxy
                    && !clays.contains(&(x, y + 1))
                    && !waters.contains_key(&(x, y + 1))
                {
                    // println!("{x},{y}  {c:?} => Cell:Flow  below empty");
                    waters.insert((x, y + 1), Cell::Flow);
                } else if c == Cell::Flow
                    && (clays.contains(&(x, y + 1))
                        || waters.get(&(x, y + 1)).unwrap_or(&Cell::None) == &Cell::Full)
                {
                    // println!("{x},{y}  {c:?}    below block going left and right");
                    let mut rows = vec![(x, y)];
                    let mut found_block_left = false;
                    let mut found_block_right = false;
                    for i in 1.. {
                        if clays.contains(&(x - i, y)) {
                            found_block_left = true;
                            break;
                        }
                        rows.push((x - i, y));
                        // have next floor ?
                        if clays.contains(&(x - i, y + 1)) {
                            continue;
                        }
                        // have full water cell
                        if let Some(w) = waters.get(&(x - i, y + 1))
                            && w == &Cell::Full
                        {
                            continue;
                        }

                        break;
                    }
                    for i in 1.. {
                        if clays.contains(&(x + i, y)) {
                            found_block_right = true;
                            break;
                        }
                        rows.push((x + i, y));
                        // have next floor ?
                        if clays.contains(&(x + i, y + 1)) {
                            continue;
                        }
                        // have full water cell
                        if let Some(w) = waters.get(&(x + i, y + 1))
                            && w == &Cell::Full
                        {
                            continue;
                        }

                        break;
                    }
                    for (rx, ry) in rows {
                        if found_block_left && found_block_right {
                            waters.insert((rx, ry), Cell::Full);
                        } else {
                            waters.insert((rx, ry), Cell::Flow);
                        }
                    }
                }
            }
            // utility::_pause();
        }
        _display(&clays, &waters);
        waters
            .iter()
            .filter(|((_x, y), _)| y >= &miny && y <= &maxy)
            .count()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
