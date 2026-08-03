use std::println;

use parabolic_reflector_dish::*;
use utility::test_case_n;

test_case_n!(example,"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    part1: (part1::solve, 136),
    part2: (part2::solve, 64)
);

#[test]
fn test_slide() {
    let string = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let mut map = parse(string.to_string());
    println!("{}", _display(&map));
    slide_up(&mut map);
    slide_left(&mut map);
    slide_down(&mut map);
    slide_right(&mut map);
    println!("{}", _display(&map));
    slide_up(&mut map);
    slide_left(&mut map);
    slide_down(&mut map);
    slide_right(&mut map);
    println!("{}", _display(&map));
    slide_up(&mut map);
    slide_left(&mut map);
    slide_down(&mut map);
    slide_right(&mut map);
    println!("{}", _display(&map));
}

