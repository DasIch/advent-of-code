use std::collections::HashSet;

type Position = (i64, i64);

struct Santa {
    visited_houses: HashSet<Position>,
    position: Position,
}

impl Santa {
    fn move_to(&mut self, instruction: char) {
        match instruction {
            '^' => {
                self.position = (self.position.0, self.position.1 + 1);
            }
            'v' => {
                self.position = (self.position.0, self.position.1 - 1);
            }
            '>' => {
                self.position = (self.position.0 + 1, self.position.1);
            }
            '<' => {
                self.position = (self.position.0 - 1, self.position.1);
            }
            _ => {}
        }

        self.visited_houses.insert(self.position);
    }
}

impl Default for Santa {
    fn default() -> Self {
        Santa {
            visited_houses: HashSet::from([(0, 0)]),
            position: (0, 0),
        }
    }
}

fn part1(input: &str) -> anyhow::Result<()> {
    let mut santa = Santa::default();

    for c in input.chars() {
        santa.move_to(c);
    }

    println!(
        "[Part 1]: Houses that received at least one present: {}",
        santa.visited_houses.len()
    );

    Ok(())
}

fn part2(input: &str) -> anyhow::Result<()> {
    let mut santa = Santa::default();
    let mut robo_santa = Santa::default();

    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            santa.move_to(c);
        } else {
            robo_santa.move_to(c);
        }
    }

    let visited_houses = santa
        .visited_houses
        .union(&robo_santa.visited_houses)
        .count();
    println!(
        "[Part 2]: Houses that received at least one present: {}",
        visited_houses
    );

    Ok(())
}

pub fn main(input: &str) -> anyhow::Result<()> {
    part1(input)?;
    part2(input)?;

    Ok(())
}
