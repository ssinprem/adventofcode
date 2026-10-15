use std::collections::HashMap;

pub fn get_size(string: &str) -> usize {
    (string.len() as f64).sqrt() as usize
}

pub fn get_array(string: &str) -> Vec<Vec<char>> {
    let vec = string.chars().collect::<Vec<char>>();
    let size = get_size(string);
    let mut array = vec![vec![' '; size]; size];
    for (i, c) in vec.iter().enumerate() {
        array[i/size][i%size] = *c;
    }

    array
}

pub fn array_to_string(array: &Vec<Vec<char>>) -> String {
    array.iter().map(|line| {
        line.iter().collect::<String>()
    }).collect::<String>()
}

pub fn flip_x(string: &str) -> String {
    let array = get_array(string);
    let mut new_array = array.clone();
    let size = get_size(string);
    for y in 0..size {
        for x in 0..size {
            new_array[y][x] = array[y][size-1-x];
        }
    }
    array_to_string(&new_array)
}

pub fn flip_y(string: &str) -> String {
    let array = get_array(string);
    let mut new_array = array.clone();
    let size = get_size(string);
    for y in 0..size {
        for x in 0..size {
            new_array[y][x] = array[size-1-y][x];
        }
    }
    array_to_string(&new_array)
}

pub fn flip_z(string: &str) -> String {
    let array = get_array(string);
    let mut new_array = array.clone();
    let size = get_size(string);
    for y in 0..size {
        for x in 0..size {
            new_array[y][x] = array[x][y];
        }
    }
    array_to_string(&new_array)
}

pub fn flip_a(string: &str) -> String {
    let array = get_array(string);
    let mut new_array = array.clone();
    let size = get_size(string);
    for y in 0..size {
        for x in 0..size {
            new_array[y][x] = array[size-1-x][size-1-y];
        }
    }
    array_to_string(&new_array)
}

pub fn rotate_r(string: &str) -> String {
    let array = get_array(string);
    let mut new_array = array.clone();
    let size = get_size(string);
    for y in 0..size {
        for x in 0..size {
            new_array[y][x] = array[x][size-1-y];
        }
    }
    array_to_string(&new_array)
}

pub fn rotate_l(string: &str) -> String {
    let array = get_array(string);
    let mut new_array = array.clone();
    let size = get_size(string);
    for y in 0..size {
        for x in 0..size {
            new_array[y][x] = array[size-1-x][y];
        }
    }
    array_to_string(&new_array)
}

pub fn rotate_h(string: &str) -> String {
    let array = get_array(string);
    let mut new_array = array.clone();
    let size = get_size(string);
    for y in 0..size {
        for x in 0..size {
            new_array[y][x] = array[size-1-y][size-1-x];
        }
    }
    array_to_string(&new_array)
}

pub fn string_split(string: &str) -> Vec<String> {
    let chars = string.chars().collect::<Vec<char>>();
    let size = get_size(string);
    let mut grps = Vec::new();
    let b = if size.is_multiple_of(2) {
        2
    } else if size.is_multiple_of(3) {
        3
    } else {
        unreachable!()
    };

    let num_blocks = size / b;
    
    for br in 0..num_blocks {
        for bc in 0..num_blocks {
            let mut block = String::new();
            for i in 0..b {
                let start_idx = (br * b + i) * size + (bc * b);
                let end_idx = start_idx + b;
                for char in chars.iter().take(end_idx).skip(start_idx) {
                    block += format!("{}", char).as_str();
                }
            }
            grps.push(block);
        }
    }
    grps
}

pub fn string_join(vec: Vec<String>) -> String {
    let new_b = vec.first().unwrap().len();
    let new_b = (new_b as f64).sqrt() as usize;
    let num_blocks_per_side = (vec.len() as f64).sqrt() as usize;
    let new_n = num_blocks_per_side * new_b;
    let mut result = String::with_capacity(new_n * new_n);

    for br in 0..num_blocks_per_side {
        for i in 0..new_b {
            for bc in 0..num_blocks_per_side {
                let block_idx = br * num_blocks_per_side + bc;
                let block = &vec[block_idx];
                
                let start = i * new_b;
                let end = start + new_b;
                result.push_str(&block[start..end]);
            }
        }
    }

    result
}

pub fn parse(file: String) -> HashMap<String, String> {
    let mut rule = HashMap::new();
    file.lines().for_each(|line| {
        let (key, val) = line.split_once("=>").unwrap();
        let key = key.to_string().chars().filter(|c| matches!(c, '.'|'#')).collect::<String>();
        let val = val.to_string().chars().filter(|c| matches!(c, '.'|'#')).collect::<String>();

        rule.insert(key.to_string(), val.to_string());
        rule.insert(flip_x(&key), val.to_string());
        rule.insert(flip_y(&key), val.to_string());
        rule.insert(flip_z(&key), val.to_string());
        rule.insert(flip_a(&key), val.to_string());
        rule.insert(rotate_r(&key), val.to_string());
        rule.insert(rotate_l(&key), val.to_string());
        rule.insert(rotate_h(&key), val.to_string());
    });

    rule
}

pub mod part1 {
    use crate::{parse, string_join, string_split};

    pub fn solve(file: String, initial: String, iteration: usize) -> usize {
        let rule = parse(file);
        let mut list = initial;
        for _ in 0..iteration {
            let mut group = string_split(&list);

            group = group.iter().map(|string| {
                rule.get(string).unwrap().to_string()
            }).collect::<Vec<_>>();

            list = string_join(group);
            println!("{list:?}");
        }

        list.chars().filter(|c| c==&'#').count()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
