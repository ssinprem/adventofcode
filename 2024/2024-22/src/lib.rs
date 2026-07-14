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
    use crate::process;

    pub fn get_sell_price(secret: u64) -> Vec<u64> {
        let mut prices = Vec::with_capacity(2001);
        let mut current_secret = secret;

        prices.push(current_secret % 10);

        for _ in 0..2000 {
            current_secret = process(current_secret);
            prices.push(current_secret % 10);
        }
        prices
    }

    pub fn get_changes(prices: &Vec<u64>) -> Vec<i64> {
        prices.windows(2)
            .map(|pair| {
                pair[1] as i64 - pair[0] as i64
            }).collect()
    }

    pub fn get_amount(prices: &Vec<u64>, changes: &Vec<i64>, seq: &[i64]) -> u64 {
         if let Some(index) = changes.windows(4).position(|window| {
            window == seq
        }) {
            // The sequence of 4 changes starts at `index` in the `changes` array.
            // changes[index] = prices[index+1] - prices[index]
            // ...
            // changes[index+3] = prices[index+4] - prices[index+3]
            // The price at the moment the sequence is complete is prices[index+4].
            prices[index + 4]
        } else {
            0
        }
    }

    pub fn solve(file: String) -> u64 {
        println!("Parse secrets from file");
        let secrets = file.lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.parse::<u64>().expect("cannot parse digit")
            }).collect::<Vec<u64>>();

        let mut max = 0;
        println!("Calculate Sell Price");
        let price_set: Vec<Vec<u64>> = secrets.iter().map(|&secret | {
            get_sell_price(secret)
        }).collect();

        println!("Calculate Change");
        let changes_set: Vec<Vec<i64>> = price_set.iter().map(|prices| {
            get_changes(prices)
        }).collect();

        println!("Find the Best Sequence");
        for s1 in -9..=9 { 
            for s2 in -9..=9 {
                for s3 in -9..=9 {
                    for s4 in -9..=9 {
                        let amount = (0..price_set.len()).map(|id| {
                            get_amount(
                                &price_set[id], 
                                &changes_set[id], 
                                &[s1,s2,s3,s4]
                            )
                        }).sum();
                        if amount > max {
                            max = amount;
                            println!(" {amount}  {:?} ",[s1,s2,s3,s4]);
                        }
                    }
                }
            }
        }
        max
    }
}
