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

pub mod part1 {
    use super::*;

    const DIRECTION : [[(i32,i32);4]; 8] = [
        [( 0, 0),( 0, 1),( 0, 2),( 0, 3)], //  E
        [( 0, 0),( 0,-1),( 0,-2),( 0,-3)], //  W
        [( 0, 0),(-1, 0),(-2, 0),(-3, 0)], //  N
        [( 0, 0),( 1, 0),( 2, 0),( 3, 0)], //  S
        [( 0, 0),( 1, 1),( 2, 2),( 3, 3)], //  SE
        [( 0, 0),( 1,-1),( 2,-2),( 3,-3)], //  SW
        [( 0, 0),(-1,-1),(-2,-2),(-3,-3)], //  NW
        [( 0, 0),(-1, 1),(-2, 2),(-3, 3)], //  NE
    ];

    pub fn solve(file: String) -> u64 {
        let grid = parse(file);
        let mut count = 0;
        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                for dir in DIRECTION {
                    if let Some(x) = grid.get(row as i32 + dir[0].0, col as i32 + dir[0].1) &&
                       let Some(m) = grid.get(row as i32 + dir[1].0, col as i32 + dir[1].1) &&
                       let Some(a) = grid.get(row as i32 + dir[2].0, col as i32 + dir[2].1) &&
                       let Some(s) = grid.get(row as i32 + dir[3].0, col as i32 + dir[3].1) &&
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
    pub fn solve(_file: String) -> u64 {
        0
    }
}
