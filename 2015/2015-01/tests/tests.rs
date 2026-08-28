use not_quite_lisp::*;

#[test]
fn example_part1() {
    assert_eq!(0, part1::solve("(())".to_string()));
    assert_eq!(0, part1::solve("()()".to_string()));
    assert_eq!(3, part1::solve("((( ".to_string()));
    assert_eq!(3, part1::solve("(()(()(".to_string()));
    assert_eq!(3, part1::solve("))(((((".to_string()));
    assert_eq!(-1, part1::solve("())".to_string()));
    assert_eq!(-1, part1::solve("))(".to_string()));
    assert_eq!(-3, part1::solve(")))".to_string()));
    assert_eq!(-3, part1::solve(")())())".to_string()));
}

#[test]
fn example_part2() {
    assert_eq!(1, part2::solve(")".to_string()));
    assert_eq!(5, part2::solve("()())".to_string()));
}
