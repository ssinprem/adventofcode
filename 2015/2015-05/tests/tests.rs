use doesnt_he_have_intern_elves_for_this::*;

#[test]
fn example_part1(){
    assert_eq!(true, part1::is_valid("ugknbfddgicrmopn".to_string()));
    assert_eq!(true, part1::is_valid("aaa".to_string()));
    assert_eq!(false, part1::is_valid("jchzalrnumimnmhp".to_string()));
    assert_eq!(false, part1::is_valid("haegwjzuvuyypxyu".to_string()));
    assert_eq!(false, part1::is_valid("dvszwmarrgswjxmb".to_string()));
}

#[test]
fn example_part2(){
    assert_eq!(true, part2::is_valid("qjhvhtzxzqqjkmpb".to_string()));
    assert_eq!(true, part2::is_valid("xxyxx".to_string()));
    assert_eq!(true, part2::is_valid("xyxy".to_string()));
    assert_eq!(true, part2::is_valid("aaaa".to_string()));
    assert_eq!(false, part2::is_valid("aaa".to_string()));
    assert_eq!(false, part2::is_valid("uurcxstgmygtbstg".to_string()));
    assert_eq!(false, part2::is_valid("ieodomkazucvgmuy".to_string()));
}
