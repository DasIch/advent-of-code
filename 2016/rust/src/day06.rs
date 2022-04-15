use std::collections::HashMap;

pub fn main(input: &str) -> anyhow::Result<()> {
    let messages: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let mut counts: Vec<HashMap<char, usize>> = messages
        .first()
        .unwrap()
        .chars()
        .map(|_| HashMap::new())
        .collect();

    for message in &messages {
        for (c, column_counts) in message.chars().zip(counts.iter_mut()) {
            *column_counts.entry(c).or_insert(0) += 1;
        }
    }

    let error_corrected_message_max: String = counts
        .iter()
        .map(|column_counts| {
            column_counts
                .iter()
                .max_by_key(|(_, count)| *count)
                .unwrap()
                .0
        })
        .collect();

    println!(
        "Error-corrected message (max): {}",
        error_corrected_message_max
    );

    let error_corrected_message_min: String = counts
        .iter()
        .map(|column_counts| {
            column_counts
                .iter()
                .min_by_key(|(_, count)| *count)
                .unwrap()
                .0
        })
        .collect();

    println!(
        "Error-corrected message (min): {}",
        error_corrected_message_min
    );

    Ok(())
}
