use regex::Regex;

#[allow(clippy::type_complexity)]
pub fn parse(file: String) -> (Vec<u64>,Vec<(String, Vec<Vec<u64>>)>) {

    let seeds = file.lines().find(|line| line.starts_with("seeds:")).unwrap()
    .strip_prefix("seeds: ").unwrap()
    .split_whitespace().map(|str| str.parse().unwrap()).collect();
    let regex = Regex::new(r"([a-z\-]+) map:\n([0-9 \n]+)").unwrap();
    // let mut maps = HashMap::<String, Vec<[u64; 3]>>::new();

    let maps = regex.captures_iter(&file).map(|caps| {
        let name = caps.get(1).unwrap().as_str().to_string();
        let lists = caps.get(2).unwrap().as_str().lines().filter_map(|line| {
            if !line.is_empty() {
                Some(
                    line.split_whitespace()
                    .map(|str| str.parse().unwrap()).collect()
                )
            } else {
                None
            }
        }).collect();
        (name, lists)
    }).collect();
    (seeds, maps)
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let (seeds,type_maps) = parse(file);
        println!("{seeds:?}");
        const MAX: u64 = u64::MAX;
        let mut slots: Vec<_> = seeds.iter().map(|seed| {
            [*seed, MAX, MAX, MAX, MAX, MAX, MAX, MAX]
        }).collect();

        for (id, (_str,maps)) in type_maps.iter().enumerate(){
            for slot in slots.iter_mut() {
                let last = slot[id];
                let mut found = false;
                for map in maps {
                    let &dst = map.first().unwrap();
                    let &src = map.get(1).unwrap();
                    let &len = map.get(2).unwrap();
                    
                    if (src..(src+len)).contains(&last) {
                        let i = last-src;
                        slot[id+1] = dst+i;
                        found = true;
                        break;
                    }
                }
                if !found {
                    slot[id+1] = last;
                }
            }
        }
        slots.iter().map(|slot| slot[7]).min().unwrap()
    }
}

pub mod part2 {
    use super::*;

    // Helper function to map ranges through one layer of maps
    fn map_ranges(ranges: &[(u64, u64)], maps: &[Vec<u64>]) -> Vec<(u64, u64)> {
        let mut mapped_ranges: Vec<(u64, u64)> = Vec::new();
        let mut unmapped_ranges: Vec<(u64, u64)> = ranges.to_vec();

        for map_entry in maps {
            let dest_start = map_entry[0];
            let source_start = map_entry[1];
            let length = map_entry[2];
            let source_end = source_start + length;

            let mut next_unmapped_ranges: Vec<(u64, u64)> = Vec::new();

            for (start, len) in unmapped_ranges {
                let end = start + len;

                // Part of the range before the entry's source range
                let before_start = start;
                let before_end = std::cmp::min(end, source_start);
                if before_start < before_end {
                    next_unmapped_ranges.push((before_start, before_end - before_start));
                }

                // Part of the range overlapping with the entry's source range
                let overlap_start = std::cmp::max(start, source_start);
                let overlap_end = std::cmp::min(end, source_end);
                if overlap_start < overlap_end {
                    let mapped_start = dest_start + (overlap_start - source_start);
                    mapped_ranges.push((mapped_start, overlap_end - overlap_start));
                }

                // Part of the range after the entry's source range
                let after_start = std::cmp::max(start, source_end);
                let after_end = end;
                if after_start < after_end {
                    next_unmapped_ranges.push((after_start, after_end - after_start));
                }
            }
            unmapped_ranges = next_unmapped_ranges;
        }

        mapped_ranges.extend(unmapped_ranges);
        mapped_ranges
    }

    pub fn solve(file: String) -> u64 {
        let (old_seeds, type_maps) = parse(file);

        let mut ranges: Vec<(u64, u64)> = old_seeds
            .chunks_exact(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        for (_str, maps) in type_maps {
            ranges = map_ranges(&ranges, &maps);
        }

        ranges.iter().map(|(start, _len)| *start).min().unwrap_or(0)
    }
}
