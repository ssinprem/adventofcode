use chronal_charge::*;
use utility::test_case_n;

test_case_n!(example1, "18", part1: (part1::solve, (29, (33,45))));
test_case_n!(example2, "42", part1: (part1::solve, (30, (21,61))));

#[test]
fn test_fuel_cell_power_level() {
    assert_eq!(fuel_power(122, 79, 57), -5);
    assert_eq!(fuel_power(217,196, 39),  0);
    assert_eq!(fuel_power(101,153, 71),  4);
}