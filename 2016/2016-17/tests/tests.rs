use two_steps_forward::*;
use utility::test_case_n;

test_case_n!(example0,
    "hijkl",
    part1: (part1::solve, None),
    part2: (part2::solve, None)
);
test_case_n!(example1,
    "ihgpwlah",
    part1: (part1::solve, Some("DDRRRD".to_string())),
    part2: (part2::solve, Some(370))
);
test_case_n!(example2,
    "kglvqrro",
    part1: (part1::solve, Some("DDUDRLRRUDRD".to_string())),
    part2: (part2::solve, Some(492))
);
test_case_n!(example3,
    "ulqzkmiv",
    part1: (part1::solve, Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string())),
    part2: (part2::solve, Some(830))
);

#[test]
fn _test() {
    println!("{}", format!("{:x}", md5::compute("hijkl")));
    println!("{}", format!("{:x}", md5::compute("hijklD")));
    println!("{}", format!("{:x}", md5::compute("hijklDR")));
    println!("{}", format!("{:x}", md5::compute("hijklDU")));
}
