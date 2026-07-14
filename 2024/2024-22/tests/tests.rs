use monkey_market::{part2::{get_changes, get_sell_price}, *};
use utility::test_case_n;

test_case_n!(example,"
1
10
100
2024",
    part1: (part1::solve, 37327623)
);

#[test]
fn test_function() {
    assert_eq!(prune(100000000), 16113920);
    let temp = process(123);
    assert_eq!(temp, 15887950);
    let temp = process(temp);
    assert_eq!(temp, 16495136);
    let temp = process(temp);
    assert_eq!(temp, 527345);
    let temp = process(temp);
    assert_eq!(temp, 704524);
    let temp = process(temp);
    assert_eq!(temp, 1553684);
    let temp = process(temp);
    assert_eq!(temp, 12683156);
    let temp = process(temp);
    assert_eq!(temp, 11100544);
    let temp = process(temp);
    assert_eq!(temp, 12249484);
    let temp = process(temp);
    assert_eq!(temp, 7753432);
    let temp = process(temp);
    assert_eq!(temp, 5908254);

    assert_eq!((0..2000).fold(1,   |init,_| process(init) ), 8685429);
    assert_eq!((0..2000).fold(10,  |init,_| process(init) ), 4700978);
    assert_eq!((0..2000).fold(100, |init,_| process(init) ), 15273692);
    assert_eq!((0..2000).fold(2024,|init,_| process(init) ), 8667524);
}

#[test]
fn short_part2() {
    let price = get_sell_price(123);
    let change = get_changes(&price);
    assert_eq!(part2::get_amount(&price,&change, &[-1,-1,0,2]), 6);
    let price = get_sell_price(1);
    let change = get_changes(&price);
    assert_eq!(part2::get_amount(&price,&change, &[-2,1,-1,3]), 7);
    let price = get_sell_price(2);
    let change = get_changes(&price);
    assert_eq!(part2::get_amount(&price,&change, &[-2,1,-1,3]), 7);
    let price = get_sell_price(3);
    let change = get_changes(&price);
    assert_eq!(part2::get_amount(&price,&change, &[-2,1,-1,3]), 0);
    let price = get_sell_price(2024);
    let change = get_changes(&price);
    assert_eq!(part2::get_amount(&price,&change, &[-2,1,-1,3]), 9);
}

test_case_n!(example,"
1
2
3
2024",
    part2: (part2::solve, 23)
);