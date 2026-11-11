use grid::Grid;

const MOD: usize = 20183;
const X_MUL: usize = 16807;
const Y_MUL: usize = 48271;

#[derive(Clone, PartialEq)]
pub enum MapType {
    Rock,
    Wet,
    Narrow,
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Equip {
    Torch,
    ClimbingGear,
    Neither,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Event {
    Move((usize, usize)),
    Change(Equip),
}

pub fn parse(file: String) -> (usize, (usize, usize)) {
    use regex::Regex;
    let regex_dept = Regex::new(r"depth: (\d+)").unwrap();
    let dept = regex_dept
        .captures(file.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let regex_pos = Regex::new(r"target: (\d+),(\d+)").unwrap();
    let match_pos = regex_pos.captures(file.as_str()).unwrap();
    let x = match_pos.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let y = match_pos.get(2).unwrap().as_str().parse::<usize>().unwrap();
    (dept, (x, y))
}

pub fn create_grid(dept: usize, target: (usize, usize)) -> Grid<MapType> {
    let mut map = Grid::new(target.1 + 50, target.0 + 50);

    let rows = map.rows();
    let cols = map.cols();

    for row in 0..rows {
        for col in 0..cols {
            if (col, row) == (0, 0) || (col, row) == target {
                let cell = map.get_mut(row, col).unwrap();
                *cell = dept % MOD;
            } else if row == 0 {
                let cell = map.get_mut(0, col).unwrap();
                *cell = ((col * X_MUL) + dept) % MOD;
            } else if col == 0 {
                let cell = map.get_mut(row, 0).unwrap();
                *cell = ((row * Y_MUL) + dept) % MOD;
            } else {
                let value = map.get(row, col - 1).unwrap() * map.get(row - 1, col).unwrap();
                let value = (value + dept) % MOD;
                let cell = map.get_mut(row, col).unwrap();
                *cell = value;
            }
        }
    }

    map.map(|x| match x % 3 {
        0 => MapType::Rock,
        1 => MapType::Wet,
        2 => MapType::Narrow,
        _ => unreachable!(),
    })
    .clone()
}

pub fn _display(grid: &Grid<MapType>, target: (usize, usize)) {
    let rows = grid.rows();
    let cols = grid.cols();

    for row in 0..rows {
        for col in 0..cols {
            if (col, row) == (0, 0) {
                print!("M");
            } else if (col, row) == target {
                print!("T");
            } else {
                match grid.get(row, col) {
                    Some(&MapType::Rock) => print!("."), // rocky    need climbing or torch
                    Some(&MapType::Wet) => print!("="),  // wet      need climbing or neither
                    Some(&MapType::Narrow) => print!("|"), // narrow   need torch or neother
                    None => print!("?"),
                }
            }
        }
        println!();
    }
}

pub mod part1 {

    use crate::{_display, MapType, create_grid, parse};

    pub fn solve(file: String) -> usize {
        let (dept, target) = parse(file);
        let grid = create_grid(dept, target);

        _display(&grid, target);

        let mut vals = 0;
        for row in 0..=target.1 {
            for col in 0..=target.0 {
                let val = if target != (col, row) {
                    match grid.get(row, col) {
                        Some(MapType::Rock) => 0,
                        Some(MapType::Wet) => 1,
                        Some(MapType::Narrow) => 2,
                        None => unreachable!(),
                    }
                } else {
                    0
                };
                vals += val;
            }
        }

        vals
    }
}

pub mod part2 {
    use crate::{_display, Equip, Event, MapType, create_grid, parse};
    use std::collections::{BTreeSet, HashMap};

    pub fn allow(map: &MapType, equip: &Equip) -> bool {
        matches!(
            (map, equip),
            (MapType::Rock, Equip::Torch)
                | (MapType::Rock, Equip::ClimbingGear)
                | (MapType::Wet, Equip::ClimbingGear)
                | (MapType::Wet, Equip::Neither)
                | (MapType::Narrow, Equip::Torch)
                | (MapType::Narrow, Equip::Neither)
        )
    }

    pub fn solve(file: String) -> usize {
        let (dept, target) = parse(file);
        let grid = create_grid(dept, target);

        let mut best = HashMap::new();
        // use Binary Tree Set to get less cost in stack first.
        let mut stack = BTreeSet::new();

        _display(&grid, target);

        // cost, equip, (pos)
        stack.insert((0_usize, Equip::Torch, (0_usize, 0_usize), vec![]));
        while let Some((cost, equip, pos, events)) = stack.pop_first() {
            if let Some((pre_best, _evt)) = best.get(&(pos, equip))
                && cost >= *pre_best
            {
                continue;
            }
            best.insert((pos, equip), (cost, events.clone()));

            let mut candidate = Vec::new();
            // find the next step able to move with current equipment
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next_pos = (
                    (pos.0 as isize + dx) as usize,
                    (pos.1 as isize + dy) as usize,
                );
                if let Some(next_map) = grid.get(next_pos.1, next_pos.0)
                    && allow(next_map, &equip)
                {
                    // use cost 1 minute for moving
                    candidate.push((cost + 1, equip, next_pos, Event::Move(next_pos)));
                }
            }
            // change the equipment with current position
            let curr_map = grid.get(pos.1, pos.0).unwrap();
            for e in [Equip::Torch, Equip::ClimbingGear, Equip::Neither] {
                if equip != e && allow(curr_map, &e) {
                    // use cost 7 minute for changing equipment
                    candidate.push((cost + 7, e, pos, Event::Change(e)));
                }
            }
            for (cost, equip, pos, event) in candidate {
                if let Some((pre_best, _evt)) = best.get(&(pos, equip))
                    && cost >= *pre_best
                {
                    continue;
                }
                stack.insert((cost, equip, pos, [events.clone(), vec![event]].concat()));
            }
        }

        // find at target point with torch equipment
        let best = best
            .iter()
            .find(|((pos, equip), _cost)| pos == &target && equip == &Equip::Torch)
            .unwrap();
        best.1.0
    }
}
