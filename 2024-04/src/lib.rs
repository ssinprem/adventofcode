use grid::*;

pub fn parse(file: String) -> Grid<char> {
    let filter_file: String = file.lines().filter(|line| !line.is_empty())
        .fold("".to_string(), |acc, line|  acc+line+"\n" );
    let rows = file.lines().filter(|line| !line.is_empty()).count();
    let cols = filter_file.lines()
        .find(|line| !line.is_empty()).expect("cannot get first line")
        .chars().count();
    println!(" r: {rows} c: {cols}");
    let mut grid = Grid::<char>::new(0,cols);

    filter_file.lines().enumerate().for_each(|(no,line)| {
        grid.insert_row(no, line.chars().collect());
    });
    grid
}

const C: (i32,i32)  = ( 0, 0);
const E: (i32,i32)  = ( 0, 1);
const W: (i32,i32)  = ( 0,-1);
const S: (i32,i32)  = ( 1, 0);
const N: (i32,i32)  = (-1, 0);
const SE: (i32,i32) = ( 1, 1);
const SW: (i32,i32) = ( 1,-1);
const NW: (i32,i32) = (-1,-1);
const NE: (i32,i32) = (-1, 1);
pub fn m(d : (i32,i32), m: i32) -> (i32,i32) {
    (d.0*m, d.1*m)
}
pub mod part1 {
    use super::*;

    
    pub fn solve(file: String) -> u64 {
        //   X  M   A             S 
        let directions : [[(i32,i32);4]; 8] = [
            [C, E,  m(E,2),  m(E,3)], //  E
            [C, W,  m(W,2),  m(W,3)], //  W
            [C, N,  m(N,2),  m(N,3)], //  N
            [C, S,  m(S,2),  m(S,3)], //  S
            [C, SE, m(SE,2), m(SE,3)], //  SE
            [C, SW, m(SW,2), m(SW,3)], //  SW
            [C, NW, m(NW,2), m(NW,3)], //  NW
            [C, NE, m(NE,2), m(NE,3)], //  NE
        ];
        let grid = parse(file);
        let mut count = 0;
        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                for d in directions {
                    if let Some(x) = grid.get(row as i32 + d[0].0, col as i32 + d[0].1) &&
                       let Some(m) = grid.get(row as i32 + d[1].0, col as i32 + d[1].1) &&
                       let Some(a) = grid.get(row as i32 + d[2].0, col as i32 + d[2].1) &&
                       let Some(s) = grid.get(row as i32 + d[3].0, col as i32 + d[3].1) &&
                       *x == 'X'&& *m == 'M' && *a == 'A' && *s == 'S' {
                        count+=1;
                    }
                }
            }
        }
        count
    }
}

pub mod part2 {
    use super::*;
        
    //   NW   NE    M   M
    //      C         A
    //   SW   SE    S   S
    const DIRECTION : [[(i32,i32);5]; 4] = [
    //    M   M  A   S   S
        [NW, NE, C, SW, SE], //  N
        [NW, SW, C, NE, SE], //  W
        [SW, SE, C, NW, NE], //  S
        [NE, SE, C, NW, SW], //  E
    ];

    pub fn solve(file: String) -> u64 {
        let grid = parse(file);
        let mut count = 0;
        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                for dir in DIRECTION {
                    if let Some(m1) = grid.get(row as i32 + dir[0].0, col as i32 + dir[0].1) &&
                       let Some(m2) = grid.get(row as i32 + dir[1].0, col as i32 + dir[1].1) &&
                       let Some(a) =  grid.get(row as i32 + dir[2].0, col as i32 + dir[2].1) &&
                       let Some(s1) = grid.get(row as i32 + dir[3].0, col as i32 + dir[3].1) &&
                       let Some(s2) = grid.get(row as i32 + dir[4].0, col as i32 + dir[4].1) &&
                       *m1 == 'M'&& *m2 == 'M' && *a == 'A' && *s1 == 'S' && *s2 == 'S' {
                        count+=1;
                    }
                }
            }
        }
        count
    }
}
