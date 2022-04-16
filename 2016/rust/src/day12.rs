use std::str::FromStr;

use anyhow::Context;

type Value = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            _ => Err(anyhow::anyhow!("invalid register: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Literal {
    Register(Register),
    Value(Value),
}

impl FromStr for Literal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "a" | "b" | "c" | "d" => Self::Register(s.parse()?),
            value => Self::Value(value.parse()?),
        })
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Copy {
        source: Literal,
        destination: Register,
    },
    Increase {
        register: Register,
    },
    Decrease {
        register: Register,
    },
    JumpIfNotZero {
        condition: Literal,
        offset: i32,
    },
}

#[derive(Debug)]
struct VM {
    register_a: Value,
    register_b: Value,
    register_c: Value,
    register_d: Value,
    program_counter: usize,
    instructions: Vec<Instruction>,
}

impl VM {
    fn new(instructions: &[Instruction]) -> Self {
        Self {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            register_d: 0,
            program_counter: 0,
            instructions: instructions.to_vec(),
        }
    }

    fn get_register(&self, register: Register) -> Value {
        match register {
            Register::A => self.register_a,
            Register::B => self.register_b,
            Register::C => self.register_c,
            Register::D => self.register_d,
        }
    }

    fn get_register_mut(&mut self, register: Register) -> &mut Value {
        match register {
            Register::A => &mut self.register_a,
            Register::B => &mut self.register_b,
            Register::C => &mut self.register_c,
            Register::D => &mut self.register_d,
        }
    }

    fn eval_literal(&self, literal: Literal) -> Value {
        match literal {
            Literal::Register(register) => self.get_register(register),
            Literal::Value(value) => value,
        }
    }

    fn eval(&mut self) -> anyhow::Result<()> {
        while let Some(instruction) = self.instructions.get(self.program_counter) {
            match *instruction {
                Instruction::Copy {
                    source,
                    destination,
                } => {
                    let value = self.eval_literal(source);
                    let register = self.get_register_mut(destination);
                    *register = value;
                    self.program_counter += 1;
                }
                Instruction::Increase { register } => {
                    *self.get_register_mut(register) += 1;
                    self.program_counter += 1;
                }
                Instruction::Decrease { register } => {
                    *self.get_register_mut(register) -= 1;
                    self.program_counter += 1;
                }
                Instruction::JumpIfNotZero { condition, offset } => {
                    if self.eval_literal(condition) == 0 {
                        self.program_counter += 1;
                    } else {
                        self.program_counter = (if offset > 0 {
                            self.program_counter.checked_add(offset as usize)
                        } else {
                            self.program_counter.checked_sub(offset.abs() as usize)
                        })
                        .context(format!(
                            "instruction @ pc {} has invalid offset {}",
                            self.program_counter, offset
                        ))?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let mut instructions = vec![];
    for line in input.lines() {
        instructions
            .push(parse_instruction(line).context(format!("invalid instruction: {line:?}"))?);
    }
    Ok(instructions)
}

fn parse_instruction(instruction: &str) -> anyhow::Result<Instruction> {
    match instruction.split_whitespace().collect::<Vec<&str>>()[..] {
        ["cpy", source, destination] => {
            let source = match source {
                "a" | "b" | "c" | "d" => Literal::Register(source.parse()?),
                literal => Literal::Value(literal.parse()?),
            };
            Ok(Instruction::Copy {
                source,
                destination: destination.parse()?,
            })
        }
        ["inc", register] => Ok(Instruction::Increase {
            register: register.parse()?,
        }),
        ["dec", register] => Ok(Instruction::Decrease {
            register: register.parse()?,
        }),
        ["jnz", register, offset] => Ok(Instruction::JumpIfNotZero {
            condition: register.parse()?,
            offset: offset.parse()?,
        }),
        _ => Err(anyhow::anyhow!("invalid instruction: {instruction:?}")),
    }
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let instructions = parse_input(input)?;

    let mut vm = VM::new(&instructions[..]);
    vm.eval()?;

    println!("Register a: {}", vm.register_a);

    let mut vm = VM::new(&instructions[..]);
    vm.register_c = 1;
    vm.eval()?;

    println!("Register a with c initialized to 1: {}", vm.register_a);

    Ok(())
}
