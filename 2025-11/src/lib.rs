pub fn parse(file: String) -> Vec<(String,String)> {
    file.lines().fold(vec![], |mut vec, line| {
        if let Some((src, dsts)) = line.split_once(":"){
            // print!("Src '{src}' ");
            // print!("Dsts ");
            let mut dst_iter = dsts.split(" ").into_iter()
                .filter(|d| !d.is_empty());
            while let Some(dst) = dst_iter.next(){
                // print!("'{dst}' ");
                vec.push((src.to_string(), dst.to_string()));
            }
            // println!();
        }
        vec
    })
}

pub fn find_path(graph: &Vec<(String, String)>, src: &String, dst: &String, stack: Vec<String>) -> Option<Vec<Vec<String>>> {
    if src == dst {
        println!("Found dst {stack:?} {dst}");
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
        
        if let Some(paths) = find_path(&graph, &node.1, &dst, new_stack.clone()) {
            for next_path in paths {
                if next_path.iter().any(|n| n == &node.0 ) {
                    continue;
                }
                let mut path = Vec::new();
                path.push(node.0.to_string());
                next_path.iter().for_each(|node| {
                    path.push(node.to_string());
                });
                // println!("backtrack {path:?}");
                root.push(path);
            }
        }
    });

    if root.len() > 0 {
        println!("traceback {} {src} {dst}", root.len());
        Some(root)
    } else {
        None
    }
}

pub mod part1 {
    pub fn solve(file: String) -> usize {
        let graph = crate::parse(file);
        if let Some(paths) = 
            crate::find_path(&graph, &"you".to_string(), &"out".to_string(), vec![]) {
            paths.iter().for_each(|path| { 
                println!("{path:?}");
            });
            paths.len()
        } else {
            unreachable!()
        }
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        let graph = crate::parse(file);
        if let Some(mut paths) = crate::find_path(&graph, &"svr".to_string(), &"out".to_string(), vec![]) {
            println!("===== Paths =====");
            paths.iter().for_each(|path| { 
                println!("{path:?}");
            });
            paths = paths.iter().filter(|&cpaths| {
                cpaths.iter().any(|p| *p == "fft") &&
                cpaths.iter().any(|p| *p == "dac")
            }).map(|ps| ps.clone()).collect();
            println!("===== Valid Paths =====");
            paths.iter().for_each(|path| { 
                println!("{path:?}");
            });
            paths.len() as u64
        } else {
            unreachable!()
        }
    }
}
