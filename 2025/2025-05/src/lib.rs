pub mod part1 {
    pub fn solve(file: String) -> u64 {
        if let Some((igd, order)) = file.split_once("\n\n") {
            order
                .lines()
                .map(|line| {
                    if let Ok(n) = line.parse::<u64>()
                        && igd.lines().any(|line| {
                            let (start, end) = line.split_once('-').unwrap();
                            let nstart = start.parse::<u64>().unwrap();
                            let nend = end.parse::<u64>().unwrap();
                            n >= nstart && n <= nend
                        })
                    {
                        return 1;
                    }
                    0
                })
                .sum()
        } else {
            0
        }
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        if let Some((igd, _)) = file.split_once("\n\n") {
            let mut sort_line = igd
                .lines()
                .map(|line| {
                    let (start, end) = line.split_once('-').unwrap();
                    let nstart = start.parse::<u64>().unwrap();
                    let nend = end.parse::<u64>().unwrap();
                    (nstart, nend)
                })
                .collect::<Vec<(u64, u64)>>();
            sort_line.sort_by(|(astart, _), (bstart, _)| astart.cmp(bstart));

            let mut cnt = 0;
            let (mut cur_start, mut cur_end) = sort_line.first().copied().unwrap();
            for &(new_start, new_end) in sort_line.iter().skip(1) {
                if cur_end < new_start {
                    cnt += cur_end - cur_start + 1;
                    cur_start = new_start;
                    cur_end = new_end;
                } else {
                    // extend the range
                    cur_end = cur_end.max(new_end);
                }
            }
            cnt += cur_end - cur_start + 1;
            cnt
        } else {
            0
        }
    }
}
