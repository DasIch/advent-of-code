use json::JsonValue;

fn part1(document: &str) -> f64 {
    fn sum(value: &json::JsonValue) -> f64 {
        match value {
            JsonValue::Number(_) => value.as_f64().unwrap(),
            JsonValue::Object(_) => value.entries().map(|(_, v)| sum(v)).sum(),
            JsonValue::Array(values) => values.iter().map(sum).sum(),
            _ => 0.0,
        }
    }
    sum(&json::parse(document).unwrap())
}

fn part2(document: &str) -> f64 {
    fn sum(value: &json::JsonValue) -> f64 {
        match value {
            JsonValue::Number(_) => value.as_f64().unwrap(),
            JsonValue::Object(_) => {
                let has_red = value.entries().any(|(_, v)| v == "red");
                if has_red {
                    0.0
                } else {
                    value.entries().map(|(_, v)| sum(v)).sum()
                }
            }
            JsonValue::Array(values) => values.iter().map(sum).sum(),
            _ => 0.0,
        }
    }
    sum(&json::parse(document).unwrap())
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let sum_of_all_numbers = part1(input);
    println!("Sum of all numbers: {sum_of_all_numbers}");

    let sum_of_all_numbers_wo_red = part2(input);
    println!("Sum of all numbers without red: {sum_of_all_numbers_wo_red}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2("[1,2,3]"), 6.0);
        assert_eq!(part2("[1,{\"c\":\"red\",\"b\":2},3]"), 4.0);
    }
}
