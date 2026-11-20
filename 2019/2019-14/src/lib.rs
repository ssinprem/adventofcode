use std::collections::HashMap;

pub fn parse(file: String) -> HashMap<String, (usize, Vec<(String, usize)>)> {
    file.lines().map(|line| {
        let (reactant,product) = line.split_once("=>").unwrap();
        let (amu,pro) = product.trim().split_once(" ").unwrap();
        let amu = amu.parse::<usize>().unwrap();
        
        let reactant = reactant.split(",").map(|re | {
            let (n,r) = re.trim().split_once(" ").unwrap();
            (r.to_string(), n.parse::<usize>().unwrap())
        }).collect::<Vec<_>>();

        (pro.to_string(), (amu, reactant))
    }).collect()
}

pub mod part1 {
    use std::collections::HashMap;

use crate::parse;

    pub fn solve(file: String) -> usize {
        let formular = parse(file);

        let mut needs = HashMap::new();
        needs.insert("FUEL".to_string(), 1_usize);

        println!("={needs:?}");

        while let Some((pro,n)) = needs.iter().find(|(s,n)|
                s != &&"ORE".to_string() &&
                **n >= formular.get(&s.to_string()).unwrap().0
        )
        {
            let (pro, n) = (pro.to_string(), *n);
            let (amu, reactants) = formular.get(&pro.to_string()).unwrap();
            needs.entry(pro).and_modify(|n| *n -= amu);
            needs.retain(|_name,c| *c != 0);
            for (re_str, re_amu) in reactants {
                needs.entry(re_str.to_string())
                    .and_modify(|n| *n += *re_amu)
                    .or_insert(*re_amu);
            }
            println!("={needs:?}");
        }

        

        *needs.get("ORE").unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
