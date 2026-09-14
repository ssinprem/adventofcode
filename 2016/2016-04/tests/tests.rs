use security_through_obscurity::*;
use utility::test_case_n;

test_case_n!(example,
"aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]",
    part1: (part1::solve, 1514)
);

#[test]
fn encypt() {
    let name = "qzmt-zixmtkozy-ivhz-343[dummy]";
    let room = parse(name.to_string());
    println!("{room:?}");
    let room = room[0].clone();
    assert_eq!(part2::encypt(&room), "very encrypted name");
}