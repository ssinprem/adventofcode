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


#[test]
fn convert () {
    const A : usize = 0;
    const B : usize = 1;
    const C : usize = 2;
    const D : usize = 3;
    let set_vals = [[0,0,0,0],[0,0,1,0]];

    for vals in set_vals {
        let mut vals = vals;
        vals[A] = 1;
        vals[B] = 1;
        vals[D] = 26;
        // 4 - 9
        if vals[C] != 0 {
            vals[C] = 7;
            vals[D] += vals[C];
            vals[C] = 0;
        }
        // 10 - 16
        while vals[D] != 0 {
            vals[C] = vals[A];
            vals[A] += vals[B];
            vals[B] = 0;
            vals[B] = vals[C];
            vals[D] -= 1;
        }
        // 17-23
        vals[C] = 19;
        vals[D] = 14;
        vals[A] += vals[D] * vals[C];

        println!("{:?}", vals)
    }
}