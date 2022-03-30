use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl FromStr for Coordinate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((x, y)) => Ok(Self {
                x: x.parse()?,
                y: y.parse()?,
            }),
            None => Err(anyhow::anyhow!("invalid coordinate: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
struct Rectangle {
    bottom_left: Coordinate,
    top_right: Coordinate,
}

impl FromStr for Rectangle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" through ") {
            Some((bottom_left, top_right)) => Ok(Self {
                bottom_left: bottom_left.parse()?,
                top_right: top_right.parse()?,
            }),
            None => Err(anyhow::anyhow!("invalid rectangle: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    command: Command,
    rectangle: Rectangle,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some(remaining) = line.strip_prefix("turn on ") {
            Ok(Self {
                command: Command::TurnOn,
                rectangle: remaining.parse()?,
            })
        } else if let Some(remaining) = line.strip_prefix("turn off ") {
            Ok(Self {
                command: Command::TurnOff,
                rectangle: remaining.parse()?,
            })
        } else if let Some(remaining) = line.strip_prefix("toggle ") {
            Ok(Self {
                command: Command::Toggle,
                rectangle: remaining.parse()?,
            })
        } else {
            Err(anyhow::anyhow!("Invalid instruction: {}", line))
        }
    }
}

fn apply_instructions<T, F>(grid: &mut [[T; 1000]; 1000], instructions: &[Instruction], eval: F)
where
    F: Fn(&mut T, &Instruction),
{
    for instruction in instructions {
        for row in grid
            .iter_mut()
            .take(instruction.rectangle.top_right.y + 1)
            .skip(instruction.rectangle.bottom_left.y)
        {
            for cell in row
                .iter_mut()
                .take(instruction.rectangle.top_right.x + 1)
                .skip(instruction.rectangle.bottom_left.x)
            {
                eval(cell, instruction)
            }
        }
    }
}

fn part1(instructions: &[Instruction]) {
    let mut grid = [[false; 1000]; 1000];

    apply_instructions(
        &mut grid,
        instructions,
        |cell, instruction| match instruction.command {
            Command::TurnOn => *cell = true,
            Command::TurnOff => *cell = false,
            Command::Toggle => *cell = !*cell,
        },
    );

    let lit_lights: usize = grid
        .iter()
        .map(|row| row.iter().filter(|&b| *b).count())
        .sum();
    println!("{} lights are lit", lit_lights);
}

fn part2(instructions: &[Instruction]) {
    let mut grid = [[0u64; 1000]; 1000];

    apply_instructions(
        &mut grid,
        instructions,
        |cell, instruction| match instruction.command {
            Command::TurnOn => *cell += 1,
            Command::TurnOff => *cell = cell.saturating_sub(1),
            Command::Toggle => *cell += 2,
        },
    );

    let brightness: u64 = grid.iter().map(|row| row.iter().sum::<u64>()).sum();
    println!("total brightness {}", brightness);
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()?;

    part1(&instructions);
    part2(&instructions);

    Ok(())
}
