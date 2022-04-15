use std::io::Write;

use md5::{Digest, Md5};

fn to_md5_hex(s: &str) -> String {
    use std::fmt::Write;
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();

    let mut hash = String::new();
    for byte in result {
        write!(&mut hash, "{:02x}", byte).unwrap();
    }
    hash
}

fn find_first_door_password(door_id: &str) {
    let mut password = String::new();
    let mut indixes = 0..;
    while password.len() < 8 {
        let s = format!("{}{}", door_id, indixes.next().unwrap());
        let h = to_md5_hex(s.as_str());
        if &h[0..5] == "00000" {
            let c = h.chars().nth(5).unwrap();
            password.push(c);
            print!("\rFirst door password: {}", password);
            std::io::stdout().flush().unwrap();
        }
    }
    println!();
}

fn find_second_door_password(door_id: &str) {
    let mut password = vec!['_', '_', '_', '_', '_', '_', '_', '_'];
    let mut indixes = 1..;
    while password.iter().any(|c| *c == '_') {
        let s = format!("{}{}", door_id, indixes.next().unwrap());
        let h = to_md5_hex(s.as_str());
        if &h[0..5] == "00000" {
            if let Some(position) = h.chars().nth(5).unwrap().to_digit(10) {
                if let Some(c) = password.get_mut(position as usize) {
                    if *c == '_' {
                        *c = h.chars().nth(6).unwrap();
                    }
                }
            }
            print!(
                "\rSecond door password: {}",
                password.iter().collect::<String>()
            );
            std::io::stdout().flush().unwrap();
        }
    }
    println!();
}

pub fn main(input: &str) -> anyhow::Result<()> {
    find_first_door_password(input);
    find_second_door_password(input);

    Ok(())
}
