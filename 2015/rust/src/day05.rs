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

fn is_nice_part1(s: &str) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];
    let contains_three_vowels = s.chars().map(is_vowel).filter(|&b| b).count() >= 3;
    let contains_bad_string = bad_strings.iter().any(|bad_string| s.contains(bad_string));
    contains_three_vowels && contains_repetition(s) && !contains_bad_string
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let nice_strings_part1 = input.lines().map(is_nice_part1).filter(|&b| b).count();
    println!("[Part 1]: Number of nice strings: {}", nice_strings_part1);

    Ok(())
}
