static KEYPAD1: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

static KEYPAD2: [[char; 5]; 5] = [
    ['0', '0', '1', '0', '0'],
    ['0', '2', '3', '4', '0'],
    ['5', '6', '7', '8', '9'],
    ['0', 'A', 'B', 'C', '0'],
    ['0', '0', 'D', '0', '0'],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<Move>>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'U' => Ok(Move::Up),
                    'R' => Ok(Move::Right),
                    'D' => Ok(Move::Down),
                    'L' => Ok(Move::Left),
                    _ => Err(anyhow::anyhow!("invalid character")),
                })
                .collect()
        })
        .collect()
}

fn follow_instructions<const N: usize>(
    keypad: [[char; N]; N],
    initial_position: (usize, usize),
    instructions: &[Vec<Move>],
) -> String {
    let (mut y, mut x) = initial_position;
    let mut code = String::new();
    for button in instructions {
        for m in button {
            match m {
                Move::Up if y > 0 && keypad[y - 1][x] != '0' => {
                    y -= 1;
                }
                Move::Right if x < keypad.len() - 1 && keypad[y][x + 1] != '0' => {
                    x += 1;
                }
                Move::Down if y < keypad.len() - 1 && keypad[y + 1][x] != '0' => {
                    y += 1;
                }
                Move::Left if x > 0 && keypad[y][x - 1] != '0' => {
                    x -= 1;
                }
                _ => {}
            };
        }

        code.push(keypad[y][x]);
    }
    code
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let instructions = parse_input(input)?;

    println!(
        "Code for imagined keypad layout: {}",
        follow_instructions(KEYPAD1, (1, 1), &instructions)
    );
    println!(
        "Code for actual keypad layout: {}",
        follow_instructions(KEYPAD2, (2, 0), &instructions)
    );

    Ok(())
}
