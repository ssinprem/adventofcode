use stream_processing::*;

#[test]
fn example() {
    let list = vec![
        (1, "{}".to_string()),
        (6, "{{{}}}".to_string()),
        (5, "{{},{}}".to_string()),
        (16, "{{{},{},{{}}}}".to_string()),
        (1, "{<a>,<a>,<a>,<a>}".to_string()),
        (9, "{{<ab>},{<ab>},{<ab>},{<ab>}}".to_string()),
        (9, "{{<!!>},{<!!>},{<!!>},{<!!>}}".to_string()),
        (3, "{{<a!>},{<a!>},{<a!>},{<ab>}}".to_string()),
    ];

    for (score, string) in list {
        assert_eq!(score, part1::solve(string));
    }
}

#[test]
fn example2() {
    let list = vec![
        (0, "<>".to_string()),
        (17, "<random characters>".to_string()),
        (3, "<<<<>".to_string()),
        (2, "<{!>}>".to_string()),
        (0, "<!!>".to_string()),
        (0, "<!!!>>".to_string()),
        (10, r#"<{o"i!a,<{i<a>"#.to_string()),
    ];

    for (score, string) in list {
        assert_eq!(score, part2::solve(string));
    }
}
