use std::collections::VecDeque;

use anyhow::Context;

static SPELLS: &'static [Spell] = &[
    Spell {
        name: "Magic Missile",
        mana_cost: 53,
        damage: 4,
        healing: 0,
        effect: None,
    },
    Spell {
        name: "Drain",
        mana_cost: 73,
        damage: 2,
        healing: 2,
        effect: None,
    },
    Spell {
        name: "Shield",
        mana_cost: 113,
        damage: 0,
        healing: 0,
        effect: Some(EffectDescription {
            duration: 6,
            armor: 7,
            damage: 0,
            mana: 0,
        }),
    },
    Spell {
        name: "Poison",
        mana_cost: 173,
        damage: 0,
        healing: 0,
        effect: Some(EffectDescription {
            duration: 6,
            armor: 0,
            damage: 3,
            mana: 0,
        }),
    },
    Spell {
        name: "Recharge",
        mana_cost: 229,
        damage: 0,
        healing: 0,
        effect: Some(EffectDescription {
            duration: 5,
            armor: 0,
            damage: 0,
            mana: 101,
        }),
    },
];

#[derive(Debug, Clone)]
struct Boss {
    hit_points: u32,
    damage: u32,
}

fn parse(input: &str) -> anyhow::Result<Boss> {
    let mut lines = input.lines();
    let hit_points = lines
        .next()
        .context("input ends before hit points")?
        .strip_prefix("Hit Points: ")
        .context("unexpected data where hit points were expected")?
        .parse()?;
    let damage = lines
        .next()
        .context("input ends before damage")?
        .strip_prefix("Damage: ")
        .context("unexpected data where damage was expected")?
        .parse()?;

    Ok(Boss { hit_points, damage })
}

#[derive(Debug, Clone)]
struct Spell {
    name: &'static str,
    mana_cost: u32,
    damage: u32,
    healing: u32,
    effect: Option<EffectDescription>,
}

#[derive(Debug, Clone)]
struct EffectDescription {
    duration: u32,
    armor: u32,
    damage: u32,
    mana: u32,
}

#[derive(Debug, Clone)]
struct Effect {
    name: &'static str,
    description: &'static EffectDescription,
    timer: u32,
}

#[derive(Debug, Clone)]
struct Player {
    hit_points: u32,
    mana_points: u32,
    active_effects: Vec<Effect>,
    spent_mana: u32,
}

impl Player {
    fn armor(&self) -> u32 {
        self.active_effects
            .iter()
            .map(|effect| effect.description.armor)
            .sum()
    }

    fn can_cast(&self, spell: &Spell) -> bool {
        let in_use = match spell.effect {
            Some(_) => self
                .active_effects
                .iter()
                .any(|effect| effect.name == spell.name && effect.timer > 1),
            None => false,
        };
        !in_use && spell.mana_cost <= self.mana_points
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    InProgress,
}

impl Outcome {
    fn and_then<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        match self {
            Self::InProgress => f(),
            otherwise => otherwise,
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    player: Player,
    boss: Boss,
    player_turn_penalty: u32,
}

impl Game {
    fn play_round(&mut self, spell: &'static Spell) -> Outcome {
        self.player_turn(spell).and_then(|| self.boss_turn())
    }

    fn player_turn(&mut self, spell: &'static Spell) -> Outcome {
        // It's assumed the caller has checked that the spell can be cast and that it's castable.

        self.player.hit_points = self
            .player
            .hit_points
            .saturating_sub(self.player_turn_penalty);

        if self.player.hit_points == 0 {
            return Outcome::Loss;
        }

        self.apply_effects().and_then(|| {
            self.player.mana_points -= spell.mana_cost;
            self.player.spent_mana += spell.mana_cost;
            if let Some(description) = &spell.effect {
                self.player.active_effects.push(Effect {
                    name: spell.name,
                    description,
                    timer: description.duration,
                })
            }
            if spell.damage > 0 {
                self.boss.hit_points = self.boss.hit_points.saturating_sub(spell.damage);
            }
            if spell.healing > 0 {
                self.player.hit_points += spell.healing;
            }
            match self.boss.hit_points {
                0 => Outcome::Win,
                _ => Outcome::InProgress,
            }
        })
    }

    fn boss_turn(&mut self) -> Outcome {
        self.apply_effects().and_then(|| {
            if self.boss.damage > self.player.armor() {
                let new_hit_points = self
                    .player
                    .hit_points
                    .saturating_sub(self.boss.damage - self.player.armor());
                self.player.hit_points = new_hit_points;
            } else {
                self.player.hit_points = self.player.hit_points.saturating_sub(1);
            };

            match self.player.hit_points {
                0 => Outcome::Loss,
                _ => Outcome::InProgress,
            }
        })
    }

    fn apply_effects(&mut self) -> Outcome {
        for effect in &mut self.player.active_effects {
            if effect.description.damage > 0 {
                self.boss.hit_points = self
                    .boss
                    .hit_points
                    .saturating_sub(effect.description.damage);
            }
            if effect.description.mana > 0 {
                self.player.mana_points += effect.description.mana;
            }
            effect.timer -= 1;
        }
        self.player.active_effects.retain(|effect| effect.timer > 0);

        match self.boss.hit_points {
            0 => Outcome::Win,
            _ => Outcome::InProgress,
        }
    }
}

fn find_least_mana_spent_game(initial_game: Game) -> Game {
    let mut games = VecDeque::from([initial_game]);
    let mut won_games = vec![];
    let mut min_spent_mana = u32::MAX;
    while let Some(game) = games.pop_front() {
        if min_spent_mana != u32::MAX && game.player.spent_mana > min_spent_mana {
            // mana spent is too high, let's stop pursuing this game any further
            continue;
        }
        let castable_spells = SPELLS.iter().filter(|spell| game.player.can_cast(spell));
        // if castable spells is empty, we implicity lose. in that case we do nothing.
        for spell in castable_spells {
            let mut game = game.clone();
            match game.play_round(spell) {
                Outcome::Win => {
                    if game.player.spent_mana < min_spent_mana {
                        min_spent_mana = game.player.spent_mana;
                    }
                    won_games.push(game);
                }
                Outcome::Loss => {
                    // :( Give up and do nothing
                }
                Outcome::InProgress => games.push_back(game),
            };
        }
    }

    won_games
        .into_iter()
        .min_by(|g1, g2| g1.player.spent_mana.cmp(&g2.player.spent_mana))
        .unwrap()
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let boss = parse(input)?;
    let player = Player {
        hit_points: 50,
        mana_points: 500,
        active_effects: vec![],
        spent_mana: 0,
    };
    let part1_game = find_least_mana_spent_game(Game {
        player: player.clone(),
        boss: boss.clone(),
        player_turn_penalty: 0,
    });
    println!(
        "Least mana spent in won game: {}",
        part1_game.player.spent_mana
    );

    let part2_game = find_least_mana_spent_game(Game {
        player,
        boss,
        player_turn_penalty: 1,
    });
    println!(
        "Least mana spent in won game with penalty: {}",
        part2_game.player.spent_mana
    );

    Ok(())
}
