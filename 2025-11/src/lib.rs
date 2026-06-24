pub fn parse(file: String) -> Vec<(String,String)> {
    file.lines().fold(vec![], |mut vec, line| {
        if let Some((src, dsts)) = line.split_once(":"){
            for dst in dsts.split(" ")
                .filter(|d| !d.is_empty()) {
                vec.push((src.to_string(), dst.to_string()));
            }
        }
        vec
    })
}

pub mod part1 {
    // use std::collections::HashMap;
    pub fn find_path(graph: &Vec<(String, String)>, src: &String, dst: &String, stack: Vec<String>) -> Option<Vec<Vec<String>>> {
        if src == dst {
            return Some(vec![vec![dst.to_string()]]);
        }
    
        let mut root = Vec::new();
        graph.iter().filter(|&node| *node.0 == *src)
        .for_each(|node| {
            // go to the same path, skip it.
            if stack.iter().any(|n| n == &node.1) {
                return;
            }
            let mut new_stack = stack.clone();
            new_stack.push(src.to_string());
            
            if let Some(paths) = find_path(graph, &node.1, dst, new_stack.clone()) {
                for next_path in paths {
                    if next_path.iter().any(|n| n == &node.0 ) {
                        continue;
                    }
                    let mut path = Vec::new();
                    path.push(node.0.to_string());
                    next_path.iter().for_each(|node| {
                        path.push(node.to_string());
                    });
                    root.push(path);
                }
            }
        });
    
        if !root.is_empty() {
            Some(root)
        } else {
            None
        }
    }

    pub fn solve(file: String) -> u64 {
        let graph = crate::parse(file);
        
        // let mut memo = HashMap::<(String, bool,bool), u64>::new();
        // crate::part2::dfs("you".to_string(), true, true, &graph, &mut memo)
        if let Some(paths) = 
            find_path(&graph, &"you".to_string(), &"out".to_string(), vec![]) {
            paths.len() as u64
        } else {
            unreachable!()
        }
    }
}

pub mod part2 {
    use std::collections::HashMap;

    pub fn dfs(
        node: String,
        has_dac: bool,
        has_fft: bool, 
        graph: &Vec<(String,String)>,
        memo: &mut HashMap<(String,bool,bool), u64>
    ) -> u64 {

        let has_dac = has_dac || node == "dac";
        let has_fft = has_fft || node == "fft";

        if node == "out" {
            if has_dac && has_fft {
                return 1;
            } else {
                return 0;
            }
        }

        if let Some(&count) = memo.get(&(node.to_string(),has_dac,has_fft)) {
            return count;
        }

        let count= graph.iter()
            .filter(|(src,_dst)| *src == *node)
            .map(|(_src, dst)| {
                dfs(dst.to_string(), has_dac, has_fft, graph, memo)
            }).sum();

        memo.insert((node,has_dac,has_fft), count);
        count
    }

    pub fn solve(file: String) -> u64 {
        let graph = crate::parse(file);
        let mut memo = HashMap::<(String, bool,bool), u64>::new();
        dfs("svr".to_string(), false, false, &graph, &mut memo)
    }
}
