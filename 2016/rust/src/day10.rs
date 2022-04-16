use std::collections::HashMap;

type Microchip = u32;

#[derive(Debug)]
struct Bot {
    values: Vec<Microchip>,
}

#[derive(Debug)]
enum Destination {
    Output(String),
    Bot(String),
}

fn parse_input(
    input: &str,
) -> (
    HashMap<String, Bot>,
    HashMap<String, (Destination, Destination)>,
) {
    let mut bots = HashMap::new();
    let mut connections = HashMap::new();
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[..] {
            ["value", value, "goes", "to", "bot", bot] => {
                let entry = bots
                    .entry(bot.to_string())
                    .or_insert(Bot { values: vec![] });
                entry.values.push(value.parse().unwrap());
            }
            ["bot", bot, "gives", "low", "to", "bot", low, "and", "high", "to", "bot", high] => {
                connections.insert(
                    bot.to_string(),
                    (
                        Destination::Bot(low.to_string()),
                        Destination::Bot(high.to_string()),
                    ),
                );
            }
            ["bot", bot, "gives", "low", "to", "bot", low, "and", "high", "to", "output", high] => {
                connections.insert(
                    bot.to_string(),
                    (
                        Destination::Bot(low.to_string()),
                        Destination::Output(high.to_string()),
                    ),
                );
            }
            ["bot", bot, "gives", "low", "to", "output", low, "and", "high", "to", "bot", high] => {
                connections.insert(
                    bot.to_string(),
                    (
                        Destination::Output(low.to_string()),
                        Destination::Bot(high.to_string()),
                    ),
                );
            }
            ["bot", bot, "gives", "low", "to", "output", low, "and", "high", "to", "output", high] =>
            {
                connections.insert(
                    bot.to_string(),
                    (
                        Destination::Output(low.to_string()),
                        Destination::Output(high.to_string()),
                    ),
                );
            }
            _ => panic!("invalid instruction: {}", line),
        }
    }
    (bots, connections)
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let mut outputs: HashMap<String, Microchip> = HashMap::new();
    let (mut bots, connections) = parse_input(input);
    let mut comparisons = vec![];

    while let Some((name, bot)) = bots.iter_mut().find(|(_name, bot)| bot.values.len() == 2) {
        bot.values.sort_unstable();
        let high = bot.values.pop().unwrap();
        let low = bot.values.pop().unwrap();
        comparisons.push((name.to_string(), low, high));
        let (low_dest, high_dest) = connections.get(name).unwrap();
        match low_dest {
            Destination::Output(name) => {
                outputs.insert(name.to_string(), low);
            }
            Destination::Bot(name) => {
                let bot = bots
                    .entry(name.to_string())
                    .or_insert(Bot { values: vec![] });
                assert!(bot.values.len() < 2);
                bot.values.push(low);
            }
        }
        match high_dest {
            Destination::Output(name) => {
                outputs.insert(name.to_string(), high);
            }
            Destination::Bot(name) => {
                let bot = bots
                    .entry(name.to_string())
                    .or_insert(Bot { values: vec![] });
                assert!(bot.values.len() < 2);
                bot.values.push(high);
            }
        }
    }

    for (name, low, high) in comparisons {
        if low == 17 && high == 61 {
            println!("Comparison bot={name} low={low} high={high}");
        }
    }

    println!(
        "Product of 0, 1 and 2 outputs: {}",
        outputs.get("0").unwrap() * outputs.get("1").unwrap() * outputs.get("2").unwrap()
    );

    Ok(())
}
