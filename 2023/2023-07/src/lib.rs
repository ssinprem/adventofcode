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

pub fn parse(file: String) -> Vec<(String, u64)> {
    file.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (card, bet) = line.split_once(" ").unwrap();

            (card.to_string(), bet.parse::<u64>().unwrap())
        })
        .collect()
}

pub fn compare_hands(a: (CardSet, Vec<u32>), b: (CardSet, Vec<u32>)) -> Ordering {
    let (a_set, a_value) = (a.0, a.1);
    let (b_set, b_value) = (b.0, b.1);
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
    const CARDS: [char; 15] = [
        ' ', ' ', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
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
        (
            match hands
                .iter()
                .map(|(_c, n)| *n)
                .collect::<Vec<u32>>()
                .as_slice()
            {
                [5] => CardSet::FiveKind,
                [4, 1] => CardSet::FourKind,
                [3, 2] => CardSet::FullHourse,
                [3, 1, 1] => CardSet::ThreeKind,
                [2, 2, 1] => CardSet::TwoPair,
                [2, 1, 1, 1] => CardSet::OnePair,
                [1, 1, 1, 1, 1] => CardSet::HighCard,
                _ => unreachable!(),
            },
            card.chars()
                .map(|c| CARDS.iter().position(|x| *x == c).unwrap() as u32)
                .collect(),
        )
    }

    pub fn solve(file: String) -> u64 {
        let mut hands = parse(file);
        hands.sort_by(|a, b| compare_hands(type_pts(a.0.as_str()), type_pts(b.0.as_str())));
        hands
            .iter()
            .enumerate()
            .inspect(|(r, (card, _bet))| println!("{r} {card} {:?}", type_pts(card.as_str())))
            .map(|(rank, (_, bet))| *bet * (rank + 1) as u64)
            .sum::<u64>()
    }
}

pub mod part2 {
    use crate::*;

    const CARDS: [char; 14] = [
        ' ', 'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

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
        let j_pts = CARDS.iter().position(|x| *x == 'J').unwrap();
        let mut count_j = 0;
        if let Some(j) = hands.iter_mut().find(|(c, _n)| *c as usize == j_pts) {
            count_j = j.1;
        }
        if count_j > 0 {
            hands.retain(|(c, _n)| *c as usize != j_pts);
            if let Some(first) = hands.first_mut() {
                first.1 += count_j
            } else {
                hands = vec![(j_pts as u32, 5)];
            }
        }
        (
            match hands
                .iter()
                .map(|(_c, n)| *n)
                .collect::<Vec<u32>>()
                .as_slice()
            {
                [5] => CardSet::FiveKind,
                [4, 1] => CardSet::FourKind,
                [3, 2] => CardSet::FullHourse,
                [3, 1, 1] => CardSet::ThreeKind,
                [2, 2, 1] => CardSet::TwoPair,
                [2, 1, 1, 1] => CardSet::OnePair,
                [1, 1, 1, 1, 1] => CardSet::HighCard,
                _ => unreachable!(),
            },
            card.chars()
                .map(|c| CARDS.iter().position(|x| *x == c).unwrap() as u32)
                .collect(),
        )
    }
    pub fn solve(file: String) -> u64 {
        let mut hands = parse(file);
        hands.sort_by(|a, b| compare_hands(type_pts(a.0.as_str()), type_pts(b.0.as_str())));
        hands
            .iter()
            .enumerate()
            .inspect(|(r, (card, _bet))| println!("{r} {card} {:?}", type_pts(card.as_str())))
            .map(|(rank, (_, bet))| *bet * (rank + 1) as u64)
            .sum::<u64>()
    }
}
