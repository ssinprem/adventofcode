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
    pub fn solve(file: String, recipe: &[u8]) -> usize {
        let target = file
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();
        let target_len = target.len();
        let mut nums = recipe.to_vec();
        let mut pos_a = 0;
        let mut pos_b = 1;
        const STEP: usize = 100_000;
        for i in 0_usize.. {
            while nums.len() < (i + 1) * STEP {
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

            if let Some(found) = nums
                .windows(target_len)
                .enumerate()
                .skip(i * STEP)
                .find_map(|(n, v)| if v.to_vec() == target { Some(n) } else { None })
            {
                return found;
            }
        }
        0
    }
}
