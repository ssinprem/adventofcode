pub mod part1 {
    pub fn solve(file: String) -> usize {
        (0..128).map(|row| {
            let key = format!("{file}-{row}");
            let hash = knot_hash::part2::solve(key.to_string());
            let binary = hash.chars().map(|char| {
                match char {
                    '0' => "0000",
                    '1' => "0001",
                    '2' => "0010",
                    '3' => "0011",
                    '4' => "0100",
                    '5' => "0101",
                    '6' => "0110",
                    '7' => "0111",
                    '8' => "1000",
                    '9' => "1001",
                    'a' => "1010",
                    'b' => "1011",
                    'c' => "1100",
                    'd' => "1101",
                    'e' => "1110",
                    'f' => "1111",
                    _ => unreachable!()
                }
            }).collect::<String>();
            // println!("{binary}");
            binary.chars().filter(|char| char == &'1').count()
        }).sum::<usize>()
    }
}

pub mod part2 {
    use grid::Grid;

    pub fn solve(file: String) -> usize {
        let mut grid = Grid::new(128,128);
        (0..128).for_each(|row| {
            let key = format!("{file}-{row}");
            let hash = knot_hash::part2::solve(key.to_string());
            let binary = hash.chars().map(|char| {
                match char {
                    '0' => "0000",
                    '1' => "0001",
                    '2' => "0010",
                    '3' => "0011",
                    '4' => "0100",
                    '5' => "0101",
                    '6' => "0110",
                    '7' => "0111",
                    '8' => "1000",
                    '9' => "1001",
                    'a' => "1010",
                    'b' => "1011",
                    'c' => "1100",
                    'd' => "1101",
                    'e' => "1110",
                    'f' => "1111",
                    _ => unreachable!()
                }
            }).collect::<String>();
            binary.chars().enumerate().for_each(|(col,char)| {
                let cell = grid.get_mut(row, col).unwrap();
                *cell = if char == '1' {
                    (true, 0)
                } else {
                    (false, 0)
                };
            })
        });

        let mut i = 1;
        loop
        {
            let mut first_cell = None;
            for row in 0..128 {
                for col in 0..128 {
                    let cell = grid.get_mut(row, col).unwrap();
                    if cell.0 && cell.1 == 0 {
                        cell.1 = i;
                        first_cell = Some((row,col));
                        break;
                    }
                }
                if first_cell != None {
                    break;
                }
            }
            if first_cell == None {
                break;
            }

            let mut stack = Vec::new();
            stack.push(first_cell.unwrap());

            while let Some((y,x)) = stack.pop() {
                [(-1,0),(0,-1),(1,0),(0,1)].iter().for_each(|(dy, dx)| {
                    let ny = (y + *dy) as isize;
                    let nx = (x + *dx) as isize;
                    if let Some(near_cell) = grid.get_mut(ny, nx) {
                        if near_cell.0 && near_cell.1 == 0 {
                            near_cell.1 = i;
                            stack.push((ny,nx));
                        }
                    }
                });
            }

            i+=1;
        }

        // for y in 0..128 {
        //     for x in 0..128 {
        //         let cell = grid.get(y, x).unwrap();
        //         if cell.0 {
        //             print!("{}", (cell.1 % 26 + 65) as u8 as char);
        //         } else {
        //             print!(".")
        //         }
        //     }
        //     println!();
        // }

        i-1
    }
}
