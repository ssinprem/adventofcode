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
    part2: (part2::solve, 0)
);

#[test]
fn test_compare() {
    assert_eq!(compare_hands("KK677", "KK677"), Ordering::Equal);
    assert_eq!(compare_hands("KK677", "KTJJT"), Ordering::Greater);
    assert_eq!(compare_hands("KK677", "T55J5"), Ordering::Less);
    assert_eq!(compare_hands("QQQJA", "T55J5"), Ordering::Greater);
    assert_eq!(compare_hands("AAAKA", "AAQAA"), Ordering::Greater);
    assert_eq!(compare_hands("44344", "45444"), Ordering::Less);
    assert_eq!(compare_hands("QQQQA", "QQAQA"), Ordering::Greater);
    assert_eq!(compare_hands("22222", "QQQQA"), Ordering::Greater);
}
