use secret_entrance::part2::*;

#[test]
fn basic() {
    let input = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
".to_string();
    let expect = 6;
    let output = solution1(input);
    assert_eq!(output, expect);
}

#[test]
fn at_crossing_zero_once() {
    let input = "
R60
L20
R20
L20
".to_string();
    let expect = 4;
    let output = solution1(input);
    assert_eq!(output, expect);
}

#[test]
fn over_round() {
    let input = "
R100
L100
R200
L200
".to_string();
    let expect = 6;
    let output = solution1(input);
    assert_eq!(output, expect);
}

#[test]
fn no_rotation() {
    let input = "".to_string();
    let expect = 0;
    let output = solution1(input);
    assert_eq!(output, expect);
}

#[test]
fn single_large_rotation() {
    let input = "R1000".to_string();
    let expect = 10;
    let output = solution1(input);
    assert_eq!(output, expect);
}

#[test]
fn double_large_rotation() {
    let input = "
R1000
L2000".to_string();
    let expect = 30;
    let output = solution1(input);
    assert_eq!(output, expect);
}

#[test]
fn boundary_extact_zero_landing() {
    let input = "
L50
R100
L100
R5
"
    .to_string();
    let expect = 3;
    let output = solution1(input);
    assert_eq!(output, expect);
}

#[test]
fn passing_thru_0_without_end_onit() {
    let input = "
L60
R20
R80
"
    .to_string();
    let expect = 2;
    let output = solution1(input);
    assert_eq!(output, expect);
}

#[test]
fn no_hits() {
    let input = "
R10
L20
R5
L3
"
    .to_string();
    let expect = 0;
    let output = solution1(input);
    assert_eq!(output, expect);
}

#[test]
fn back_to_start() {
    let input = "
R10
L20
R10
"
    .to_string();
    let expect = 0;
    let output = solution1(input);
    assert_eq!(output, expect);
}

#[test]
fn test_50_0_90() {
    let input = "
L150
L110
R120
L110
"
    .to_string();
    let expect = 7;
    let output = solution1(input);
    assert_eq!(output, expect);
}

