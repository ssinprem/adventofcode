use permutaion_promenade::*;
use utility::test_case_n;

test_case_n!(example,
"s1,x3/4,pe/b",
    part1: (part1::solve, "baedc", 5),
    part2: (part2::solve, "abcde", 5)
);

test_case_n!(example2,
"x13/12,pb/n,s10,x5/3,pl/g,x15/1,s2,x10/3,x9/12",
    part1: (part1::solve, "ehloigkjbafpmncd", 16),
    part2: (part2::solve, "jncdafgbehklmiop", 16)
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
