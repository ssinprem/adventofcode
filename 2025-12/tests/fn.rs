use christmas_tree_farm::*;

#[test]
fn test_display_non_rotate () {
    let pattern = 
        vec![
            vec![true, false, false],
            vec![true, false, false],
            vec![true, true,  false],
        ];
    assert_eq!(display(&transform(&pattern, Transform::Non)) ,
"
#..
#..
##.
")
}

#[test]
fn test_display_rotate_left () {
    let pattern = 
        vec![
            vec![true, false, false],
            vec![true, false, false],
            vec![true, true,  false],
        ];
    assert_eq!(display(&transform(&pattern, Transform::RotLeft)), 
"
...
..#
###
")
}

#[test]
fn test_display_rotate_right () {
    let pattern = 
        vec![
            vec![true, false, false],
            vec![true, false, false],
            vec![true, true,  false],
        ];
    assert_eq!(display(&transform(&pattern, Transform::RotRight)), 
"
###
#..
...
")
}

#[test]
fn test_display_rotate_haft () {
    let pattern = 
        vec![
            vec![true, false, false],
            vec![true, false, false],
            vec![true, true,  false],
        ];
    assert_eq!(display(&transform(&pattern, Transform::RotHalf)), 
"
.##
..#
..#
")
}

#[test]
fn test_display_rotate_flip_h () {
    let pattern = 
        vec![
            vec![true, false, false],
            vec![true, false, false],
            vec![true, true,  false],
        ];
    assert_eq!(display(&transform(&pattern, Transform::FlipHor)), 
"
##.
#..
#..
")
}

#[test]
fn test_display_rotate_flip_v () {
    let pattern = 
        vec![
            vec![true, false, false],
            vec![true, false, false],
            vec![true, true,  false],
        ];
    assert_eq!(display(&transform(&pattern, Transform::FlipVir)), 
"
..#
..#
.##
")
}

#[test]
fn test_display_rotate_flip_i () {
    let pattern = 
        vec![
            vec![true, false, false],
            vec![true, false, false],
            vec![true, true,  false],
        ];
    assert_eq!(display(&transform(&pattern, Transform::FlipInc)), 
"
...
#..
###
")
}

#[test]
fn test_display_rotate_flip_d () {
    let pattern = 
        vec![
            vec![true, false, false],
            vec![true, false, false],
            vec![true, true,  false],
        ];
    assert_eq!(display(&transform(&pattern, Transform::FlipDec)), 
"
###
..#
...
")
}

#[test]
fn test_display_yard_empty() {
    let map = generate_yard(4, 6);
    assert_eq!(display(&map),
"
....
....
....
....
....
....
")
}

#[test]
fn test_display_yard_with_pattern() {
    let map = generate_yard(4, 6);
    assert_eq!(display(&map),
"
....
....
....
....
....
....
");
    let pattern = 
        vec![
            vec![true, true,  true],
            vec![true, false, true],
            vec![true, false, true],
        ];
    let result = put_yard(map.clone(), &pattern, 0, 0);
    assert!(result.is_ok());
    let new_map = result.unwrap();
    assert_eq!(display(&new_map),
"
###.
#.#.
#.#.
....
....
....
");
    let result = put_yard(new_map.clone(), &pattern, 1, 1);
    assert!(result.is_err());
    let result = put_yard(new_map.clone(), &transform(&pattern, Transform::RotLeft), 1, 1);
    assert!(result.is_err());
    let result = put_yard(new_map.clone(), &transform(&pattern, Transform::RotRight), 1, 1);
    assert!(result.is_err());
    let result = put_yard(new_map.clone(), &transform(&pattern, Transform::RotHalf), 1, 1);
    assert!(result.is_ok());
    let new_map = result.unwrap();
assert_eq!(display(&new_map),
"
###.
####
####
.###
....
....
");
}

#[test]
fn test_check_variat_full() {
    let pattern = vec![
        vec![true, false, false],
        vec![true, false, false],
        vec![false, false, false],
    ];
    assert_eq!(get_varints(&pattern).len(), 8)
}

#[test]
fn test_check_variat_half() {
    let pattern = vec![
        vec![true, false, false],
        vec![false, false, false],
        vec![false, false, false],
    ];
    assert_eq!(get_varints(&pattern).len(), 4)
}

#[test]
fn test_check_variat_quad() {
    let pattern = vec![
        vec![false, true, false],
        vec![false, true, false],
        vec![false, true, false],
    ];
    assert_eq!(get_varints(&pattern).len(), 2)
}

#[test]
fn test_check_variat_same() {
    let pattern = vec![
        vec![false, false, false],
        vec![false, true, false],
        vec![false, false, false],
    ];
    assert_eq!(get_varints(&pattern).len(), 1)
}
