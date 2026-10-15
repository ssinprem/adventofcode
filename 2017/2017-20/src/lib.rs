

pub fn parse(file: String) -> Vec<(i64,i64,i64,i64,i64,i64,i64,i64,i64)> {
    use regex::Regex;
    let regex = Regex::new(r"p=<([-0-9]+),([-0-9]+),([-0-9]+)>, v=<([-0-9]+),([-0-9]+),([-0-9]+)>, a=<([-0-9]+),([-0-9]+),([-0-9]+)>").unwrap();

    file.lines().filter_map(|line| {
        if let Some(matched) = regex.captures(line)
        && let Some(px) = matched.get(1) && let Ok(px) = px.as_str().parse::<i64>()
        && let Some(py) = matched.get(2) && let Ok(py) = py.as_str().parse::<i64>()
        && let Some(pz) = matched.get(3) && let Ok(pz) = pz.as_str().parse::<i64>()
        && let Some(vx) = matched.get(4) && let Ok(vx) = vx.as_str().parse::<i64>()
        && let Some(vy) = matched.get(5) && let Ok(vy) = vy.as_str().parse::<i64>()
        && let Some(vz) = matched.get(6) && let Ok(vz) = vz.as_str().parse::<i64>()
        && let Some(ax) = matched.get(7) && let Ok(ax) = ax.as_str().parse::<i64>()
        && let Some(ay) = matched.get(8) && let Ok(ay) = ay.as_str().parse::<i64>()
        && let Some(az) = matched.get(9) && let Ok(az) = az.as_str().parse::<i64>()
        {
            
            Some((px,py,pz,vx,vy,vz,ax,ay,az))
        } else {
            None
        }
    }).collect()
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> usize {
        let list = parse(file);

        list.iter().enumerate()
            .min_by_key(|(_id,
                (   _px,_py,_pz,
                    _vx,_vy,_vz,
                    ax,ay,az
                ))|
            {
                ax.abs() + ay.abs() + az.abs()
            }).unwrap().0
    }
}

pub mod part2 {
    use crate::parse;
    pub fn solve(file: String) -> usize {
        let list = parse(file);
        let mut list = list.into_iter().map(|part| {
            Some(part)
        }).collect::<Vec<Option<_>>>();

        loop {
            for part in list.iter_mut() {
                if let Some((px,py,pz,vx,vy,vz,ax,ay,az)) = part
                {

                }
            }

            if list.iter().filter(|p| p.is_some()).count() <= 1 {
                break;
            }
        }


        list.iter()
        .position(| p | p.is_some())
        .unwrap()
    }
}
