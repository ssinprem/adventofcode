use spiral_memory::*;

#[test]
fn test_part1() {
    assert_eq!(part1::solve(1), 0);
    assert_eq!(part1::solve(12), 3);
    assert_eq!(part1::solve(23), 2);
    assert_eq!(part1::solve(1024), 31);
}
#[test]
fn test_part2() {
    assert_eq!(part2::solve(1), 1);
    assert_eq!(part2::solve(5), 5);
    assert_eq!(part2::solve(58), 59);
    assert_eq!(part2::solve(805), 806);
}