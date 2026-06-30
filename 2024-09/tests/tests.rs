use disk_fragmenter::*;
use utility::test_case_n;

test_case_n!(example,
"2333133121414131402",
    part1: (part1::solve, 1928),
    part2: (part2::solve, 2858)
);

#[test]
fn test_checksum() {
    let disks = [
        ("0099811188827773336446555566..............", 1928),
        ("00992111777.44.333....5555.6666.....8888..", 2858),
    ];
    for (disk, chksm) in disks {
        let vec_disk = disk
            .chars()
            .map(|c| match c {
                '.' => Space::Free,
                c => Space::Used(c.to_string().parse::<usize>().expect("cannot parse")),
            })
            .collect::<Vec<Space>>();
        assert_eq!(checksum(vec_disk), chksm);
    }
}
