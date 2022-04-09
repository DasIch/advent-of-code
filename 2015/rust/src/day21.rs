use std::cmp::max;

use anyhow::Context;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Character {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

impl From<&Build> for Character {
    fn from(build: &Build) -> Character {
        let damage = build.weapon.damage + build.rings.iter().map(|item| item.damage).sum::<i32>();
        let armor = build.armor.map(|item| item.armor).unwrap_or(0)
            + build.rings.iter().map(|item| item.armor).sum::<i32>();
        Character {
            hit_points: 100,
            damage,
            armor,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Item {
    cost: u32,
    damage: i32,
    armor: i32,
}

impl Item {
    const fn new(cost: u32, damage: i32, armor: i32) -> Self {
        Item {
            cost,
            damage,
            armor,
        }
    }
}

#[derive(Debug)]
struct Build {
    weapon: Item,
    armor: Option<Item>,
    rings: Vec<Item>,
}

impl Build {
    fn cost(&self) -> u32 {
        self.weapon.cost
            + self.armor.map(|item| item.cost).unwrap_or(0)
            + self.rings.iter().map(|item| item.cost).sum::<u32>()
    }
}

const WEAPONS: [Item; 5] = [
    Item::new(8, 4, 0),
    Item::new(10, 5, 0),
    Item::new(25, 6, 0),
    Item::new(40, 7, 0),
    Item::new(74, 8, 0),
];

const ARMOR: [Option<Item>; 6] = [
    None,
    Some(Item::new(13, 0, 1)),
    Some(Item::new(31, 0, 2)),
    Some(Item::new(53, 0, 3)),
    Some(Item::new(75, 0, 4)),
    Some(Item::new(102, 0, 5)),
];

const RINGS: [Item; 6] = [
    Item::new(25, 1, 0),
    Item::new(50, 2, 0),
    Item::new(100, 3, 0),
    Item::new(20, 0, 1),
    Item::new(40, 0, 2),
    Item::new(80, 0, 3),
];

fn parse_input(input: &str) -> anyhow::Result<Character> {
    let mut stats = input.trim().split('\n');
    let hit_points = stats
        .next()
        .context("input end before hit points")?
        .strip_prefix("Hit Points: ")
        .context("not hit points when expected")?
        .parse()?;
    let damage = stats
        .next()
        .context("input end before damage")?
        .strip_prefix("Damage: ")
        .context("no damage when expected")?
        .parse()?;
    let armor = stats
        .next()
        .context("input end before armor")?
        .strip_prefix("Armor: ")
        .context("no armor when expected")?
        .parse()?;
    Ok(Character {
        hit_points,
        damage,
        armor,
    })
}

fn play(mut player: Character, mut boss: Character) -> bool {
    for round in 0.. {
        if round % 2 == 0 {
            //            println!(
            //                "The player deals {}-{} = {} damage; the boss goes down to {} hit points.",
            //                player.damage,
            //                boss.armor,
            //                player.damage - boss.armor,
            //                boss.hit_points - max(player.damage - boss.armor, 1)
            //            );
            boss.hit_points -= max(player.damage - boss.armor, 1);
        } else {
            //            println!(
            //                "The boss deals {}-{} = {} damage; the boss goes down to {} hit points.",
            //                boss.damage,
            //                player.armor,
            //                boss.damage - player.armor,
            //                player.hit_points - max(boss.damage - player.armor, 1)
            //            );
            player.hit_points -= max(boss.damage - player.armor, 1);
        }

        if player.hit_points <= 0 || boss.hit_points <= 0 {
            break;
        }
    }
    player.hit_points > 0
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let enemy = parse_input(input)?;

    let mut builds = vec![];
    for weapon in WEAPONS {
        for armor in ARMOR {
            for ring_count in 0..=2 {
                for rings in RINGS
                    .iter()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .combinations(ring_count)
                {
                    builds.push(Build {
                        weapon,
                        armor,
                        rings: rings.into_iter().map(|item| *item).collect(),
                    });
                }
            }
        }
    }
    builds.sort_by_key(|build| build.cost());
    for build in &builds {
        if play(build.into(), enemy.clone()) {
            println!("Won fight with {} gold spent.", build.cost());
            break;
        }
    }

    for build in builds.iter().rev() {
        if !play(build.into(), enemy.clone()) {
            println!("Lost fight with {} gold spent.", build.cost());
            break;
        }
    }
    Ok(())
}
