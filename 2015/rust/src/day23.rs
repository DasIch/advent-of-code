use std::str::FromStr;

use anyhow::Context;

#[derive(Debug)]
struct Computer {
    register_a: u32,
    register_b: u32,
    program_counter: isize,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn run(&mut self) {
        loop {
            let program_counter = match usize::try_from(self.program_counter) {
                Ok(program_counter) => program_counter,
                Err(_) => break,
            };
            let instruction = match self.instructions.get(program_counter) {
                Some(instruction) => instruction,
                None => break,
            };
            match instruction {
                Instruction::Half(Register::A) => {
                    self.register_a = self.register_a / 2;
                    self.program_counter += 1;
                }
                Instruction::Half(Register::B) => {
                    self.register_b = self.register_b / 2;
                    self.program_counter += 1;
                }
                Instruction::Triple(Register::A) => {
                    self.register_a = self.register_a * 3;
                    self.program_counter += 1;
                }
                Instruction::Triple(Register::B) => {
                    self.register_b = self.register_b * 3;
                    self.program_counter += 1;
                }
                Instruction::Increment(Register::A) => {
                    self.register_a += 1;
                    self.program_counter += 1;
                }
                Instruction::Increment(Register::B) => {
                    self.register_b += 1;
                    self.program_counter += 1;
                }
                Instruction::Jump(offset) => {
                    self.program_counter = self.program_counter + offset;
                }
                Instruction::JumpIfEven(register, offset) => {
                    let value = match register {
                        Register::A => self.register_a,
                        Register::B => self.register_b,
                    };
                    if value % 2 == 0 {
                        self.program_counter = self.program_counter + offset;
                    } else {
                        self.program_counter += 1;
                    }
                }
                Instruction::JumpIfOne(register, offset) => {
                    let value = match register {
                        Register::A => self.register_a,
                        Register::B => self.register_b,
                    };
                    if value == 1 {
                        self.program_counter = self.program_counter + offset;
                    } else {
                        self.program_counter += 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => Err(anyhow::anyhow!("invalid register: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match &line[..3] {
            "hlf" => Ok(Self::Half(line[4..].parse()?)),
            "tpl" => Ok(Self::Triple(line[4..].parse()?)),
            "inc" => Ok(Self::Increment(line[4..].parse()?)),
            "jmp" => Ok(Self::Jump(line[4..].parse()?)),
            "jie" => {
                let (register, offset) = line[4..]
                    .split_once(", ")
                    .context("invalid jie instruction")?;
                Ok(Self::JumpIfEven(register.parse()?, offset.parse()?))
            }
            "jio" => {
                let (register, offset) = line[4..]
                    .split_once(", ")
                    .context("invalid jio instruction")?;
                Ok(Self::JumpIfOne(register.parse()?, offset.parse()?))
            }
            _ => Err(anyhow::anyhow!("invalid instruction: {}", line)),
        }
    }
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let mut computer = Computer {
        register_a: 0,
        register_b: 0,
        program_counter: 0,
        instructions: instructions.clone(),
    };
    computer.run();
    println!("[Part 1]: Value in register a: {}", computer.register_a);
    println!("[Part 1]: Value in register b: {}", computer.register_b);

    let mut computer = Computer {
        register_a: 1,
        register_b: 0,
        program_counter: 0,
        instructions: instructions.clone(),
    };
    computer.run();
    println!("[Part 2]: Value in register a: {}", computer.register_a);
    println!("[Part 2]: Value in register b: {}", computer.register_b);

    Ok(())
}
