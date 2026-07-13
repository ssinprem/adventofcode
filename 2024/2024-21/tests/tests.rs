use std::collections::HashMap;

use keypad_conundrum::*;
use utility::test_case_n;

test_case_n!(example,"
029A
980A
179A
456A
379A",
    part1: (part1::solve, 126384)
);

#[test]
fn test_function() {
    let mut gbest = HashMap::new();
    assert_eq!(get_best_length("029A".to_string(), 1, &mut gbest), 12);
    assert_eq!(get_best_length("029A".to_string(), 2, &mut gbest), 28);
    assert_eq!(get_best_length("029A".to_string(), 3, &mut gbest), 68);
    assert_eq!(get_best_length("980A".to_string(), 3, &mut gbest), 60);
    assert_eq!(get_best_length("179A".to_string(), 3, &mut gbest), 68);
    assert_eq!(get_best_length("456A".to_string(), 3, &mut gbest), 64);
    assert_eq!(get_best_length("379A".to_string(), 3, &mut gbest), 64);
}

#[test]
fn test_1_25(){
    let mut gbest = HashMap::new();
    assert_eq!(get_best_length(">".to_string(), 26, &mut gbest), 20790420654);
    assert_eq!(get_best_length("^".to_string(), 26, &mut gbest), 22411052532);
    assert_eq!(get_best_length("v".to_string(), 26, &mut gbest), 27622800565);
    assert_eq!(get_best_length("<".to_string(), 26, &mut gbest), 30331287706);
}