use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Preference {
    person: String,
    happiness_units: i64,
    neighbor: String,
}

fn parse_preference(line: &str) -> Preference {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\.$"
        )
        .unwrap();
    }
    let captures = RE.captures(line).unwrap();

    let person = captures.get(1).unwrap().as_str().to_string();
    let sign = captures.get(2).unwrap().as_str();
    let happiness_units: i64 = captures.get(3).unwrap().as_str().parse().unwrap();
    let neighbor = captures.get(4).unwrap().as_str().to_string();

    let happiness_units = match sign {
        "gain" => happiness_units,
        "lose" => -happiness_units,
        _ => panic!("incorrect sign: {}", sign),
    };
    Preference {
        person,
        happiness_units,
        neighbor,
    }
}

fn evaluate_happiness(
    arrangement: &[&String],
    preferences: &HashMap<String, HashMap<String, i64>>,
) -> i64 {
    let mut total_happiness = 0;
    for (i, person) in arrangement.iter().enumerate() {
        let personal_preferences = preferences.get(person.as_str()).unwrap();
        let left_neighbor = {
            if i == 0 {
                arrangement.last().unwrap()
            } else {
                arrangement.get(i - 1).unwrap()
            }
        }
        .as_str();
        let right_neighbor = {
            if i == arrangement.len() - 1 {
                arrangement.first().unwrap()
            } else {
                arrangement.get(i + 1).unwrap()
            }
        }
        .as_str();

        if let Some(happiness) = personal_preferences.get(left_neighbor) {
            total_happiness += happiness;
        }
        if let Some(happiness) = personal_preferences.get(right_neighbor) {
            total_happiness += happiness;
        }
    }
    total_happiness
}

fn find_optimal_arrangement_happiness(preferences: &[Preference]) -> i64 {
    let mut preferences_by_person = HashMap::new();
    for preference in preferences {
        let entry = preferences_by_person
            .entry(preference.person.clone())
            .or_insert_with(HashMap::new);
        entry.insert(preference.neighbor.clone(), preference.happiness_units);
    }

    let people: Vec<String> = preferences_by_person.keys().cloned().collect();
    people
        .iter()
        .permutations(people.len())
        .map(|arrangement| evaluate_happiness(&arrangement[..], &preferences_by_person))
        .max()
        .unwrap()
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let mut preferences: Vec<Preference> = input.lines().map(parse_preference).collect();

    let optimal_total_change_in_happiness = find_optimal_arrangement_happiness(&preferences);
    println!("Optimal total change in happiness: {optimal_total_change_in_happiness}");

    preferences.push(Preference {
        person: "Me".to_string(),
        happiness_units: 0,
        neighbor: "Anyone".to_string(),
    });
    let optimal_total_change_in_happiness = find_optimal_arrangement_happiness(&preferences);
    println!("Optimal total change in happiness with me: {optimal_total_change_in_happiness}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reference() {
        assert_eq!(
            parse_preference("Alice would gain 54 happiness units by sitting next to Bob."),
            Preference {
                person: "Alice".to_string(),
                happiness_units: 54,
                neighbor: "Bob".to_string()
            }
        );
        assert_eq!(
            parse_preference("Alice would lose 79 happiness units by sitting next to Carol."),
            Preference {
                person: "Alice".to_string(),
                happiness_units: -79,
                neighbor: "Carol".to_string(),
            }
        );
    }
}
