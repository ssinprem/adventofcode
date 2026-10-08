pub fn process(list: &mut [usize], seqs: &[usize]) {
    let len = list.len();
    let mut index = 0;
    for (skip, &seq) in seqs.iter().enumerate() {
        let mut temp = Vec::new();
        for i in 0..seq {
            temp.push(*list.get((index + i) % len).unwrap());
        }
        for i in 0..seq {
            list[(index + i) % len] = temp.pop().unwrap();
        }
        index = (index + seq + skip) % len;
    }
}

pub mod part1 {
    use super::process;

    pub fn parse(file: String) -> Vec<usize> {
        file.split(",")
            .map(|str| str.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }

    pub fn solve(file: String, len: usize) -> usize {
        let seqs = parse(file);
        let mut list = (0..len).collect::<Vec<usize>>();
        process(&mut list, &seqs);
        list[0] * list[1]
    }
}

pub mod part2 {
    use super::process;

    pub fn parse(file: String) -> Vec<usize> {
        file.chars()
            .map(|char| char as usize)
            .collect::<Vec<usize>>()
    }

    fn _display(list: &[usize]) -> String {
        let mut output = String::new();
        output += "[\n";

        list.chunks(16).for_each(|chunk| {
            chunk
                .iter()
                .for_each(|item| output += format!("{item:4} ").as_str());
            output += "\n";
        });
        output + "]"
    }

    pub fn solve(file: String) -> String {
        let seqs = [parse(file), vec![17, 31, 73, 47, 23]].concat();
        let mut list = (0..256).collect::<Vec<usize>>();

        process(&mut list, &seqs.repeat(64));
        // println!("{}", _display(&list));

        list.chunks(16)
            .map(|chunk: &[usize]| {
                chunk
                    .iter()
                    .copied()
                    .reduce(|acc, item| acc ^ item)
                    .unwrap()
            })
            .map(|val| format!("{val:02x}"))
            .reduce(|string, str| string + str.as_str())
            .unwrap()
    }
}
