use std::collections::VecDeque;
use std::fmt;

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

#[derive(Debug)]
struct Screen {
    pixels: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Screen {
    fn new() -> Self {
        Self {
            pixels: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    fn turn_on(&mut self, width: usize, height: usize) {
        for row in 0..height {
            for pixel in 0..width {
                self.pixels[row][pixel] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, k: usize) {
        let mut rotated: VecDeque<bool> = self.pixels[row].iter().map(|pixel| *pixel).collect();
        rotated.rotate_right(k);
        for (i, pixel) in rotated.into_iter().enumerate() {
            self.pixels[row][i] = pixel;
        }
    }

    fn rotate_column(&mut self, column: usize, k: usize) {
        let mut rotated: VecDeque<bool> = self.pixels.iter().map(|row| row[column]).collect();
        rotated.rotate_right(k);
        for (row, pixel) in rotated.into_iter().enumerate() {
            self.pixels[row][column] = pixel;
        }
    }

    fn lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|pixel| **pixel).count())
            .sum()
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.pixels {
            for pixel in row {
                if pixel {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
enum Instruction {
    Rect { width: usize, height: usize },
    RotateRow { row: usize, k: usize },
    RotateColumn { column: usize, k: usize },
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in input.lines() {
        if line.starts_with("rect") {
            let (width, height) = line.split_once(' ').unwrap().1.split_once('x').unwrap();
            instructions.push(Instruction::Rect {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
            });
        } else if line.starts_with("rotate row") {
            let (row, k) = line.split_once('=').unwrap().1.split_once(" by ").unwrap();
            instructions.push(Instruction::RotateRow {
                row: row.parse().unwrap(),
                k: k.parse().unwrap(),
            });
        } else if line.starts_with("rotate column") {
            let (column, k) = line.split_once('=').unwrap().1.split_once(" by ").unwrap();
            instructions.push(Instruction::RotateColumn {
                column: column.parse().unwrap(),
                k: k.parse().unwrap(),
            });
        } else {
            panic!("invalid line: {:?}", line)
        }
    }
    instructions
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let instructions = parse_input(input);
    let mut screen = Screen::new();
    for instruction in instructions {
        match instruction {
            Instruction::Rect { width, height } => screen.turn_on(width, height),
            Instruction::RotateRow { row, k } => screen.rotate_row(row, k),
            Instruction::RotateColumn { column, k } => screen.rotate_column(column, k),
        }
        println!("{}", screen);
    }

    println!("Lit pixels: {}", screen.lit_pixels());

    Ok(())
}
