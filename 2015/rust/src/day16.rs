use std::collections::HashMap;

fn parse_description(line: &str) -> HashMap<String, i32> {
    let (_, properties) = line.split_once(": ").unwrap();
    properties
        .split(", ")
        .map(|property| property.split_once(": ").unwrap())
        .map(|(k, v)| (k.to_string(), v.parse().unwrap()))
        .collect()
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let known_properties = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
    let sues: Vec<HashMap<String, i32>> = input.lines().map(parse_description).collect();

    for (i, sue) in sues.iter().enumerate() {
        let is_part1_gifter = sue
            .iter()
            .all(|(key, value)| *known_properties.get(key.as_str()).unwrap() == *value);

        if is_part1_gifter {
            println!("[Part 1]: Sue {} sent the gift", i + 1);
            continue;
        }

        let is_part2_gifter = sue.iter().all(|(key, value)| {
            let known_value = known_properties.get(key.as_str()).unwrap();
            match key.as_str() {
                "cats" => value >= known_value,
                "trees" => value >= known_value,
                "pomeranians" => value <= known_value,
                "goldfish" => value <= known_value,
                _ => value == known_value,
            }
        });

        if is_part2_gifter {
            println!("[Part 2]: Sue {} sent the gift", i + 1);
        }
    }

    Ok(())
}
