use fractal_art::*;
use std::error::Error;
use std::fs::read_to_string;

#[test]
fn example() -> Result<(), Box<dyn Error>> {
    let file = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#".to_string();
    let initial = "###..#.#.".to_string();
    assert_eq!(part1::solve(file.to_string(), initial.to_string(), 0), 5);
    assert_eq!(part1::solve(file.to_string(), initial.to_string(), 1), 4);
    assert_eq!(part1::solve(file.to_string(), initial.to_string(), 2), 12);
    Ok(())
}

#[test]
fn transform() {
    let string = "##..#.###".to_string();
    assert_eq!(flip_x(&string), ".##.#.###".to_string());
    assert_eq!(flip_y(&string), "###.#.##.".to_string());
    assert_eq!(rotate_l(&string), "#.#####..".to_string());
    assert_eq!(rotate_r(&string), "..#####.#".to_string());
    assert_eq!(rotate_h(&string), "###.#..##".to_string());
}

#[test]
fn first() -> Result<(), Box<dyn Error>> {
    let file = read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))?;
    let rule = parse(file);

    let mut vrule = rule.iter().collect::<Vec<_>>();

    vrule.sort_by_key(|(k,v)|
        format!("{} {:03} {}", k.len(), k.chars().filter(|c| c==&'#' ).count(),v.to_string())
    );
    for r in vrule {
        println!("{} => {}", r.0, r.1);
    }


    let string = ".#...####".to_string();
    let a = string.to_string();
    println!("{}   => {:?}", a.to_string(), rule.get(&a.to_string()));
    let a = flip_x(&string);
    println!("{}   => {:?}", a.to_string(), rule.get(&a.to_string()));
    let a = flip_y(&string);
    println!("{}   => {:?}", a.to_string(), rule.get(&a.to_string()));
    let a = flip_z(&string);
    println!("{}   => {:?}", a.to_string(), rule.get(&a.to_string()));
    let a = rotate_l(&string);
    println!("{}   => {:?}", a.to_string(), rule.get(&a.to_string()));
    let a = rotate_r(&string);
    println!("{}   => {:?}", a.to_string(), rule.get(&a.to_string()));
    let a = rotate_h(&string);
    println!("{}   => {:?}", a.to_string(), rule.get(&a.to_string()));

    Ok(())
}

#[test]
fn test_split() {
    
    println!("{:?}", string_split(
        &(0..4).map(|i| ('a' as u8 + i) as char).collect::<String>().to_string()
    ));
    println!("{:?}", string_split(
        &(0..9).map(|i| ('a' as u8 + i) as char).collect::<String>().to_string()
    ));
    println!("{:?}", string_split(
        &(0..16).map(|i| ('a' as u8 + i) as char).collect::<String>().to_string()
    ));
    println!("{:?}", string_split(
        &(0..26).map(|i| ('a' as u8 + i) as char)
        .chain((0..10).map(|i| ('A' as u8 + i) as char))
        .collect::<String>().to_string()
    ));
    
    let vec = vec!["abgh", "cdij", "efkl", "mnst", "opuv", "qrwx", "yzEF", "ABGH", "CDIJ"];
    let vec = vec.iter().map(|str| str.to_string()).collect::<Vec<_>>();
    println!("{}", string_join(vec));
}