use std::collections::HashMap;

use anyhow::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    turn: Turn,
    blocks: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_to_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input.trim().split(", ").map(parse_instruction).collect()
}

fn parse_instruction(s: &str) -> anyhow::Result<Instruction> {
    match (&s[0..1], &s[1..]) {
        ("L", blocks) => Ok(Instruction {
            turn: Turn::Left,
            blocks: blocks
                .parse()
                .context(format!("invalid instruction: {:?}", s))?,
        }),
        ("R", blocks) => Ok(Instruction {
            turn: Turn::Right,
            blocks: blocks
                .parse()
                .context(format!("invalid instruction: {:?}", s))?,
        }),
        _ => Err(anyhow::anyhow!("invalid instruction: {:?}", s)),
    }
}

fn follow_instructions(instructions: &[Instruction]) -> (i32, HashMap<Location, (u32, u32)>) {
    let mut facing = Direction::North;
    let mut location = Location::new(0, 0);
    let mut visit_counter = 0;
    let mut visited_locations = HashMap::new();

    for instruction in instructions {
        facing = match (facing, instruction.turn) {
            (Direction::North, Turn::Left) => Direction::West,
            (Direction::North, Turn::Right) => Direction::East,
            (Direction::East, Turn::Left) => Direction::North,
            (Direction::East, Turn::Right) => Direction::South,
            (Direction::South, Turn::Left) => Direction::East,
            (Direction::South, Turn::Right) => Direction::West,
            (Direction::West, Turn::Left) => Direction::South,
            (Direction::West, Turn::Right) => Direction::North,
        };
        for _ in 0..instruction.blocks {
            location = match facing {
                Direction::North => Location::new(location.x, location.y + 1),
                Direction::East => Location::new(location.x + 1, location.y),
                Direction::South => Location::new(location.x, location.y - 1),
                Direction::West => Location::new(location.x - 1, location.y),
            };
            let visits = visited_locations
                .entry(location)
                .or_insert((visit_counter, 0));
            (*visits).0 = visit_counter;
            (*visits).1 += 1;
            visit_counter += 1;
        }
    }

    (location.distance_to_origin(), visited_locations)
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let instructions = parse_input(input)?;

    let (distance, visited_locations) = follow_instructions(&instructions);

    println!("Distance from initial location: {distance}");

    let mut locations_visited_twice: Vec<(Location, (u32, u32))> = visited_locations
        .into_iter()
        .filter(|(_, (_, visits))| *visits == 2)
        .collect();
    locations_visited_twice.sort_by(|location1, location2| (location1.1).0.cmp(&(location2.1).0));

    let first_location_visited_twice = locations_visited_twice.first().unwrap().0;

    println!(
        "Distance from first location visited twice: {}",
        first_location_visited_twice.distance_to_origin()
    );

    Ok(())
}
