pub mod part1 {
    use std::collections::VecDeque;

    pub fn run_node(series: &mut VecDeque<u64>) -> Vec<u64> {
        let node_cnt = series.pop_front().unwrap();
        let meta_cnt = series.pop_front().unwrap();
        let mut meta = vec![];

        for _ in 0..node_cnt {
            let m = run_node(series);
            meta = [meta, m].concat();
        }

        for _ in 0..meta_cnt {
            let v = series.pop_front().unwrap();
            meta.push(v);
        }

        meta
    }

    pub fn solve(file: String) -> u64 {
        let num = file
            .split_whitespace()
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<VecDeque<u64>>();
        let mut series = num.clone();
        run_node(&mut series).iter().sum::<u64>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
