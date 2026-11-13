use n1202_program_alarm::*;

#[test]
fn test_example() {
    let mut mem = parse("1,9,10,3,2,3,11,0,99,30,40,50".to_string());

    mem = process(&mem);
    assert_eq!(*mem.first().unwrap(), 3500);
}
