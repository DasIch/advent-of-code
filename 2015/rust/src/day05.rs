fn is_vowel(c: char) -> bool {
    "aeiou".contains(c)
}

fn contains_repetition(s: &str) -> bool {
    let mut chars = s.chars();
    let mut current = chars.next().unwrap();
    for c in chars {
        if c == current {
            return true;
        }
        current = c;
    }
    false
}

fn contains_nonoverlapping_pair_twice(s: &str) -> bool {
    use std::collections::HashMap;
    use std::iter::zip;

    let mut pairs: HashMap<(char, char), Vec<(usize, usize)>> = HashMap::new();

    for ((i, a), (j, b)) in zip(s.chars().enumerate(), s.chars().enumerate().skip(1)) {
        let pair = (a, b);
        let location = (i, j);
        let locations = pairs.entry(pair).or_insert(vec![]);
        for previous_location in locations.iter() {
            if previous_location.1 != location.0 {
                return true;
            }
        }
        locations.push(location);
    }
    false
}

fn contains_sandwiched_character(s: &str) -> bool {
    use std::iter::zip;

    for ((a, _), c) in zip(zip(s.chars(), s.chars().skip(1)), s.chars().skip(2)) {
        if a == c {
            return true;
        }
    }
    false
}

fn is_nice_part1(s: &str) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];
    let contains_three_vowels = s.chars().map(is_vowel).filter(|&b| b).count() >= 3;
    let contains_bad_string = bad_strings.iter().any(|bad_string| s.contains(bad_string));
    contains_three_vowels && contains_repetition(s) && !contains_bad_string
}

fn is_nice_part2(s: &str) -> bool {
    contains_nonoverlapping_pair_twice(s) &&
    contains_sandwiched_character(s)
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let nice_strings_part1 = input.lines().map(is_nice_part1).filter(|&b| b).count();
    let nice_strings_part2 = input.lines().map(is_nice_part2).filter(|&b| b).count();
    println!("[Part 1]: Number of nice strings: {}", nice_strings_part1);
    println!("[Part 2]: Number of nice strings: {}", nice_strings_part2);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_nonoverlapping_pair_twice() {
        assert!(contains_nonoverlapping_pair_twice("xyxy"));
        assert!(contains_nonoverlapping_pair_twice("aabcdefgaa"));
        assert!(!contains_nonoverlapping_pair_twice("aaa"));
    }

    #[test]
    fn test_contains_sandwiched_character() {
        assert!(contains_sandwiched_character("xyx"));
        assert!(contains_sandwiched_character("abcdefeghi"));
        assert!(contains_sandwiched_character("aaa"));
        assert!(!contains_sandwiched_character("abc"));
    }
}
