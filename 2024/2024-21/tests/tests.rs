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
fn test_get_sequence_by_botton() {

    assert_eq!(
        get_seq_by_btn('9', '2'),
        "vv<A".to_string()
    );
    assert_eq!(
        get_seq_by_btn('A', '7'),
        "^^^<<A".to_string()
    );
    assert_eq!(
        get_seq_by_btn('1', 'A'),
        ">>vA".to_string()
    );

    assert_eq!(
        get_seq_by_btn('<', 'A'),
        ">>^A".to_string()
    );
    assert_eq!(
        get_seq_by_btn('^', 'v'),
        "vA".to_string()
    );
    assert_eq!(
        get_seq_by_btn('^', '>'),
        "v>A".to_string()
    );
}

#[test]
fn test_get_sequence_by_set() {
    assert_eq!(
        get_seq_by_set("09".to_string()),
        "<A^^^>A".to_string()
    );
    assert_eq!(
        get_seq_by_set("029A".to_string()),
        "<A^A^^>AvvvA".to_string()
    );
    assert_eq!(
        get_seq_by_set("<A^A>^^AvvvA".to_string()),
        "v<<A>>^A<A>AvA^<AA>Av<AAA^>A".to_string()
    );
}