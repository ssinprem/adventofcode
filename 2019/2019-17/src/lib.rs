use std::collections::HashMap;

// copy process from aoc_2019_13  and modified to able put long input
// return when got Opcode 99 <terminate>
pub fn process(
    memory: &mut HashMap<usize, i128>,
    pc: &mut usize,
    relative_base: &mut isize,
    input: &[i128],
) -> Option<Vec<isize>> {

    let mut output = vec![];
    let mut i = 0;
    let size = input.len();
    loop {
        let inst = memory.get(pc).expect("cannot access pc mem");
        let p1_mode = (inst / 100) % 10;
        let p2_mode = (inst / 1000) % 10;
        let p3_mode = (inst / 10000) % 10;

        let a_value = if ![1, 2, 4, 5, 6, 7, 8, 9].contains(&(inst % 100)) {
            0
        } else if p1_mode == 0 {
            let addr = *memory.get(&(*pc + 1)).unwrap() as usize;
            *memory.get(&addr).unwrap_or(&0)
        } else if p1_mode == 1 {
            *memory.get(&(*pc + 1)).unwrap_or(&0)
        } else if p1_mode == 2 {
            let offset = *memory.get(&(*pc + 1)).unwrap() as isize;
            *memory
                .get(&((*relative_base + offset) as usize))
                .unwrap_or(&0)
        } else {
            unreachable!()
        };
        let b_value = if ![1, 2, 5, 6, 7, 8].contains(&(inst % 100)) {
            0
        } else if p2_mode == 0 {
            let addr = *memory.get(&(*pc + 2)).unwrap() as usize;
            *memory.get(&addr).unwrap_or(&0)
        } else if p2_mode == 1 {
            *memory.get(&(*pc + 2)).unwrap_or(&0)
        } else if p2_mode == 2 {
            let offset = *memory.get(&(*pc + 2)).unwrap() as isize;
            *memory
                .get(&((*relative_base + offset) as usize))
                .unwrap_or(&0)
        } else {
            unreachable!()
        };

        let write_addr = if ![1, 2, 3, 7, 8].contains(&(inst % 100)) {
            0
        } else if inst % 100 == 3 {
            if p1_mode == 0 {
                *memory.get(&(*pc + 1)).unwrap() as usize
            } else if p1_mode == 2 {
                (*relative_base + (*memory.get(&(*pc + 1)).unwrap()) as isize) as usize
            } else {
                unreachable!()
            }
        } else {
            if p3_mode == 0 {
                *memory.get(&(*pc + 3)).unwrap() as usize
            } else if p3_mode == 2 {
                (*relative_base + (*memory.get(&(*pc + 3)).unwrap()) as isize) as usize
            } else {
                unreachable!()
            }
        };
        if inst % 100 == 1 {
            // Add
            let result = a_value + b_value;
            memory.insert(write_addr, result);
            *pc += 4;
        } else if inst % 100 == 2 {
            // Mul
            let result = a_value * b_value;
            memory.insert(write_addr, result);
            *pc += 4;
        } else if inst % 100 == 3 {
            // Input
            if i >= size {
                unreachable!();
            }

            memory.insert(write_addr, input[i]);
            i += 1;
            *pc += 2;
        } else if inst % 100 == 4 {
            // Output
            *pc += 2;
            output.push(a_value as isize);
            // output.push(format!("{a_value}"));
        } else if inst % 100 == 5 {
            // Jump if true
            if a_value != 0 {
                *pc = b_value as usize
            } else {
                *pc += 3;
            }
        } else if inst % 100 == 6 {
            // Jump if false
            if a_value == 0 {
                *pc = b_value as usize
            } else {
                *pc += 3;
            }
        } else if inst % 100 == 7 {
            // less than  ,, true put 1
            memory.insert(write_addr, if a_value < b_value { 1 } else { 0 });
            *pc += 4;
        } else if inst % 100 == 8 {
            // equal ,, true put 1
            memory.insert(write_addr, if a_value == b_value { 1 } else { 0 });
            *pc += 4;
        } else if inst % 100 == 9 {
            // move relative
            *relative_base += a_value as isize;
            *pc += 2;
        } else if inst % 100 == 99 {
            if !output.is_empty() {
                return Some(output)
            } else {
                return None;
            }
        } else {
            unreachable!();
        }
    }
}

