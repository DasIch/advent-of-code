use std::collections::HashMap;
use std::fmt;

fn parse_distance(line: &str) -> (String, String, usize) {
    let (cities, distance) = line.split_once(" = ").unwrap();
    let (a, b) = cities.split_once(" to ").unwrap();
    (a.to_string(), b.to_string(), distance.parse().unwrap())
}

#[derive(Debug, Clone)]
struct Route {
    current_location: String,
    visited_locations: Vec<String>,
    length: usize,
}

impl Route {
    fn new(initial_location: &str) -> Self {
        Route {
            current_location: initial_location.to_string(),
            visited_locations: vec![initial_location.to_string()],
            length: 0,
        }
    }

    fn visit(&self, next_location: &str, distance: usize) -> Option<Self> {
        if self.visited_locations.iter().any(|l| l == next_location) {
            None
        } else {
            let mut visited_locations = self.visited_locations.clone();
            visited_locations.push(next_location.to_string());
            Some(Route {
                current_location: next_location.to_string(),
                visited_locations,
                length: self.length + distance,
            })
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut locations = self.visited_locations.iter().peekable();
        while let Some(location) = locations.next() {
            if locations.peek().is_some() {
                write!(f, "{} -> ", location)?;
            } else {
                write!(f, "{}", location)?;
            }
        }
        write!(f, " = {}", self.length)
    }
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let mut graph: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for (city_a, city_b, distance) in input.lines().map(parse_distance) {
        let neighbors = graph.entry(city_a.clone()).or_default();
        neighbors.insert(city_b.clone(), distance);

        let neighbors = graph.entry(city_b.clone()).or_default();
        neighbors.insert(city_a.clone(), distance);
    }

    let mut completed_routes: Vec<Route> = vec![];
    let mut routes: Vec<Route> = graph.keys().map(|city| Route::new(city)).collect();
    while let Some(route) = routes.pop() {
        let next_locations = graph.get(route.current_location.as_str()).unwrap();
        for (location, distance) in next_locations {
            if let Some(new_route) = route.visit(location, *distance) {
                if new_route.visited_locations.len() == graph.len() {
                    completed_routes.push(new_route);
                } else {
                    routes.push(new_route);
                }
            }
        }
    }

    println!("{} completed routes found:", completed_routes.len());
    for route in &completed_routes {
        println!("{route}");
    }
    println!();

    let shortest_route = completed_routes
        .iter()
        .min_by(|r1, r2| r1.length.cmp(&r2.length))
        .unwrap();
    println!("Shortest route: {shortest_route}");

    let longest_route = completed_routes
        .iter()
        .max_by(|r1, r2| r1.length.cmp(&r2.length))
        .unwrap();
    println!("Longest route: {longest_route}");

    Ok(())
}
