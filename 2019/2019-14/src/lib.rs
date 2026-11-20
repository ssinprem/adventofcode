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

pub mod part1 {
    use std::collections::HashMap;

    use crate::parse;

    fn convert(
        needs: &mut HashMap<String, isize>,
        product: &String,
        formular: &HashMap<String, (isize, Vec<(String, isize)>)>,
    ) {
        let (amu, reactants) = formular.get(&product.to_string()).unwrap();
        needs.entry(product.to_string()).and_modify(|n| *n -= amu);
        needs.retain(|_name, c| *c != 0);
        for (re_str, re_amu) in reactants {
            needs
                .entry(re_str.to_string())
                .and_modify(|n| *n += *re_amu)
                .or_insert(*re_amu);
        }
    }

    pub fn solve(file: String) -> usize {
        let formular = parse(file);

        let mut needs = HashMap::new();
        needs.insert("FUEL".to_string(), 1_isize);

        while needs
            .iter()
            .filter(|(s, _n)| s != &&"ORE".to_string())
            .any(|(_s, n)| *n > 0)
        {
            // find the product able to convert with effciailcy number
            while let Some(product) = needs
                .iter()
                .find(|(s, n)| {
                    s != &&"ORE".to_string() && **n >= formular.get(&s.to_string()).unwrap().0
                })
                .map(|(s, _)| s.to_string())
            {
                convert(&mut needs, &product, &formular);
            }

            // find the product able to convert, with non-efficialcy
            if let Some(product) = needs
                .iter()
                .find(|(s, n)| s != &&"ORE".to_string() && n > &&0)
                .map(|(s, _)| s.to_string())
            {
                convert(&mut needs, &product, &formular);
            }
        }

        *needs.get("ORE").unwrap() as usize
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
