use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
struct ReindeerDescription {
    name: String,
    velocity: u32,
    flying_time: u32,
    resting_time: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReindeerState {
    Flying,
    Resting,
}

#[derive(Debug)]
struct Reindeer {
    description: ReindeerDescription,
    state: ReindeerState,
    time_in_state: u32,
    distance_travelled: u32,
    points: u32,
}

impl Reindeer {
    fn new(description: &ReindeerDescription) -> Self {
        Reindeer {
            description: description.clone(),
            state: ReindeerState::Flying,
            time_in_state: 0,
            distance_travelled: 0,
            points: 0,
        }
    }

    fn tick(&mut self) {
        match self.state {
            ReindeerState::Flying if self.time_in_state == self.description.flying_time => {
                self.state = ReindeerState::Resting;
                self.time_in_state = 0;
            }
            ReindeerState::Resting if self.time_in_state == self.description.resting_time => {
                self.state = ReindeerState::Flying;
                self.time_in_state = 0;
            }
            _ => {}
        }
        self.time_in_state += 1;
        if self.state == ReindeerState::Flying {
            self.distance_travelled += self.description.velocity;
        }
    }
}

fn parse_description(s: &str) -> ReindeerDescription {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$"
        )
        .unwrap();
    }
    let captures = RE.captures(s).unwrap();
    let name = captures.get(1).unwrap().as_str().to_string();
    let velocity = captures.get(2).unwrap().as_str().parse().unwrap();
    let flying_time = captures.get(3).unwrap().as_str().parse().unwrap();
    let resting_time = captures.get(4).unwrap().as_str().parse().unwrap();

    ReindeerDescription {
        name,
        velocity,
        flying_time,
        resting_time,
    }
}

fn race(reindeers: &mut [Reindeer]) {
    for _ in 0..2503 {
        for reindeer in reindeers.iter_mut() {
            reindeer.tick();
        }
        let max_distance_travelled = reindeers
            .iter()
            .map(|reindeer| reindeer.distance_travelled)
            .max()
            .unwrap();
        for reindeer in reindeers.iter_mut() {
            if reindeer.distance_travelled == max_distance_travelled {
                reindeer.points += 1;
            }
        }
    }
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let mut reindeers: Vec<Reindeer> = input
        .lines()
        .map(parse_description)
        .map(|description| Reindeer::new(&description))
        .collect();

    race(&mut reindeers);

    let winner_by_distance_travelled = reindeers
        .iter()
        .max_by(|r1, r2| r1.distance_travelled.cmp(&r2.distance_travelled))
        .unwrap();

    println!(
        "{} travelled the max distance {} km",
        winner_by_distance_travelled.description.name,
        winner_by_distance_travelled.distance_travelled
    );

    let winner_by_points = reindeers
        .iter()
        .max_by(|r1, r2| r1.points.cmp(&r2.points))
        .unwrap();

    println!(
        "{} scored the most points {}",
        winner_by_points.description.name, winner_by_points.points
    );

    Ok(())
}
