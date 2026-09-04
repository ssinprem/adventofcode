use jsabacusframwork_io::*;

#[test]
fn example_part1() {
    assert_eq!(6, part1::solve(r#"[1,2,3]"#.to_string()));
    assert_eq!(6, part1::solve(r#"{"a":2,"b":4} "#.to_string()));
    assert_eq!(3, part1::solve(r#"[[[3]]]"#.to_string()));
    assert_eq!(3, part1::solve(r#"{"a":{"b":4},"c":-1}"#.to_string()));
    assert_eq!(0, part1::solve(r#"{"a":[-1,1]}"#.to_string()));
    assert_eq!(0, part1::solve(r#"[-1,{"a":1}]"#.to_string()));
    assert_eq!(0, part1::solve(r#"[]"#.to_string()));
    assert_eq!(0, part1::solve(r#"{}"#.to_string()));
}

#[test]
fn example_part2() {
    assert_eq!(6, part2::solve("[1,2,3]".to_string()));
    assert_eq!(4, part2::solve(r#"[1,{"c":"red","b":2},3]"#.to_string()));
    assert_eq!(0, part2::solve(r#"{"d":"red","e":[1,2,3,4],"f":5}"#.to_string()));
    assert_eq!(6, part2::solve(r#"[1,"red",5]"#.to_string()));
    
}
