use std::collections::{BinaryHeap, HashSet};

const TARGET: Coordinate = (31, 39);

type Coordinate = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    OpenSpace,
    Wall,
    Visited,
}

fn evaluate_coordinate((x, y): Coordinate, favorite_number: usize) -> Field {
    let value = (x * x + 3 * x + 2 * x * y + y + y * y) + favorite_number;
    if value.count_ones() % 2 == 0 {
        Field::OpenSpace
    } else {
        Field::Wall
    }
}

fn find_surrounding_open_spaces((x, y): Coordinate, favorite_number: usize) -> Vec<Coordinate> {
    let neighbors = [
        (Some(x), y.checked_sub(1)), // Up
        (Some(x + 1), Some(y)),      // Right
        (Some(x), Some(y + 1)),      // Down
        (x.checked_sub(1), Some(y)), // Left
    ];
    let mut open_spaces = vec![];
    for neighbor in neighbors {
        if let (Some(x), Some(y)) = neighbor {
            if evaluate_coordinate((x, y), favorite_number) == Field::OpenSpace {
                open_spaces.push((x, y));
            }
        }
    }
    open_spaces
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path(Vec<Coordinate>);

impl Path {
    fn new(initial: Coordinate) -> Self {
        Self(vec![initial])
    }

    fn current_position(&self) -> Coordinate {
        *self.0.last().unwrap()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn with_next(&self, coordinate: Coordinate) -> Self {
        let mut coordinates = self.0.clone();
        coordinates.push(coordinate);
        Self(coordinates)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShortestPath(Path);

impl ShortestPath {
    fn distance_to_target(&self) -> f64 {
        distance(self.0.current_position(), TARGET)
    }

    fn with_next(&self, coordinate: Coordinate) -> Self {
        Self(self.0.with_next(coordinate))
    }
}

fn distance((x1, y1): Coordinate, (x2, y2): Coordinate) -> f64 {
    let x1 = x1 as f64;
    let y1 = y1 as f64;
    let x2 = x2 as f64;
    let y2 = y2 as f64;
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

impl Ord for ShortestPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort in such a way that the shortest path closest to TARGET wins.
        other.0.len().cmp(&self.0.len()).then_with(|| {
            self.distance_to_target()
                .partial_cmp(&other.distance_to_target())
                .unwrap()
        })
    }
}

impl PartialOrd for ShortestPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_path_to_target(favorite_number: usize) -> Path {
    let mut paths = BinaryHeap::from([ShortestPath(Path::new((1, 1)))]);
    let mut shortest_path: Option<Path> = None;
    let mut visited_coordinates = HashSet::from([(1, 1)]);
    while let Some(path) = paths.pop() {
        let current_position = path.0.current_position();
        if current_position == TARGET {
            match shortest_path {
                Some(current_shortest_path) if path.0.len() < current_shortest_path.0.len() => {
                    shortest_path = Some(path.0);
                }
                None => shortest_path = Some(path.0),
                _ => {}
            };
        } else {
            for neighbor in find_surrounding_open_spaces(current_position, favorite_number) {
                if !visited_coordinates.contains(&neighbor) {
                    paths.push(path.with_next(neighbor));
                    visited_coordinates.insert(neighbor);
                }
            }
        }
    }
    shortest_path.unwrap()
}

fn map_size(path: &Path) -> (usize, usize) {
    let width = path.0.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let height = path.0.iter().map(|(_, y)| *y).max().unwrap() + 1;
    (width, height)
}

fn create_map(width: usize, height: usize, favorite_number: usize) -> Vec<Vec<Field>> {
    let mut map = vec![];
    for y in 0..height {
        let mut row = vec![];
        for x in 0..width {
            row.push(evaluate_coordinate((x, y), favorite_number));
        }
        map.push(row);
    }
    map
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let favorite_number = input.parse()?;

    let shortest_path = find_shortest_path_to_target(favorite_number);
    let (width, height) = map_size(&shortest_path);
    let mut map = create_map(width, height, favorite_number);
    for (x, y) in &shortest_path.0 {
        map[*y][*x] = Field::Visited;
    }

    for row in map {
        println!(
            "{}",
            row.iter()
                .map(|field| match field {
                    Field::OpenSpace => '.',
                    Field::Wall => '#',
                    Field::Visited => 'O',
                })
                .collect::<String>()
        );
    }

    println!("Steps in shortest path: {}", shortest_path.0.len() - 1);

    Ok(())
}
