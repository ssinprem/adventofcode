pub mod part1 {
    use regex::Regex;
    pub fn solve(file: String) -> i64 {
        let regex = Regex::new(r"[-0-9]+").unwrap();

        let mut sum = 0;
        for cap in regex.captures_iter(&file) {
            if let Ok(n) = cap.get_match().as_str().parse::<i64>() {
                sum += n;
            }
        }

        sum
    }
}

pub mod part2 {
    use serde_json::Value;

    pub fn extract_sum(objects : Value) -> i64 {
        if objects.is_array() {
            objects.as_array().map(|item| {
                item.iter().map(|value| {
                    extract_sum(value.clone())
                }).sum()
            }).iter().sum()
        } else if objects.is_object() {
            if objects.as_object().into_iter().find(| obj | {
                obj.values().any(|value| value == &"red")
            }).is_some() {
                0
            } else {
                objects.as_object().into_iter().map(|obj| {
                    obj.values().map(|value| {
                        extract_sum(value.clone())
                    }).sum::<i64>()
                }).sum()
            }
        } else if objects.is_i64() {
            objects.as_i64().unwrap()
        } else {
            0
        }
    }

    pub fn solve(file: String) -> i64 {
        let objects: Value = serde_json::from_str(file.as_str()).expect("JSON cannot parse");
        extract_sum(objects)
    }
}
