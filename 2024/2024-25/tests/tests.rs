use code_chronicle::*;
use utility::test_case_n;

test_case_n!(example,
"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",
    part1: (part1::solve, 3),
    part2: (part2::solve, 0)
);

#[test]
fn test_parse_locks() {
    let input = "#####
##.##
.#.##
...##
...#.
...#.
.....";

    let (keys, locks) = parse(input.to_string());

    assert_eq!(keys.len(), 0);
    assert_eq!(locks.len(), 1);
    assert_eq!(locks[0], [1, 2, 0, 5, 3]);
}

#[test]
fn test_parse_keys() {
    let input = ".....
.....
#.#..
###..
###.#
###.#
#####";

    let (keys, locks) = parse(input.to_string());

    assert_eq!(keys.len(), 1);
    assert_eq!(locks.len(), 0);
    assert_eq!(keys[0], [4, 3, 4, 0, 2]);
}
