fn look_and_say(s: &str) -> String {
    let mut chars = s.chars();
    let mut current_char = chars.next().unwrap();
    let mut current_count = 1;
    let mut result = String::new();
    for c in chars {
        if c == current_char {
            current_count += 1;
        } else {
            result.push_str(&format!("{current_count}"));
            result.push(current_char);
            current_count = 1;
            current_char = c;
        }
    }
    result.push_str(&format!("{current_count}"));
    result.push(current_char);
    result
}

fn play(initial: &str, turns: usize) -> String {
    let mut current = initial.to_string();
    for _ in 0..turns {
        current = look_and_say(current.as_str());
    }
    current
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let length = play(input, 40).len();
    println!("Length after 40 turns: {length}");

    let length = play(input, 50).len();
    println!("Length after 50 turns: {length}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
