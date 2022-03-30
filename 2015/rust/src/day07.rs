use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
enum Output {
    Value(u16),
    MissingSignal(String),
}

impl Output {
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(u16) -> u16,
    {
        match self {
            Self::Value(value) => Self::Value(f(value)),
            missing => missing,
        }
    }
}

trait Eval {
    fn eval(&self, wires: &HashMap<String, Instruction>, signals: &HashMap<String, u16>) -> Output;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    source: Source,
    output: String,
}

impl Eval for Instruction {
    fn eval(&self, wires: &HashMap<String, Instruction>, signals: &HashMap<String, u16>) -> Output {
        self.source.eval(wires, signals)
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" -> ") {
            Some((source, output)) => Ok(Instruction {
                source: source.parse()?,
                output: output.to_string(),
            }),
            None => Err(anyhow::anyhow!("invalid instruction: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Source {
    Gate(Gate),
    Literal(Literal),
}

impl Eval for Source {
    fn eval(&self, wires: &HashMap<String, Instruction>, signals: &HashMap<String, u16>) -> Output {
        match self {
            Self::Gate(gate) => gate.eval(wires, signals),
            Self::Literal(literal) => literal.eval(wires, signals),
        }
    }
}

impl FromStr for Source {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(' ') {
            Ok(Self::Gate(s.parse()?))
        } else {
            Ok(Self::Literal(s.parse()?))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Literal {
    Wire(String),
    Value(u16),
}

impl Eval for Literal {
    fn eval(
        &self,
        _wires: &HashMap<String, Instruction>,
        signals: &HashMap<String, u16>,
    ) -> Output {
        match self {
            Self::Wire(name) => match signals.get(name) {
                Some(value) => Output::Value(*value),
                None => Output::MissingSignal(name.to_string()),
            },
            Self::Value(value) => Output::Value(*value),
        }
    }
}

impl FromStr for Literal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(char::is_numeric) {
            Ok(Self::Value(s.parse()?))
        } else {
            Ok(Self::Wire(s.to_string()))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Gate {
    Not(Literal),
    And(Literal, Literal),
    Or(Literal, Literal),
    LShift(Literal, Literal),
    RShift(Literal, Literal),
}

fn apply_operation<F>(
    left: &Literal,
    right: &Literal,
    wires: &HashMap<String, Instruction>,
    signals: &HashMap<String, u16>,
    f: F,
) -> Output
where
    F: Fn(u16, u16) -> u16,
{
    match (left.eval(wires, signals), right.eval(wires, signals)) {
        (Output::Value(lv), Output::Value(rv)) => Output::Value(f(lv, rv)),
        (Output::MissingSignal(name), _) => Output::MissingSignal(name),
        (_, Output::MissingSignal(name)) => Output::MissingSignal(name),
    }
}

impl Eval for Gate {
    fn eval(&self, wires: &HashMap<String, Instruction>, signals: &HashMap<String, u16>) -> Output {
        match self {
            Self::Not(operand) => operand.eval(wires, signals).map(|v| u16::MAX - v),
            Self::And(l, r) => apply_operation(l, r, wires, signals, |l, r| l & r),
            Self::Or(l, r) => apply_operation(l, r, wires, signals, |l, r| l | r),
            Self::LShift(l, r) => apply_operation(l, r, wires, signals, |l, r| l << r),
            Self::RShift(l, r) => apply_operation(l, r, wires, signals, |l, r| l >> r),
        }
    }
}

impl FromStr for Gate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(remaining) = s.strip_prefix("NOT ") {
            Ok(Self::Not(remaining.parse()?))
        } else if let Some((left, right)) = s.split_once(" AND ") {
            Ok(Self::And(left.parse()?, right.parse()?))
        } else if let Some((left, right)) = s.split_once(" OR ") {
            Ok(Self::Or(left.parse()?, right.parse()?))
        } else if let Some((left, right)) = s.split_once(" LSHIFT ") {
            Ok(Self::LShift(left.parse()?, right.parse()?))
        } else if let Some((left, right)) = s.split_once(" RSHIFT ") {
            Ok(Self::RShift(left.parse()?, right.parse()?))
        } else {
            Err(anyhow::anyhow!("invalid gate: {}", s))
        }
    }
}

fn eval(wires: &HashMap<String, Instruction>, signals: &mut HashMap<String, u16>) {
    let mut queue: VecDeque<String> = wires.keys().cloned().collect();

    while let Some(name) = queue.pop_front() {
        let instruction = wires.get(name.as_str()).unwrap();
        let output = instruction.eval(wires, signals);
        match output {
            Output::Value(value) => {
                signals.insert(name, value);
            }
            Output::MissingSignal(name_of_missing_wire) => {
                // get signal for missing wire
                queue.push_front(name_of_missing_wire);
                // retry later
                queue.push_back(name);
            }
        }
    }
}

fn print_signals(signals: &HashMap<String, u16>) {
    let mut pairs: Vec<(String, u16)> = signals
        .iter()
        .map(|pair| (pair.0.clone(), *pair.1))
        .collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, value) in pairs {
        println!("{}: {}", name, value);
    }
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let wires: HashMap<String, Instruction> = input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .map(|i| i.map(|instruction| (instruction.output.clone(), instruction)))
        .collect::<Result<_, _>>()?;

    let mut signals: HashMap<String, u16> = HashMap::new();

    eval(&wires, &mut signals);

    print_signals(&signals);

    if let Some(wire_a) = signals.get("a") {
        println!("[Part 1] Signal provided to wire a: {}", wire_a);

        let mut signals = HashMap::from([("b".to_string(), *wire_a)]);
        eval(&wires, &mut signals);
        let wire_a = signals.get("a").unwrap();
        println!("[Part 2] Signal provided to wire a: {}", wire_a);
    }

    Ok(())
}
