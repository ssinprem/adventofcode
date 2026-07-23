use std::{cmp::Ordering, collections::HashMap};

#[derive(PartialEq, PartialOrd, Debug)]
pub enum CardSet {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHourse,
    FourKind,
    FiveKind,
}

const CARDS: [char; 15] = [
    ' ',' ','2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
pub fn parse(file: String) -> Vec<(String, u64)> {
    file.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (card, bet) = line.split_once(" ").unwrap();

            (card.to_string(), bet.parse::<u64>().unwrap())
        })
        .collect()
}

pub fn type_pts(card: &str) -> (CardSet, Vec<u32>) {
    let set = card
        .chars()
        .fold(HashMap::<char, u32>::new(), |mut acc, c| {
            acc.entry(c).and_modify(|n| *n += 1).or_insert(1);
            acc
        });

    let hands: Vec<_> = set.iter().collect();
    let mut hands: Vec<_> = hands
        .iter()
        .map(|(c, u)| (CARDS.iter().position(|x| x == *c).unwrap() as u32, **u))
        .collect();
    hands.sort_by(|(c1, n1), (c2, n2)| {
        let cmpn = n2.cmp(n1);
        if cmpn != Ordering::Equal {
            cmpn
        } else {
            c2.cmp(c1)
        }
    });
    ( match hands.iter().map(|(_c, n)| *n).collect::<Vec<u32>>().as_slice() {
        [5] => CardSet::FiveKind,
        [4,1] => CardSet::FourKind,
        [3,2] => CardSet::FullHourse,
        [3,1,1] => CardSet::ThreeKind,
        [2,2,1] => CardSet::TwoPair,
        [2,1,1,1] => CardSet::OnePair,
        [1,1,1,1,1] => CardSet::HighCard,
        _ => unreachable!()
    }, card.chars().map(|c| CARDS.iter().position(|x| *x==c).unwrap() as u32).collect() )
}

pub fn compare_hands(a: &str, b: &str) -> Ordering {
    let (a_set, a_value) = type_pts(a);
    let (b_set, b_value) = type_pts(b);
    if a_set > b_set {
        Ordering::Greater
    } else if a_set < b_set {
        Ordering::Less
    } else {
        for (acard, bcard) in a_value.iter().zip(b_value.iter()) {
            if acard > bcard {
                return Ordering::Greater;
            } else if acard < bcard {
                return Ordering::Less;
            }
        }
        // println!("⚠️ Equal ?? {a} {b}");
        Ordering::Equal
    }
}

pub mod part1 {
    use crate::*;
    pub fn solve(file: String) -> u64 {
        let mut hands = parse(file);
        hands.sort_by(|a, b| compare_hands(a.0.as_str(), b.0.as_str()));
        hands
            .iter()
            .enumerate()
            .inspect(|(r,(card, _bet))| println!("{r} {card} {:?}",type_pts(card.as_str())))
            .map(|(rank, (_, bet))| *bet * (rank + 1) as u64)
            .sum::<u64>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
