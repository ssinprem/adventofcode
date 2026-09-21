use dragon_checksum::*;
use utility::test_case_n;

test_case_n!(example1,
"110010110100",
    part1: (part1::solve, "100".to_string(), 12)
);
test_case_n!(example2,
"10000",
    part1: (part1::solve, "01100".to_string(), 20)
);


#[test]
fn test_step() {
    assert_eq!(step1("1".to_string()), "100".to_string());
    assert_eq!(step1("0".to_string()), "001".to_string());
    assert_eq!(step1("11111".to_string()), "11111000000".to_string());
    assert_eq!(step1("111100001010".to_string()), "1111000010100101011110000".to_string());

    assert_eq!(step2("110010110100".to_string()), "100".to_string());
}