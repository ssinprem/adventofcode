pub fn mix(num1: u64, num2: u64) -> u64 {
    num1 ^ num2
}
pub fn prune(num: u64) -> u64 {
    num % 16777216
}

pub fn process(input: u64) -> u64 {
    // Calculate the result of multiplying the secret number by 64.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number.
    let mut temp = input;
    temp = mix(input, temp * 64);
    temp = prune(temp);
    // Calculate the result of dividing the secret number by 32.
    //    Round the result down to the nearest integer.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number.
    temp = mix(temp, temp / 32);
    temp = prune(temp);
    // Calculate the result of multiplying the secret number by 2048.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number.
    temp = mix(temp, temp * 2048);
    prune(temp)
}

pub mod part1 {
    use crate::process;

    pub fn solve(file: String) -> u64 {
        file.lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let temp = line.parse::<u64>().expect("cannot parse digit");
                (0..2000).fold(temp, |acc, _| process(acc))
            })
            .sum()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
