use print_dept::part2::*;

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

    assert_eq!(solve(file), 43);
}