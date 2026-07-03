use regex::*;

#[allow(clippy::type_complexity)]
pub fn parse(file: String) -> Vec<((i32,i32),(i32,i32),(i32,i32))> {
    let reg = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)"
    ).unwrap();
    
    reg.captures_iter(file.as_str()).map(|cap| {
        (
            (
                cap.get(1).unwrap().as_str().parse().expect("cannot parse"),
                cap.get(2).unwrap().as_str().parse().expect("cannot parse")
            ),
            (
                cap.get(3).unwrap().as_str().parse().expect("cannot parse"),
                cap.get(4).unwrap().as_str().parse().expect("cannot parse"),
            ),
            (
                cap.get(5).unwrap().as_str().parse().expect("cannot parse"),
                cap.get(6).unwrap().as_str().parse().expect("cannot parse")
            )
        )
    }).collect::<Vec<((i32,i32),(i32,i32),(i32,i32))>>()
}


// find A,B   A*ax + B*bx = rx
//            A*ay + B*by = ry
pub fn ab_solve(ax:i32,bx:i32,rx:i32,ay:i32,by:i32,ry:i32) -> Option<(i32,i32)> {
    let deter
     = (ax*by) - (bx*ay);

    let a = (rx*by - ry*bx) / deter;
    let b = (ry*ax - rx*ay) / deter;
    // a  b  e  c  d  f
    // ax bx rx ay by ry

    if a*ax+b*bx==rx && a*ay+b*by==ry {
        Some((a ,b))
    } else {
        None
    }
}

pub mod part1 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let datas = parse(file);

        datas.iter()
        .inspect(|d| println!("{d:?}"))
        .filter_map(|(
            (ax,ay),
            (bx,by),
            (rx,ry))
        | ab_solve(*ax, *bx, *rx, *ay, *by, *ry))
        .inspect(|d| println!("{d:?}"))
        .filter_map(|(a,b)| {
            if a>=0 && a <=100 && b>=0 && b<=100 {
                Some(a*3 + b)
            } else {
                None
            }
        })
        .sum::<i32>() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
