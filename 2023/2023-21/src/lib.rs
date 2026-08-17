use grid::*;

pub fn parse(file: String) -> (Grid<bool>, (usize, usize)) {
    let mut map = Grid::new(0, 0);
    let mut start = (0, 0);
    file.lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(row, line)| {
            if line.contains("S") {
                start = (row, line.chars().position(|c| c == 'S').unwrap())
            }
            map.push_row(line.chars().map(|c| c != '#').collect());
        });
    (map, start)
}

pub fn _display(map: &Grid<bool>, list: &[(usize, usize)]) -> String {
    let mut string = "".to_string();

    let rows = map.rows();
    let cols = map.cols();

    for row in 0..rows {
        for col in 0..cols {
            if list.contains(&(row, col)) {
                string += "O";
            } else if let Some(c) = map.get(row, col)
                && *c
            {
                string += ".";
            } else {
                string += "#";
            }
        }
        string += "\n";
    }

    string
}

pub mod part1 {
    use super::*;

    fn process(map: &Grid<bool>, list: &[(usize, usize)]) -> Vec<(usize, usize)> {
        let mut new_list = Vec::new();

        for node in list {
            let node = (node.0 as isize, node.1 as isize);
            for d in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let new_node = (node.0 + d.0, node.1 + d.1);
                if let Some(is_path) = map.get(new_node.0, new_node.1)
                    && *is_path
                {
                    let new_node = (new_node.0 as usize, new_node.1 as usize);
                    if !new_list.contains(&new_node) {
                        new_list.push(new_node);
                    }
                }
            }
        }

        new_list
    }

    pub fn solve(file: String) -> u64 {
        let (map, start) = parse(file);
        let mut list = vec![start];
        println!("{}", _display(&map, &list));

        for _i in 0..64 {
            list = process(&map, &list);
        }
        println!("{}", _display(&map, &list));

        list.len() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
