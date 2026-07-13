use grid::Grid;

// Numpad
//     -2  -1   0
//    +---+---+---+
// -3 | 7 | 8 | 9 |
//    +---+---+---+
// -2 | 4 | 5 | 6 |
//    +---+---+---+
// -1 | 1 | 2 | 3 |
//    +---+---+---+
//  0     | 0 | A |
//        +---+---+

// directional keypad
//    -2  -1   0
//       +---+---+
// 0     | ^ | A |
//   +---+---+---+
// 1 | < | v | > |
//   +---+---+---+
pub fn get_grids(is_numpad: bool) -> Grid<char> {
    if is_numpad {
        let mut numpad = Grid::new(4, 3);
        numpad.insert_row(0, vec!['7', '8', '9']);
        numpad.insert_row(1, vec!['4', '5', '6']);
        numpad.insert_row(2, vec!['1', '2', '3']);
        numpad.insert_row(3, vec![' ', '0', 'A']);
        numpad
    } else {
        let mut keypad = Grid::new(2, 3);
        keypad.insert_row(0, vec![' ', '^', 'A']);
        keypad.insert_row(1, vec!['<', 'v', '>']);
        keypad
    }
}

pub fn get_paths(from_char: char, end_char: char) -> Vec<String> {
    let is_numpad = "0123456789"
        .chars()
        .any(|char| from_char == char || end_char == char);
    let grid = get_grids(is_numpad);

    let from_cell = grid
        .clone()
        .indexed_into_iter()
        .find(|(_pos, char)| *char == from_char)
        .expect("cannot get 'start'");
    let end_cell = grid
        .clone()
        .indexed_into_iter()
        .find(|(_pos, char)| *char == end_char)
        .expect("cannot get 'end'");

    find_path(
        &grid,
        (from_cell.0.1 as isize, from_cell.0.0 as isize),
        (end_cell.0.1 as isize, end_cell.0.0 as isize),
    )
    .iter()
    .map(|string| string.to_string() + "A")
    .collect()
}

pub fn find_path(grid: &Grid<char>, curr: (isize, isize), target: (isize, isize)) -> Vec<String> {
    let dy = target.1 - curr.1;
    let dx = target.0 - curr.0;

    let mut dcell = Vec::<(char, (isize, isize))>::new();
    if dy < 0 {
        dcell.push(('^', (0, -1)))
    }
    if dy > 0 {
        dcell.push(('v', (0, 1)))
    }
    if dx < 0 {
        dcell.push(('<', (-1, 0)))
    }
    if dx > 0 {
        dcell.push(('>', (1, 0)))
    }
    if dcell.is_empty() {
        return vec!["".to_string()];
    }
    dcell
        .iter()
        .filter_map(|&(char, div)| {
            let next = (curr.0 + div.0, curr.1 + div.1);
            if let Some(next_char) = grid.get(next.1, next.0)
                && next_char != &' '
            /* empty key not allown */
            {
                let paths = find_path(grid, next, target)
                    .iter()
                    .map(|path| char.clone().to_string() + path.as_str())
                    .collect::<Vec<String>>();
                Some(paths)
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<String>>()
}

pub fn get_best_length(steps: String, deep: usize) -> usize {
    if deep == 0 {
        return steps.len();
    }
    let list_steps = "A".to_string() + steps.as_str();
    let mut count = 0;
    for pair in list_steps.chars().collect::<Vec<char>>().windows(2) {
        let from_char = pair[0];
        let end_char = pair[1];

        let paths = get_paths(from_char, end_char);
        let best_length = paths
            .iter()
            .map(|path| get_best_length(path.to_string(), deep - 1))
            .min()
            .unwrap();
        count += best_length;
    }
    count
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        file.lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let num = line.strip_suffix("A").unwrap().parse::<u64>().unwrap();
                let temp = get_best_length(line.to_string(), 3);
                num * temp as u64
            })
            .sum()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