pub mod part1 {
    pub fn solve(file: String) -> usize {
        let output = aoc_2019_09::part1::solve(file);
        let map = output
            .split(",")
            .map(|n| n.parse::<u8>().unwrap() as char)
            .collect::<String>();
        let mut grid = grid::Grid::new(0, 0);

        map.lines()
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                grid.push_row(line.chars().map(|char| !matches!(char, '.')).collect())
            });
        let rows = grid.rows();
        let cols = grid.cols();

        (1..rows - 1)
            .map(|row| {
                (1..cols - 1)
                    .map(|col| {
                        if [(-1, 0), (1, 0), (0, 0), (0, 1), (0, -1)]
                            .iter()
                            .all(|(dx, dy)| {
                                *grid.get(row as isize + dy, col as isize + dx).unwrap()
                            })
                        {
                            row * col
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

pub mod part2 {

    use crate::process;

    pub fn find_path_seq(file: String) -> String {
        #[derive(Debug, PartialEq, Clone, Copy)]
        enum Dir {
            N,
            E,
            W,
            S
        }

        #[derive(Default, PartialEq, Clone, Copy)]
        enum Cell {
            Drone(Dir),
            Path,
            #[default]
            Space,
        }

        let output = aoc_2019_09::part1::solve(file);
        let map = output
            .split(",")
            .map(|n| n.parse::<u8>().unwrap() as char)
            .collect::<String>();
        let mut grid = grid::Grid::new(0, 0);

        let mut drone = (0,0,Dir::N);

        map.lines()
            .filter(|line| !line.is_empty())
            .enumerate()
            .for_each(|(y,line)| {
                grid.push_row(line.chars().enumerate().map(|(x,char)| 
                    match char {
                        '^' => {
                            drone = (x,y,Dir::N);
                            Cell::Drone(Dir::N)
                        },
                        'v' => {
                            drone = (x,y,Dir::S);
                            Cell::Drone(Dir::S)
                        },
                        '<' => {
                            drone = (x,y,Dir::W);
                            Cell::Drone(Dir::W)
                        },
                        '>' => {
                            drone = (x,y,Dir::E);
                            Cell::Drone(Dir::E)
                        },
                        '#' => Cell::Path,
                        _ => Cell::Space
                    }).collect()
                )
            });

        let mut path = "".to_string();
        loop {
            let next_pos = match drone.2.clone() {
                Dir::N => (drone.0 as isize, drone.1  as isize- 1),
                Dir::E => (drone.0 as isize + 1, drone.1 as isize),
                Dir::W => (drone.0 as isize - 1, drone.1 as isize),
                Dir::S => (drone.0 as isize, drone.1 as isize + 1),
            };

            if next_pos.0 >= 0 && next_pos.1 >= 0
            && let Some(next_path) = grid.get(next_pos.1, next_pos.0)
            && next_path == &Cell::Path
            {
                path += "S";
                drone = (next_pos.0 as usize, next_pos.1 as usize, drone.2);
            } else if let Some(turn) = ["L","R"].iter().find(|t| {
                let candidate = match (**t, drone.2.clone()) {
                    ("L", Dir::N) => (drone.0 as isize -1, drone.1 as isize , Dir::W),
                    ("L", Dir::E) => (drone.0 as isize , drone.1 as isize -1, Dir::N),
                    ("L", Dir::W) => (drone.0 as isize , drone.1 as isize +1, Dir::S),
                    ("L", Dir::S) => (drone.0 as isize +1, drone.1 as isize , Dir::E),
                    ("R", Dir::N) => (drone.0 as isize +1, drone.1 as isize , Dir::E),
                    ("R", Dir::E) => (drone.0 as isize , drone.1 as isize +1, Dir::S),
                    ("R", Dir::W) => (drone.0 as isize , drone.1 as isize -1, Dir::N),
                    ("R", Dir::S) => (drone.0 as isize -1, drone.1 as isize , Dir::W),
                    _ => unreachable!()
                };
                if candidate.0 < 0 || candidate.1 < 0 {
                    return false;
                }
                Some(&Cell::Path) == grid.get(candidate.1, candidate.0)
            }) {
                path += turn;
                drone.2 = match (*turn,drone.2) {
                    ("L", Dir::N) => Dir::W,
                    ("L", Dir::E) => Dir::N,
                    ("L", Dir::W) => Dir::S,
                    ("L", Dir::S) => Dir::E,
                    ("R", Dir::N) => Dir::E,
                    ("R", Dir::E) => Dir::S,
                    ("R", Dir::W) => Dir::N,
                    ("R", Dir::S) => Dir::W,
                    _ => unreachable!()
                };
            } else {
                break;
            }
        }

        path.chars().collect::<Vec<char>>().chunk_by(|a,b| a==b)
        .map(|str| {
            if str.len() > 1 {
                format!("{}",str.len())
            } else {
                format!("{}",str[0])
            }
        }).collect::<String>()
    }

    pub fn solve(file: String) -> u64 {

        let path = find_path_seq(file.to_string());
        println!("{path}");

        let mut memory = aoc_2019_09::parse(file);
        memory.insert(0, 2);
        let mut pc = 0;
        let mut relative = 0;

        let input = 
        [
            "A,A,B,C,C,A,C,B,C,B\n",
            "L,4,L,4,L,6,R,10,L,6\n",
            "L,12,L,6,R,10,L,6\n",
            "R,8,R,10,L,6\n",
            "n\n"
        ].concat().as_str().chars().map(|c| c as i128).collect::<Vec<i128>>();
        
        // L4L4L6R10L6
        // L4L4L6R10L6
        // L12L6R10L6
        // R8R10L6
        // R8R10L6
        // L4L4L6R10L6
        // R8R10L6
        // L12L6R10L6
        // R8R10L6
        // L12L6R10L6 

        if let Some(output) = process(&mut memory, &mut pc, &mut relative, &input) {
            println!("{}", output.iter().map(|n| *n as u8 as char).collect::<String>());
            return *output.last().unwrap() as u64;
        }
        0
    }
}
