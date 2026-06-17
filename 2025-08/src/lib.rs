
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct JBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JBox {
    pub fn new(c: Vec<String>) -> Self{
        Self {
            x: c.first().unwrap().parse::<i64>().unwrap_or(0),
            y: c.get(1).unwrap().parse::<i64>().unwrap_or(0),
            z: c.get(2).unwrap().parse::<i64>().unwrap_or(0),
        }
    }

    pub fn dist(&self, b: JBox) -> u64 {
        (
            (self.x-b.x) * (self.x-b.x) + 
            (self.y-b.y) * (self.y-b.y) +
            (self.z-b.z) * (self.z-b.z) 
        ) as u64
    }
}

pub mod part1 {
    use crate::JBox;
    use std::{collections::{HashMap, HashSet}};

    pub fn solve(file: String, cnt: u64) -> u64 {
        let mut dists  = HashMap::<(JBox,JBox),u64>::new();
        let jboxs: Vec::<JBox> = file.lines().filter(|line| !line.is_empty())
            .map(|line| {
                let str_arr = line.splitn(3,",")
                    .map(|str| str.to_string())
                    .collect::<Vec<String>>();
                JBox::new(str_arr)
            }
            )
            .collect();

        for i in 0..jboxs.len() {
            let ibox = jboxs.get(i).unwrap();
            for j in i+1..jboxs.len() {
                let jbox = jboxs.get(j).unwrap();
                dists.insert((*ibox,*jbox), ibox.dist(*jbox));
            }
        }
        println!("jbox : {} boxs", jboxs.len());
        println!("possible pairs : {}", dists.len());
        let mut circuits = Vec::<HashSet<JBox>>::new();
        let mut rank: Vec<((JBox, JBox), u64)> = dists.into_iter().collect();
        rank.sort_by_key(|item| item.1);
        
        let threshold_dist = if rank.len() >= cnt as usize {
            rank[cnt as usize - 1].1
        } else {
            rank.last().map_or(0, |(_, dist)| *dist)
        };

        for entry in rank.iter().take_while(|(_, dist)| *dist <= threshold_dist) {
            let key = entry.0;
            if circuits.iter().any(|c| c.contains(&key.0) || c.contains(&key.1))
            {
                let exists = circuits.iter().filter(|c| c.contains(&key.0) ||  c.contains(&key.1)).collect::<Vec<&HashSet<JBox>>>();
                let mut  new_circuit = exists.iter().fold(
                    HashSet::new(), |mut new, circuit| {
                        for jbox in circuit.iter() {
                            new.insert(*jbox);
                        }
                        new
                });
                new_circuit.insert(key.0);
                new_circuit.insert(key.1);
                
                // println!("merged [{}] {:?}", new_circuit.len(), new_circuit);
                circuits = circuits.iter().filter(|target| !exists.contains(target))
                    .cloned()
                    .collect::<Vec<HashSet<JBox>>>();
                circuits.push(new_circuit);
            } else {
                let mut circuit = HashSet::<JBox>::new();
                circuit.insert(key.0);
                circuit.insert(key.1);
                // println!("new        {:?}", circuit);
                circuits.push(circuit);
            }
        }

        for jbox in jboxs {
            if circuits.iter().all(|circuit| !circuit.contains(&jbox)) {
                let mut circuit = HashSet::new();
                circuit.insert(jbox);
                circuits.push(circuit);
            }
        }

        circuits.sort_by_key(|c| std::cmp::Reverse(c.len()));
        (0..3.min(circuits.len())).map(|index| {
            if let Some(circuit) = circuits.get(index)  {
                circuit.len() as u32
            } else {
                0
            }
        })
            .inspect(|len| {print!("{len} ");})
            .product::<u32>()
            .into()
    }
}

pub mod part2 {
    use std::{collections::{HashMap, HashSet}};
    use crate::JBox;

    pub fn solve(file: String) -> u64 {
        let mut dists  = HashMap::<(JBox,JBox),u64>::new();
        let jboxs: Vec::<JBox> = file.lines().filter(|line| !line.is_empty())
            .map(|line| {
                let str_arr = line.splitn(3,",")
                    .map(|str| str.to_string())
                    .collect::<Vec<String>>();
                JBox::new(str_arr)
            }
            )
            .collect();
        let size = jboxs.len();
        for i in 0..size {
            let ibox = jboxs.get(i).unwrap();
            for j in i+1..jboxs.len() {
                let jbox = jboxs.get(j).unwrap();
                dists.insert((*ibox,*jbox), ibox.dist(*jbox));
            }
        }
        println!("jbox : {} boxs", size);
        println!("possible pairs : {}", dists.len());
        let mut circuits = Vec::<HashSet<JBox>>::new();
        let mut rank: Vec<((JBox, JBox), u64)> = dists.into_iter().collect();
        rank.sort_by(|a,b| b.1.cmp(&a.1));

        while let Some(entry) = rank.pop() {
            // println!("{:?}", entry);
            let key = entry.0;
            if circuits.iter().any(|c| c.contains(&key.0) || c.contains(&key.1))
            {
                let exists = circuits.iter().filter(|c| c.contains(&key.0) ||  c.contains(&key.1)).collect::<Vec<&HashSet<JBox>>>();
                let mut  new_circuit = exists.iter().fold(
                    HashSet::new(), |mut new, circuit| {
                        for jbox in circuit.iter() {
                            new.insert(*jbox);
                        }
                        new
                });
                new_circuit.insert(key.0);
                new_circuit.insert(key.1);
                
                // println!("merged [{}] {:?}", new_circuit.len(), new_circuit);
                circuits = circuits.iter().filter(|target| !exists.contains(target))
                    .cloned()
                    .collect::<Vec<HashSet<JBox>>>();
                if new_circuit.len() == size {
                    println!("{:?}", key);
                    return (key.0.x * key.1.x) as u64
                }
                circuits.push(new_circuit);

            } else {
                let mut circuit = HashSet::<JBox>::new();
                circuit.insert(key.0);
                circuit.insert(key.1);
                // println!("new        {:?}", circuit);
                circuits.push(circuit);
            }
        }

        0
    }
}
