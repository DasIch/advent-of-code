use std::fmt::Write;

use md5::{Digest, Md5};

fn to_md5_hex(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();

    let mut hash = String::new();
    for byte in result {
        write!(&mut hash, "{:02x}", byte).unwrap();
    }
    hash
}

fn count_leading_zeros(s: &str) -> usize {
    s.chars().take_while(|&c| c == '0').count()
}

fn is_valid_answer(answer: &str, difficulty: usize) -> bool {
    let hash = to_md5_hex(answer);
    count_leading_zeros(&hash) >= difficulty
}

fn find_answer(secret_key: &str, difficulty: usize) -> Option<u64> {
    use rayon::prelude::*;

    (1..u64::MAX)
        .into_par_iter()
        .map(|i| (i, format!("{}{}", secret_key, i)))
        .find_first(|(_, candidate)| is_valid_answer(candidate.as_str(), difficulty))
        .map(|result| result.0)
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let (answer_part1, answer_part2) =
        rayon::join(|| find_answer(input, 5), || find_answer(input, 6));

    println!("[Part 1] Answer: {:?}", answer_part1);
    println!("[Part 2] Answer: {:?}", answer_part2);
    Ok(())
}
