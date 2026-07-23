use std::{assert_eq, cmp::Ordering};

use camel_card::*;
use utility::test_case_n;

test_case_n!(example,
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
    part1: (part1::solve, 6440),
    part2: (part2::solve, 5905)
);

#[test]
fn test_compare() {
    assert_eq!(
        Ordering::Equal,
        compare_hands(part1::type_pts("KK677"), part1::type_pts("KK677"))
    );
    assert_eq!(
        Ordering::Greater,
        compare_hands(part1::type_pts("KK677"), part1::type_pts("KTJJT"))
    );
    assert_eq!(
        Ordering::Less,
        compare_hands(part1::type_pts("KK677"), part1::type_pts("T55J5"))
    );
    assert_eq!(
        Ordering::Greater,
        compare_hands(part1::type_pts("QQQJA"), part1::type_pts("T55J5"))
    );
    assert_eq!(
        Ordering::Greater,
        compare_hands(part1::type_pts("AAAKA"), part1::type_pts("AAQAA"))
    );
    assert_eq!(
        Ordering::Less,
        compare_hands(part1::type_pts("44344"), part1::type_pts("45444"))
    );
    assert_eq!(
        Ordering::Less,
        compare_hands(part1::type_pts("QQQAA"), part1::type_pts("KQQQK"))
    );
    assert_eq!(
        Ordering::Greater,
        compare_hands(part1::type_pts("22222"), part1::type_pts("QQQQA"))
    );
}
