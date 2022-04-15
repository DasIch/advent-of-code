use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn name(&self) -> String {
        decrypt(&self.encrypted_name, self.sector_id)
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}[{}]",
            self.encrypted_name, self.sector_id, self.checksum
        )
    }
}

fn parse_input(input: &str) -> Vec<Room> {
    input.lines().map(|line| parse_room(line)).collect()
}

fn parse_room(line: &str) -> Room {
    let (left, checksum) = line.strip_suffix("]").unwrap().split_once('[').unwrap();
    let (encrypted_name, sector_id) = left.rsplit_once('-').unwrap();
    Room {
        encrypted_name: encrypted_name.to_string(),
        sector_id: sector_id.parse().unwrap(),
        checksum: checksum.to_string(),
    }
}

fn calculate_checksum(encrypted_name: &str) -> String {
    let mut char_counts = HashMap::new();
    for c in encrypted_name.chars() {
        if c == '-' {
            continue;
        }
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
    }
    let mut pairs: Vec<(char, usize)> = char_counts.into_iter().collect();
    pairs.sort_by(|(c1, count1), (c2, count2)| count2.cmp(count1).then_with(|| c1.cmp(c2)));

    pairs.iter().take(5).map(|(c, _)| c).collect()
}

fn is_real(room: &Room) -> bool {
    let calculated_checksum = calculate_checksum(&room.encrypted_name);
    room.checksum == calculated_checksum
}

fn rotate_char1(c: char) -> char {
    match c {
        'z' => 'a',
        c => char::from_u32((c as u32) + 1).unwrap(),
    }
}

fn rotate_char(mut c: char, n: u32) -> char {
    for _ in 0..n {
        c = rotate_char1(c)
    }
    c
}

fn decrypt(encrypted_name: &str, n: u32) -> String {
    encrypted_name
        .chars()
        .map(|c| if c == '-' { c } else { rotate_char(c, n) })
        .collect()
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let rooms = parse_input(input);
    let real_rooms: Vec<&Room> = rooms.iter().filter(|room| is_real(room)).collect();

    println!(
        "Sum of sector IDs of the real rooms: {}",
        real_rooms.iter().map(|room| room.sector_id).sum::<u32>()
    );

    println!(
        "Sector ID of northpole-object-storage: {}",
        rooms
            .iter()
            .filter(|room| room.name() == "northpole-object-storage")
            .next()
            .unwrap()
            .sector_id
    );

    Ok(())
}
