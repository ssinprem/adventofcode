pub mod part1 {

    pub fn solve(target: usize) -> usize {
        let mut pos = (0,0);
        let mut direct = (1,0);
        let mut arow = 1;
        let mut cnt = 0;
        for _i in 2..=target {
            pos = (pos.0 + direct.0, pos.1 + direct.1);
            cnt += 1;
            if cnt == arow {
                cnt = 0;
                match direct {
                    (1,0) => { // right
                        direct = (0,-1);
                    },
                    (0,-1) => { // up
                        direct = (-1,0);
                        arow += 1;
                    },
                    (-1,0) => { // left
                        direct = (0,1);
                        
                    },
                    (0,1) => { // down
                        direct = (1,0);
                        arow += 1;
                    }
                    _ => unreachable!()
                }
            }
        }

        if pos.0 < 0 {
            pos.0 *= -1;
        }
        if pos.1 < 0 {
            pos.1 *= -1;
        }
        (pos.0 + pos.1) as usize
    }
}

pub mod part2 {
    use std::collections::HashMap;

    pub fn get_value(list: &HashMap<(i32, i32), usize>, pos: (i32,i32)) -> usize {
        list.into_iter().filter_map(|((x,y), value)| {

            if pos.0 >= x-1 && pos.0 <= x+1
            && pos.1 >= y-1 && pos.1 <= y+1
            {
                Some(value)
            } else {
                None
            }
        }).sum()
    }

    pub fn _display(list: &HashMap<(i32, i32), usize>) {
        let min_x = list.keys().map(|(x,_y)| *x).min().unwrap();
        let min_y = list.keys().map(|(_x,y)| *y).min().unwrap();
        let max_x = list.keys().map(|(x,_y)| *x).max().unwrap();
        let max_y = list.keys().map(|(_x,y)| *y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(value) = list.get(&(x,y)) {
                    print!("{value:8} ");
                } else {
                    print!("________ ");
                }
            }
            println!();
        }
    }

    pub fn solve(target: usize) -> usize {
        let mut pos = (0,0);
        let mut direct = (1,0);
        let mut arow = 1;
        let mut cnt = 0;
        let mut list = HashMap::new();
        let mut value = 1;
        list.insert((0,0), value);
        while value < target {
            pos = (pos.0 + direct.0, pos.1 + direct.1);
            value = get_value(&list, pos);
            list.insert(pos, value);
            cnt += 1;
            if cnt == arow {
                cnt = 0;
                match direct {
                    (1,0) => { // right
                        direct = (0,-1);
                    },
                    (0,-1) => { // up
                        direct = (-1,0);
                        arow += 1;
                    },
                    (-1,0) => { // left
                        direct = (0,1);
                        
                    },
                    (0,1) => { // down
                        direct = (1,0);
                        arow += 1;
                    }
                    _ => unreachable!()
                }
            }
        }

        _display(&list);
        value
    }
}
