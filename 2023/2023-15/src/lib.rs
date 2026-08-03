pub fn calc(str: String) -> u64 {
    str.chars()
        .filter(|c| !c.is_whitespace())
        .fold(0_u64, |acc, char| {
            let value: u64 = acc + char as u64;
            (value * 17) % 256
        })
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        file.split(",")
            .map(|str| calc(str.to_string()))
            .sum::<u64>()
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let mut boxs: Vec<_> = (0..256).map(|_| Vec::<(String, u32)>::new()).collect();
        file.split(",").for_each(|str| {
            if str.ends_with("-") {
                let target_label = str.strip_suffix("-").unwrap();
                let hash = calc(target_label.to_string()) as u8;

                let boxx = boxs.get_mut(hash as usize).unwrap();
                if let Some(index) = boxx.iter().position(|(label, _len)| label == target_label) {
                    boxx.remove(index);
                }
            } else if str.contains("=") {
                let (target_label, len) = str.split_once("=").unwrap();
                let hash = calc(target_label.to_string()) as u8;
                let num_len = len.parse::<u32>().unwrap();
                let boxx = boxs.get_mut(hash as usize).unwrap();
                if let Some(index) = boxx.iter().position(|(label, _len)| label == target_label) {
                    let slot = boxx.get_mut(index).unwrap();
                    slot.1 = num_len;
                } else {
                    boxx.push((target_label.to_string(), num_len));
                }
            }
        });

        boxs.iter()
            .enumerate()
            .map(|(nbox, abox)| {
                abox.iter()
                    .enumerate()
                    .map(|(nslot, slot)| (nbox + 1) * (nslot + 1) * slot.1 as usize)
                    .sum::<usize>()
            })
            .sum::<usize>() as u64
    }
}
