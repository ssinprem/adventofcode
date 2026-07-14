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
    use std::collections::{HashMap, HashSet};

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

    pub fn get_changes(prices: &[u64]) -> Vec<i64> {
        prices.windows(2)
            .map(|pair| {
                pair[1] as i64 - pair[0] as i64
            }).collect()
    }

    pub fn get_amount(prices: &[u64], changes: &[i64], seq: &[i64]) -> u64 {
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
        let secrets: Vec<u64> = file.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<u64>().expect("cannot parse digit"))
            .collect();

        let mut sequence_totals: HashMap<[i64; 4], u64> = HashMap::new();

        for &secret in &secrets {
            let prices = get_sell_price(secret);
            let changes: Vec<i64> = get_changes(&prices);

            let mut sold_sequences_for_buyer: HashSet<[i64; 4]> = HashSet::new();

            for (index, window) in changes.windows(4).enumerate() {
                if let Ok(sequence) = window.try_into() 
                    && sold_sequences_for_buyer.insert(sequence)
                {
                    let sale_price = prices[index + 4];
                    *sequence_totals.entry(sequence).or_default() += sale_price;
                }
            }
        }

        let max_seq = sequence_totals.iter().max_by_key(|(_seq,value)| *value).unwrap();
        println!("{max_seq:?}");
        *max_seq.1
    }
}
