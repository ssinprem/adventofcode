use std::{cmp::Ordering, collections::HashMap, println};

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
    let detail = hands.iter().map(|(c, _n)| *c).collect();
    if hands[0].1 == 5 {
        (CardSet::FiveKind, detail)
    } else if hands[0].1 == 4 {
        (CardSet::FourKind, detail)
    } else if hands[0].1 == 3 && hands[1].1 == 2 {
        (CardSet::FullHourse, detail)
    } else if hands[0].1 == 3 {
        (CardSet::ThreeKind, detail)
    } else if hands[0].1 == 2 && hands[1].1 == 2 {
        (CardSet::TwoPair, detail)
    } else if hands[0].1 == 2 {
        (CardSet::OnePair, detail)
    } else {
        (CardSet::HighCard, detail)
    }
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
        println!("⚠️ Equal ?? {a} {b}");
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
            .inspect(|(r,(card, bet))| println!("{r} {card} {:?}",type_pts(card.as_str())))
            .map(|(rank, (_, bet))| *bet * (rank + 1) as u64)
            .sum::<u64>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
