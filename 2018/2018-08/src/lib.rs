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
    use std::collections::VecDeque;

    pub fn run_node(series: &mut VecDeque<u64>) -> u64 {
        let node_cnt = series.pop_front().unwrap();
        let meta_cnt = series.pop_front().unwrap();
        let mut nodes = vec![];
        let mut metas = vec![];

        for _ in 0..node_cnt {
            let n = run_node(series);
            nodes.push(n);
        }

        for _ in 0..meta_cnt {
            let v = series.pop_front().unwrap();
            metas.push(v);
        }

        if node_cnt == 0 {
            metas.iter().sum::<u64>()
        } else {
            metas
                .iter()
                .map(|index| nodes.get(*index as usize - 1).unwrap_or(&0))
                .sum()
        }
    }

    pub fn solve(file: String) -> u64 {
        let num = file
            .split_whitespace()
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<VecDeque<u64>>();
        let mut series = num.clone();
        run_node(&mut series)
    }
}
