use keypad_conundrum::*;
use utility::test_case_n;

test_case_n!(example,"
029A
980A
179A
456A
379A",
    part1: (part1::solve, 126384),
    part2: (part2::solve, 0)
);

#[test]
fn test_function() {
    assert_eq!(get_best_length("029A".to_string(), 1), 12);
    assert_eq!(get_best_length("029A".to_string(), 2), 28);
    assert_eq!(get_best_length("029A".to_string(), 3), 68);
    assert_eq!(get_best_length("980A".to_string(), 3), 60);
    assert_eq!(get_best_length("179A".to_string(), 3), 68);
    assert_eq!(get_best_length("456A".to_string(), 3), 64);
    assert_eq!(get_best_length("379A".to_string(), 3), 64);
}
