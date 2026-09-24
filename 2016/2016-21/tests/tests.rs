use scrambles_letters_and_hash::*;
use utility::test_case_n;

test_case_n!(example,
"swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d",
    part1: (part1::solve, "decab".to_string(), "abcde".to_string()),
    part2: (part2::solve, 0)
);

#[test]
fn test_part1() {
    let mut string = "abcde".to_string();
    let steps = vec![
        ("swap position 4 with position 0","ebcda"),
        ("swap letter d with letter b","edcba"),
        ("reverse positions 0 through 4","abcde"),
        ("rotate left 1 step","bcdea"),
        ("move position 1 to position 4","bdeac"),
        ("move position 3 to position 0","abdec"),
        ("rotate based on position of letter b","ecabd"),
        ("rotate based on position of letter d","decab"),
    ];

    for (step,result) in steps {
        string = part1::solve(step.to_string(), string);
        println!("{step:40} ->   {}  {string} <--> {result} ",
            if string == result { "✅" } else { "❌" }
        );
        assert_eq!(string, result);
    }
}