use std::collections::HashSet;

fn has_abba(s: &str) -> bool {
    for i in 4..=s.len() {
        let sequence: Vec<char> = s[i - 4..i].chars().collect();
        if sequence[0] == sequence[3] && sequence[1] == sequence[2] && sequence[0] != sequence[1] {
            return true;
        }
    }
    false
}

#[derive(Debug)]
enum Sequence {
    Supernet(String),
    Hypernet(String),
}

fn parse_ipv7_address(mut address: &str) -> Vec<Sequence> {
    let mut result = vec![];
    let mut in_hypernet = false;
    loop {
        let delimiter = if in_hypernet { ']' } else { '[' };
        if let Some((sequence, remaining)) = address.split_once(delimiter) {
            if in_hypernet {
                result.push(Sequence::Hypernet(sequence.to_string()));
            } else {
                result.push(Sequence::Supernet(sequence.to_string()));
            }
            address = remaining;
            in_hypernet = !in_hypernet;
        } else {
            assert!(!in_hypernet);
            result.push(Sequence::Supernet(address.to_string()));
            break;
        }
    }
    result
}

fn supports_tls(address: &[Sequence]) -> bool {
    let mut result = false;
    for sequence in address {
        match sequence {
            Sequence::Supernet(s) if !result => result = has_abba(s),
            Sequence::Hypernet(s) if has_abba(s) => {
                return false;
            }
            _ => {}
        };
    }
    result
}

fn supports_ssl(address: &[Sequence]) -> bool {
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();
    for sequence in address {
        match sequence {
            Sequence::Supernet(s) => abas.extend(get_abas(s)),
            Sequence::Hypernet(s) => babs.extend(get_babs(s)),
        };
    }
    abas.intersection(&babs).count() > 0
}

fn get_abas(s: &str) -> HashSet<(char, char)> {
    let mut result = HashSet::new();
    for i in 3..=s.len() {
        let sequence: Vec<char> = s[i - 3..i].chars().collect();
        if sequence[0] == sequence[2] && sequence[0] != sequence[1] {
            result.insert((sequence[0], sequence[1]));
        }
    }
    result
}

fn get_babs(s: &str) -> HashSet<(char, char)> {
    get_abas(s).into_iter().map(|(b, a)| (a, b)).collect()
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let addresses: Vec<Vec<Sequence>> = input.lines().map(parse_ipv7_address).collect();

    let addresses_supporting_tls = addresses
        .iter()
        .filter(|address| supports_tls(address))
        .count();
    println!("{} addresses support TLS", addresses_supporting_tls);

    let addresses_supporting_ssl = addresses
        .iter()
        .filter(|address| supports_ssl(address))
        .count();
    println!("{} addresses support SSL", addresses_supporting_ssl);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_abba() {
        assert!(has_abba("xabba"));
        assert!(has_abba("abbax"));
        assert!(has_abba("abba"));
    }
}
