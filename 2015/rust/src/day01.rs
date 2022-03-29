pub fn main(input: &str) -> anyhow::Result<()> {
    let mut floor = 0;
    let mut first_basement_visit_at_position = None;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => {}
        }
        if floor == -1 && first_basement_visit_at_position == None {
            // The first character in the instructions has position 1, the second character has position 2, and so on.
            first_basement_visit_at_position = Some(i + 1);
        }
    }
    println!("Floor: {}", floor);
    println!(
        "First basement visit at position: {:?}",
        first_basement_visit_at_position
    );
    Ok(())
}
