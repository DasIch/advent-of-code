fn increment_char(c: char) -> (bool, char) {
    if c == 'z' {
        (true, 'a')
    } else {
        (false, char::from_u32(c as u32 + 1).unwrap())
    }
}

fn increment_password(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().rev().collect();
    let mut carry = false;
    for c in chars.iter_mut() {
        (carry, *c) = increment_char(*c);
        if !carry {
            break;
        }
    }
    if carry {
        chars.push('a');
    }
    chars.iter().rev().collect()
}

fn includes_increasing_straight(s: &str) -> bool {
    use std::iter::zip;

    for ((a, b), c) in zip(zip(s.chars(), s.chars().skip(1)), s.chars().skip(2)) {
        let (carry_a, incremented_a) = increment_char(a);
        let (carry_b, incremented_b) = increment_char(b);
        if !carry_a && incremented_a == b && !carry_b && incremented_b == c {
            return true;
        }
    }
    false
}

fn contains_forbidden_chars(s: &str) -> bool {
    s.chars().any(|c| ['i', 'o', 'l'].contains(&c))
}

fn contains_two_nonoverlapping_pairs(s: &str) -> bool {
    use std::iter::zip;

    let mut locations = vec![];
    for (a, b) in zip(s.chars().enumerate(), s.chars().enumerate().skip(1)) {
        if a.1 == b.1 {
            locations.push((a.0, b.0));
        }
    }

    for (a, b) in zip(locations.iter(), locations.iter().skip(1)) {
        if a.1 == b.0 {
            return false;
        }
    }
    locations.len() >= 2
}

fn meets_security_requirements(password: &str) -> bool {
    includes_increasing_straight(password)
        && !contains_forbidden_chars(password)
        && contains_two_nonoverlapping_pairs(password)
}

fn next_password(password: &str) -> String {
    let mut password = password.to_string();
    loop {
        password = increment_password(password.as_str());
        if meets_security_requirements(password.as_str()) {
            break;
        }
    }
    password
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let next = next_password(input);
    let next2 = next_password(next.as_str());

    println!("Next password: {next}");
    println!("Next password: {next2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_char() {
        assert_eq!(increment_char('a'), (false, 'b'));
        assert_eq!(increment_char('z'), (true, 'a'));
    }

    #[test]
    fn test_increment_password() {
        assert_eq!(increment_password("a"), "b");
        assert_eq!(increment_password("z"), "aa");
        assert_eq!(increment_password("zz"), "aaa");
    }

    #[test]
    fn test_includes_increasing_straight() {
        assert!(includes_increasing_straight("abc"));
        assert!(includes_increasing_straight("bcd"));
        assert!(includes_increasing_straight("cde"));
        assert!(includes_increasing_straight("xyz"));
        assert!(!includes_increasing_straight("abd"));

        assert!(!includes_increasing_straight("yza"));

        assert!(includes_increasing_straight("hijklmmn"));
        assert!(includes_increasing_straight("abcdefgh"));
        assert!(includes_increasing_straight("abcdffaa"));
        assert!(includes_increasing_straight("ghijklmn"));
        assert!(includes_increasing_straight("ghjaabcc"));
    }

    #[test]
    fn test_contains_two_nonoverlapping_pairs() {
        assert!(!contains_two_nonoverlapping_pairs("hijklmmn"));
        assert!(contains_two_nonoverlapping_pairs("abbceffg"));
        assert!(!contains_two_nonoverlapping_pairs("abbcegjk"));
        assert!(contains_two_nonoverlapping_pairs("abcdffaa"));
        assert!(contains_two_nonoverlapping_pairs("ghjaabcc"));
    }

    #[test]
    fn test_meets_security_requirements() {
        assert!(!meets_security_requirements("hijklmmn"));
        assert!(!meets_security_requirements("abbceffg"));
        assert!(!meets_security_requirements("abbcegjk"));
        assert!(meets_security_requirements("abcdffaa"));
        assert!(meets_security_requirements("ghjaabcc"));
    }

    #[test]
    fn test_next_password() {
        assert_eq!(next_password("abcdefgh"), "abcdffaa");
        assert_eq!(next_password("ghijklmn"), "ghjaabcc");
    }
}
