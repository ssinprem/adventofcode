use print_dept::part1::*;

#[test]
fn example() {

    let file = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.".to_string();

    assert_eq!(solve(file), 13);
}