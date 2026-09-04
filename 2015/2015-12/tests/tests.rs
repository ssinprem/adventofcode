use jsabacusframwork_io::*;

#[test]
fn example() {
    assert_eq!(6, part1::solve("[1,2,3]".to_string()));
    assert_eq!(6, part1::solve("{\"a\":2,\"b\":4} ".to_string()));
    assert_eq!(3, part1::solve("[[[3]]]".to_string()));
    assert_eq!(3, part1::solve("{\"a\":{\"b\":4},\"c\":-1}".to_string()));
    assert_eq!(0, part1::solve("{\"a\":[-1,1]}".to_string()));
    assert_eq!(0, part1::solve("[-1,{\"a\":1}]".to_string()));
    assert_eq!(0, part1::solve("[]".to_string()));
    assert_eq!(0, part1::solve("{}".to_string()));
}
