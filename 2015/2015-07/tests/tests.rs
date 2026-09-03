use some_assembly_required::*;

#[test]
fn example(){
  let circuit = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i".to_string();

    let hm = part1::solve(circuit);
    assert_eq!(hm.get(&"x".to_string()), Some(&123_u16));
    assert_eq!(hm.get(&"y".to_string()), Some(&456_u16));
    assert_eq!(hm.get(&"d".to_string()), Some(&72_u16));
    assert_eq!(hm.get(&"e".to_string()), Some(&507_u16));
    assert_eq!(hm.get(&"f".to_string()), Some(&492_u16));
    assert_eq!(hm.get(&"g".to_string()), Some(&114_u16));
    assert_eq!(hm.get(&"h".to_string()), Some(&65412_u16));
    assert_eq!(hm.get(&"i".to_string()), Some(&65079_u16));
}

