pub mod part1 {

    fn _display(rows: &Vec<Vec<bool>>) -> String {
        let mut output = String::new();
        for row in rows {
            for cell in row {
                output += if *cell {
                    "."
                } else {
                    "^"
                };
            }
            output+="\n";
        }
        output
    }

    pub fn solve(file: String, target_rows: usize) -> u64 {
        let first = file.chars().map(|char|
            match char {
            '.' => true,    // safe
            '^' => false,   // trap
            _ => unreachable!()
        }).collect::<Vec<bool>>();

        let mut rows = vec![first];

        while rows.len() < target_rows {
            let last = rows.last().unwrap();
            let size = last.len();
            let mut new_row = vec![];

            new_row.push(!matches!((last[0], last[1]), (false, false) | (true, false)));
            last.windows(3).for_each(|grp| {
                new_row.push(match (grp[0],grp[1],grp[2]) {
                    (false, false, true) => false,
                    (true, false, false) => false,
                    (false, true, true) => false,
                    (true, true, false) => false,
                    _ => true // safe
                });
            });
            new_row.push(!matches!((last[size-2], last[size-1]), (false, false) | (false, true)));
            rows.push(new_row);
        }
        println!("{}",_display(&rows));
        rows.iter().map(
            |row| row.iter().filter(|cell| **cell).count()
        ).sum::<usize>() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
