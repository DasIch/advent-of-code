fn unquote(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s
        .strip_prefix('"')
        .unwrap()
        .strip_suffix('"')
        .unwrap()
        .chars()
        .enumerate();
    while let Some((_, c)) = chars.next() {
        match c {
            '\\' => {
                match chars.next() {
                    Some((i, 'x')) => {
                        // add two to `i` to account for:
                        // - the initial "
                        // - the 'x' which we want to skip
                        let offset = i + 2;
                        let hex_notation = &s[offset..offset + 2];
                        let ordinal = u8::from_str_radix(hex_notation, 16).unwrap();
                        result.push(ordinal as char);

                        // skip
                        chars.next();
                        chars.next();
                    }
                    Some((_, c)) => {
                        result.push(c);
                    }
                    _ => unreachable!("invalid escape sequence"),
                }
            }
            c => {
                assert!(c != '\\');
                result.push(c);
            }
        }
    }
    result
}

fn quote(s: &str) -> String {
    let mut result = String::new();
    result.push('"');
    for c in s.chars() {
        match c {
            '"' => {
                result.push('\\');
                result.push(c);
            }
            '\\' => {
                result.push('\\');
                result.push('\\');
            }
            c => {
                result.push(c);
            }
        }
    }

    result.push('"');
    result
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let mut original_char_count = 0;
    let mut unquoted_char_count = 0;
    let mut quoted_char_count = 0;
    for line in input.lines() {
        original_char_count += line.chars().count();
        unquoted_char_count += unquote(line).chars().count();
        quoted_char_count += quote(line).chars().count();
    }

    println!(
        "[Part 1] Answer: {}",
        original_char_count - unquoted_char_count
    );

    println!(
        "[Part 1] Answer: {}",
        quoted_char_count - original_char_count
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unquote() {
        assert_eq!(unquote("\"\""), "");
        assert_eq!(unquote("\"\"").len(), 0);

        assert_eq!(unquote("\"abc\""), "abc");
        assert_eq!(unquote("\"abc\"").len(), 3);

        assert_eq!(unquote("\"aaa\\\"aaa\""), "aaa\"aaa");
        assert_eq!(unquote("\"aaa\\\"aaa\"").len(), 7);

        assert_eq!(unquote("\"\\x27\""), "\'");
        assert_eq!(unquote("\"\\x27\"").len(), 1);

        // custom cases
        assert_eq!(
            unquote("\"v\\xfb\\\"lgs\\\"kvjfywmut\\x9cr\""),
            "vû\"lgs\"kvjfywmut\u{9c}r"
        );
        assert_eq!(
            unquote("\"v\\xfb\\\"lgs\\\"kvjfywmut\\x9cr\"")
                .chars()
                .count(),
            18
        );
    }

    #[test]
    fn test_quote() {
        assert_eq!(quote("\"\""), "\"\\\"\\\"\"");
        assert_eq!(quote("\"\"").len(), 6);

        assert_eq!(quote("\"abc\""), "\"\\\"abc\\\"\"");
        assert_eq!(quote("\"abc\"").len(), 9);

        assert_eq!(quote("\"aaa\\\"aaa\""), "\"\\\"aaa\\\\\\\"aaa\\\"\"");
        assert_eq!(quote("\"aaa\\\"aaa\"").len(), 16);

        assert_eq!(quote("\"\\x27\""), "\"\\\"\\\\x27\\\"\"");
        assert_eq!(quote("\"\\x27\"").len(), 11);
    }
}
