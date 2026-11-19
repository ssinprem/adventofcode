pub fn parse(file: String) -> Vec<(usize, usize)> {
    let mut list = Vec::new();
    file.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if char == '#' {
                list.push((x, y));
            }
        });
    });
    list
}

pub fn _display(list: &[((usize, usize), usize)]) {
    let max_x = list.iter().map(|((x, _), _)| *x).max().unwrap();
    let max_y = list.iter().map(|((_, y), _)| *y).max().unwrap();
    let max_v = list.iter().map(|((_, _), v)| *v).max().unwrap();
    let size = format!("{max_v}").len();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let mut str;
            if let Some(((_, _), v)) = list.iter().find(|((tx, ty), _v)| tx == &x && ty == &y) {
                str = format!("{}", *v);
                while str.len() < size {
                    str = " ".to_string() + str.as_str();
                }
                print!("{str}");
            } else {
                str = "_".to_string();
                while str.len() < size {
                    str = "_".to_string() + str.as_str();
                }
                print!("{str}");
            }
        }
        println!();
    }
}

pub mod part1 {
    use crate::{_display, parse};
    use std::cmp::Reverse;

    pub fn solve(file: String) -> ((usize, usize), usize) {
        let list = parse(file);
        let candidate = list.clone();
        let mut candidate = candidate
            .iter()
            .map(|(cx, cy)| {
                let mut diff = list
                    .iter()
                    .map(|(tx, ty)| (*tx as isize - *cx as isize, *ty as isize - *cy as isize))
                    .collect::<Vec<_>>();
                diff.sort_by_key(|(dx, dy)| dx.abs() + dy.abs());
                diff.remove(0);
                let mut target = vec![];

                let fn_base = |dx: isize, dy: isize| {
                    let div = utility::gcd(dx.unsigned_abs(), dy.unsigned_abs()) as isize;
                    (dx / div, dy / div)
                };
                for (dx, dy) in diff {
                    let mut mul = 1;
                    let mut block = false;
                    let base = fn_base(dx, dy);

                    loop {
                        let space = (base.0 * mul, base.1 * mul);
                        if target.contains(&space) {
                            block = true;
                            break;
                        }
                        if space == (dx, dy) {
                            break;
                        }
                        mul += 1;
                    }
                    if !block {
                        target.push((dx, dy));
                    }
                }
                ((*cx, *cy), target.len())
            })
            .collect::<Vec<_>>();

        candidate.sort_by_key(|((cx, cy), count)| (Reverse(*count), Reverse(*cy), Reverse(*cx)));

        _display(&candidate);

        *candidate.first().unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
