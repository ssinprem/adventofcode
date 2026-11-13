use the_tyranny_of_the_rocket_equation::*;

#[test]
fn test_part1() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583);
}

#[test]
fn test_part2() {
    assert_eq!(part2::solve("12".to_string()), 2);
    assert_eq!(part2::solve("14".to_string()), 2);
    assert_eq!(part2::solve("1969".to_string()), 966);
    assert_eq!(part2::solve("100756".to_string()), 50346);
}
