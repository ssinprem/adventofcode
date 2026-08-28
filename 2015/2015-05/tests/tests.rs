use doesnt_he_have_intern_elves_for_this::*;
use utility::test_case_n;

#[test]
fn example_part1(){
    assert_eq!(true, part1::is_valid("ugknbfddgicrmopn".to_string()));
    assert_eq!(true, part1::is_valid("aaa".to_string()));
    assert_eq!(false, part1::is_valid("jchzalrnumimnmhp".to_string()));
    assert_eq!(false, part1::is_valid("haegwjzuvuyypxyu".to_string()));
    assert_eq!(false, part1::is_valid("dvszwmarrgswjxmb".to_string()));
}