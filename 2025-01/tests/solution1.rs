use secret_entrance::part1::*;

#[test]
fn basic() {
    let input = 
"L68
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
    let expect = 3;
    let output = solution1(input.clone());
    assert_eq!(output, expect);

    let output = solution2(input.clone());
    assert_eq!(output, expect);

    let output = solution3(input.clone());
    assert_eq!(output, expect);

    let output = solution4(input.clone());
    assert_eq!(output, expect);
}

#[test]
fn double_large_rotation() {
    let input = "
R1000
L2000".to_string();
    let expect = 0;
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
    let expect = 0;
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