use permutaion_promenade::*;
use utility::test_case_n;

test_case_n!(example,
"s1,x3/4,pe/b",
    part1: (part1::solve, "baedc", 5),
    part2: (part2::solve, 0)
);

#[test]
fn test_function() {
    assert_eq!(do_spin("abcde".to_string(), 1), "eabcd");
    assert_eq!(do_spin("abcde".to_string(), 2), "deabc");
    assert_eq!(do_spin("abcde".to_string(), 3), "cdeab");
    assert_eq!(do_spin("abcde".to_string(), 4), "bcdea");

    assert_eq!(do_exchange("abcde".to_string(), 0, 1), "bacde");
    assert_eq!(do_exchange("abcde".to_string(), 0, 2), "cbade");
    assert_eq!(do_exchange("abcde".to_string(), 1, 3), "adcbe");
    assert_eq!(do_exchange("abcde".to_string(), 2, 4), "abedc");

    assert_eq!(do_partner("abcde".to_string(), 'a', 'b'), "bacde");
    assert_eq!(do_partner("abcde".to_string(), 'a', 'c'), "cbade");
    assert_eq!(do_partner("abcde".to_string(), 'b', 'd'), "adcbe");
    assert_eq!(do_partner("abcde".to_string(), 'c', 'e'), "abedc");
}
