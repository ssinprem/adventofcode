pub mod part1 {
    pub fn solve(file: String, recipe: &[u8]) -> String {
        let after = file.parse::<usize>().unwrap();
        let mut nums = recipe.to_vec();
        let mut pos_a = 0;
        let mut pos_b = 1;

        while nums.len() <= after + 10 {
            let num_a = nums[pos_a];
            let num_b = nums[pos_b];
            let sum = num_a + num_b;

            if sum >= 10 {
                nums.push(sum / 10);
            }
            nums.push(sum % 10);
            pos_a = (pos_a + num_a as usize + 1) % nums.len();
            pos_b = (pos_b + num_b as usize + 1) % nums.len();
        }

        nums.drain(after..after + 10)
            .map(|n| format!("{n}"))
            .collect()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
