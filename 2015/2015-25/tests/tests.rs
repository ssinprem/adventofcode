use let_it_snow::*;

#[test]
fn pos_order() {
    assert_eq!(0, position_order(1, 1));
    assert_eq!(1, position_order(2, 1));
    assert_eq!(2, position_order(1, 2));
    assert_eq!(3, position_order(3, 1));
    assert_eq!(4, position_order(2, 2));
    assert_eq!(5, position_order(1, 3));
    assert_eq!(6, position_order(4, 1));
    assert_eq!(7, position_order(3, 2));
    assert_eq!(8, position_order(2, 3));
    assert_eq!(9, position_order(1, 4));
}

#[test]
fn example() {
    assert_eq!(part1::solve(1, 1), 20151125);
    assert_eq!(part1::solve(1, 2), 18749137);
    assert_eq!(part1::solve(1, 3), 17289845);
    assert_eq!(part1::solve(1, 4), 30943339);
    assert_eq!(part1::solve(1, 5), 10071777);
    assert_eq!(part1::solve(1, 6), 33511524);
    assert_eq!(part1::solve(2, 1), 31916031);
    assert_eq!(part1::solve(2, 2), 21629792);
    assert_eq!(part1::solve(2, 3), 16929656);
    assert_eq!(part1::solve(2, 4), 7726640);
    assert_eq!(part1::solve(2, 5), 15514188);
    assert_eq!(part1::solve(2, 6), 4041754);
    assert_eq!(part1::solve(3, 1), 16080970);
    assert_eq!(part1::solve(3, 2), 8057251);
    assert_eq!(part1::solve(3, 3), 1601130);
    assert_eq!(part1::solve(3, 4), 7981243);
    assert_eq!(part1::solve(3, 5), 11661866);
    assert_eq!(part1::solve(3, 6), 16474243);
    assert_eq!(part1::solve(4, 1), 24592653);
    assert_eq!(part1::solve(4, 2), 32451966);
    assert_eq!(part1::solve(4, 3), 21345942);
    assert_eq!(part1::solve(4, 4), 9380097);
    assert_eq!(part1::solve(4, 5), 10600672);
    assert_eq!(part1::solve(4, 6), 31527494);
    assert_eq!(part1::solve(5, 1), 77061);
    assert_eq!(part1::solve(5, 2), 17552253);
    assert_eq!(part1::solve(5, 3), 28094349);
    assert_eq!(part1::solve(5, 4), 6899651);
    assert_eq!(part1::solve(5, 5), 9250759);
    assert_eq!(part1::solve(5, 6), 31663883);
    assert_eq!(part1::solve(6, 1), 33071741);
    assert_eq!(part1::solve(6, 2), 6796745);
    assert_eq!(part1::solve(6, 3), 25397450);
    assert_eq!(part1::solve(6, 4), 24659492);
    assert_eq!(part1::solve(6, 5), 1534922);
    assert_eq!(part1::solve(6, 6), 27995004);
}
