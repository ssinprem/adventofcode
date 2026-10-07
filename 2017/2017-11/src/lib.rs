pub fn cal_steps(pos: (i32, i32)) -> u32 {
    let abs1 = pos.0.abs();
    let abs2 = pos.1.abs();
    let dianog = abs1.min(abs2);
    let nondinog = (abs1.max(abs2) - dianog) / 2;
    (dianog + nondinog) as u32
}
pub mod part1 {
    use super::cal_steps;

    pub fn solve(file: String) -> u32 {
        let mut pos = (0, 0);

        file.split(",").for_each(|dir| match dir {
            "n" => pos = (pos.0, pos.1 + 2),
            "s" => pos = (pos.0, pos.1 - 2),
            "ne" => pos = (pos.0 + 1, pos.1 + 1),
            "sw" => pos = (pos.0 - 1, pos.1 - 1),
            "nw" => pos = (pos.0 - 1, pos.1 + 1),
            "se" => pos = (pos.0 + 1, pos.1 - 1),
            _ => unreachable!(),
        });

        cal_steps(pos)
    }
}

pub mod part2 {
    use std::collections::HashSet;
    use super::cal_steps;

    pub fn solve(file: String) -> u32 {
        let mut pos = (0, 0);
        let mut history = HashSet::new();
        file.split(",").for_each(|dir| {
            match dir {
                "n" => pos = (pos.0, pos.1 + 2),
                "s" => pos = (pos.0, pos.1 - 2),
                "ne" => pos = (pos.0 + 1, pos.1 + 1),
                "sw" => pos = (pos.0 - 1, pos.1 - 1),
                "nw" => pos = (pos.0 - 1, pos.1 + 1),
                "se" => pos = (pos.0 + 1, pos.1 - 1),
                _ => unreachable!(),
            }
            history.insert(pos);
        });

        history.iter().map(|pos| {
            cal_steps(*pos)
        }).max().unwrap()
    }
}
