use std::collections::HashMap;
use leonardo_monorail::*;



#[test]
fn example() {
    let inst =
"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";
    let ans = vec![
        ("a".to_string(),42),
        ("b".to_string(),0),
        ("c".to_string(),0),
        ("d".to_string(),0)
    ].into_iter().collect::<HashMap<String, i32>>();
    assert_eq!(part1::solve(inst.to_string()), ans);
}
