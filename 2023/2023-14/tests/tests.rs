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
    // cycle 1
    slide_up(&mut map);
    slide_left(&mut map);
    slide_down(&mut map);
    slide_right(&mut map);
    assert_eq!(
        _display(&map),
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"
    );
    // cycle 2
    slide_up(&mut map);
    slide_left(&mut map);
    slide_down(&mut map);
    slide_right(&mut map);
    assert_eq!(
        _display(&map),
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
"
    );
    // cycle 3
    slide_up(&mut map);
    slide_left(&mut map);
    slide_down(&mut map);
    slide_right(&mut map);
    assert_eq!(
        _display(&map),
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
"
    );
}
