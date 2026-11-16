pub struct Line {
    pub start: (isize, isize),
    pub end: (isize, isize),
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn contains(&self, p: &(isize, isize)) -> bool {
        let x_min = self.start.0.min(self.end.0);
        let x_max = self.start.0.max(self.end.0);
        let y_min = self.start.1.min(self.end.1);
        let y_max = self.start.1.max(self.end.1);

        (x_min..=x_max).contains(&p.0) && (y_min..=y_max).contains(&p.1)
    }

    pub fn cross(&self, other: &Line) -> Option<(isize, isize)> {
        let (v, h) = match (self.is_vertical(), other.is_vertical()) {
            (true, false) => (self, other),
            (false, true) => (other, self),
            _ => return None,
        };
        //              h (.0, .1)
        //              |
        // (.0, .1)     |
        //   v ---------+---------
        //              |
        //              |

        let p = (v.start.0, h.start.1);
        if v.contains(&p) && h.contains(&p) && p != (0, 0) {
            Some(p)
        } else {
            None
        }
    }
}

pub fn parse(file: String) -> Vec<Vec<Line>> {
    file.lines()
        .map(|line| {
            let mut curr = (0, 0);
            let mut vec = vec![];
            line.split(",").for_each(|step| {
                let (dir, num) = step.split_at(1);
                let num = num.parse::<isize>().unwrap();
                let next = match dir {
                    "U" => (curr.0, curr.1 - num),
                    "D" => (curr.0, curr.1 + num),
                    "L" => (curr.0 - num, curr.1),
                    "R" => (curr.0 + num, curr.1),
                    _ => unreachable!(),
                };
                vec.push(Line {
                    start: curr,
                    end: next,
                });
                curr = next;
            });
            vec
        })
        .collect()
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> usize {
        let lines = parse(file);
        let line_a = lines.first().unwrap();
        let line_b = lines.last().unwrap();

        let cross = line_a
            .iter()
            .flat_map(|a| line_b.iter().filter_map(|b| a.cross(b)))
            .collect::<Vec<_>>();

        cross
            .iter()
            .map(|p| p.0.abs_diff(0) + p.1.abs_diff(0))
            .min()
            .unwrap()
    }
}

pub mod part2 {
    use crate::Line;
    use crate::parse;

    fn gen_points(lines: &Vec<Line>) -> Vec<(isize, isize)> {
        let mut points = vec![];

        for line in lines {
            let start = line.start;
            let end = line.end;

            if points.is_empty() {
                points.push(start);
            }
            let mut curr = start;
            let diff = if line.is_vertical() {
                if start.1 < end.1 { (0, 1) } else { (0, -1) }
            } else {
                if start.0 < end.0 { (1, 0) } else { (-1, 0) }
            };
            while curr != end {
                curr = (curr.0 + diff.0, curr.1 + diff.1);
                points.push(curr);
            }
        }
        points
    }

    pub fn solve(file: String) -> usize {
        let lines = parse(file);
        let line_a = lines.first().unwrap();
        let line_b = lines.last().unwrap();
        let pts_a = gen_points(line_a);
        let pts_b = gen_points(line_b);

        let cross = line_a
            .iter()
            .flat_map(|a| line_b.iter().filter_map(|b| a.cross(b)))
            .map(|point| {
                (
                    point,
                    pts_a.iter().position(|p| p == &point).unwrap(),
                    pts_b.iter().position(|p| p == &point).unwrap(),
                )
            })
            .collect::<Vec<_>>();

        cross.iter().map(|p| p.1 + p.2).min().unwrap()
    }
}
