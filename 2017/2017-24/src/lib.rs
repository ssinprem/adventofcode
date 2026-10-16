use std::collections::HashSet;

pub fn parse(file: String) -> HashSet<(u32, u32)> {
    file.lines()
        .map(|line| {
            let (a, b) = line.split_once("/").unwrap();

            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();

            if a < b { (a, b) } else { (b, a) }
        })
        .collect::<HashSet<(u32, u32)>>()
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> u32 {
        let lists = parse(file);
        let mut stacks = Vec::new();
        let mut done: Vec<Vec<(u32, u32)>> = Vec::new();

        stacks.push((vec![], lists.clone(), 0));
        while let Some((proc, list, need)) = stacks.pop() {
            done.push(proc.clone());
            for next in list.clone() {
                let mut new_proc = proc.clone();
                let mut new_list = list.clone();
                if next.0 == need {
                    new_list.remove(&next);
                    new_proc.push(next);
                    stacks.push((new_proc, new_list, next.1))
                } else if next.1 == need {
                    new_list.remove(&next);
                    new_proc.push(next);
                    stacks.push((new_proc, new_list, next.0))
                }
            }
        }

        done.iter()
            .map(|list| {
                list.iter().map(|(a, b)| *a + *b).sum()
            })
            .max()
            .unwrap()
    }
}

pub mod part2 {
    use crate::parse;

    pub fn solve(file: String) -> u32 {
        let lists = parse(file);
        let mut stacks = Vec::new();
        let mut done: Vec<Vec<(u32, u32)>> = Vec::new();

        stacks.push((vec![], lists.clone(), 0));
        while let Some((proc, list, need)) = stacks.pop() {
            done.push(proc.clone());
            for next in list.clone() {
                let mut new_proc = proc.clone();
                let mut new_list = list.clone();
                if next.0 == need {
                    new_list.remove(&next);
                    new_proc.push(next);
                    stacks.push((new_proc, new_list, next.1))
                } else if next.1 == need {
                    new_list.remove(&next);
                    new_proc.push(next);
                    stacks.push((new_proc, new_list, next.0))
                }
            }
        }

        let max_lenght = done.iter().map(|list| list.len()).max().unwrap();

        done.iter()
            .filter(|list| list.len() == max_lenght)
            .map(|list| {
                list.iter().map(|(a, b)| *a + *b).sum()
            })
            .max()
            .unwrap()
    }
}
