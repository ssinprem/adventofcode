pub(crate) fn solven(file: String, target: usize) -> u64 {
    file.lines()
    .map(|line: &str| {
        // The total number of deletions we are allowed to make
        let mut del_allown = line.len() - target;
        let mut stack = Vec::<u64>::new();

        line.chars().for_each(|c| {
            let n = c.to_digit(10).unwrap_or(0) as u64;
            // While the stack isn't empty, 
            // AND the current digit is larger than the top of the stack,
            // AND we still have deletions left to use:
            while ! stack.is_empty() &&
                Some(n) > stack.last().copied() &&
                del_allown > 0
            {
                // Pop the smaller digit out; we are choosing to delete it
                _ = stack.pop();
                del_allown -= 1;
            }
            // Push the current digit onto the stack
            stack.push(n);
        });
        // If we still have deletions left after looking at all digits,
        // truncate the remaining allowed deletions from the end of the stack
        while del_allown > 0 {
            _ = stack.pop();
            del_allown -= 1;
        }
        // Convert the first n elements of the stack into an integer
        stack.truncate(target); 
        stack.iter().fold(0, |acc, &n| acc*10 + n )
    })
    // .inspect(|f| {println!("{f}")})
    .sum::<u64>()
}

pub mod part1 {
    pub fn solve(file: String) -> u64 {
        crate::solven(file, 2)
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        crate::solven(file, 12)
    }
}
