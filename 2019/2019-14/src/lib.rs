use std::collections::HashMap;

pub fn parse(file: String) -> HashMap<String, (isize, Vec<(String, isize)>)> {
    file.lines()
        .map(|line| {
            let (reactant, product) = line.split_once("=>").unwrap();
            let (amu, pro) = product.trim().split_once(" ").unwrap();
            let amu = amu.parse::<isize>().unwrap();

            let reactant = reactant
                .split(",")
                .map(|re| {
                    let (n, r) = re.trim().split_once(" ").unwrap();
                    (r.to_string(), n.parse::<isize>().unwrap())
                })
                .collect::<Vec<_>>();

            (pro.to_string(), (amu, reactant))
        })
        .collect()
}

#[allow(clippy::type_complexity)]
pub fn convert(
    needs: &mut HashMap<String, isize>,
    product: &String,
    formular: &HashMap<String, (isize, Vec<(String, isize)>)>,
) {
    let (amu, reactants) = &formular[product];
    let qty = needs[product];
    let batches = (qty + amu - 1) / amu;
    *needs.get_mut(product).unwrap() -= batches * amu;
    for (name, n) in reactants {
        *needs.entry(name.to_string()).or_insert(0) += batches * n;
    }
}

#[allow(clippy::type_complexity)]
pub fn ore_for_fuel(
    formular: &HashMap<String, (isize, Vec<(String, isize)>)>,
    fuel: isize,
) -> isize {
    let mut needs = HashMap::new();
    needs.insert("FUEL".to_string(), fuel);

    // find the product able to convert with effciailcy number
    while let Some(product) = needs
        .iter()
        .find(|(s, n)| s != &&"ORE".to_string() && **n > 0)
        .map(|(s, _)| s.to_string())
    {
        convert(&mut needs, &product, formular);
    }
    needs["ORE"]
}

pub mod part1 {
    use crate::{ore_for_fuel, parse};

    pub fn solve(file: String) -> usize {
        let formular = parse(file);
        ore_for_fuel(&formular, 1) as usize
    }
}

pub mod part2 {
    use crate::{ore_for_fuel, parse};

    pub fn solve(file: String) -> usize {
        let formular = parse(file);
        const ORE: isize = 1_000_000_000_000;

        let per_one = ore_for_fuel(&formular, 1);
        let (mut lo, mut hi) = (ORE / per_one, ORE);

        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            let num = ore_for_fuel(&formular, mid);
            if num <= ORE {
                lo = mid
            } else {
                hi = mid - 1;
            }
        }
        lo as usize
    }
}
