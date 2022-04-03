use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Light::On => write!(f, "#"),
            Light::Off => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    fn is_coordinate_in_grid(&self, coordinate: (isize, isize)) -> bool {
        let size = self.0.len();
        coordinate.0 >= 0
            && (coordinate.0 as usize) < size
            && coordinate.1 >= 0
            && (coordinate.1 as usize) < size
    }
}

impl Grid<Light> {
    fn neighbors(&self) -> Grid<usize> {
        let mut grid = vec![vec![0; self.0.len()]; self.0.len()];

        for y in 0..self.0.len() {
            for x in 0..self.0.len() {
                let x = x as isize;
                let y = y as isize;
                let count = [
                    (x, y - 1),     // top
                    (x + 1, y - 1), // top right
                    (x + 1, y),     // right
                    (x + 1, y + 1), // bottom right
                    (x, y + 1),     // bottom
                    (x - 1, y + 1), // bottom left
                    (x - 1, y),     // left
                    (x - 1, y - 1), // top left
                ]
                .into_iter()
                .filter(|c| self.is_coordinate_in_grid(*c))
                .map(|c| self.0.get(c.1 as usize).unwrap().get(c.0 as usize).unwrap())
                .filter(|light| **light == Light::On)
                .count();
                *grid
                    .get_mut(y as usize)
                    .unwrap()
                    .get_mut(x as usize)
                    .unwrap() = count;
            }
        }

        Grid(grid)
    }
}

impl FromStr for Grid<Light> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lights: Vec<Vec<Light>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Light::On,
                        '.' => Light::Off,
                        _ => panic!("invalid input"),
                    })
                    .collect()
            })
            .collect();
        if lights.is_empty() || lights.len() != lights.first().unwrap().len() {
            return Err(anyhow::anyhow!("input is empty or not a square grid"));
        }
        Ok(Grid(lights))
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn step<F>(grid: &Grid<Light>, transition: F) -> Grid<Light>
where
    F: Fn(Light, usize, bool) -> Light,
{
    let mut result = vec![vec![Light::Off; grid.0.len()]; grid.0.len()];
    let neighbors = grid.neighbors();
    for (y, row) in grid.0.iter().enumerate() {
        for (x, light) in row.iter().enumerate() {
            let neighboring_lights = *neighbors.0.get(y).unwrap().get(x).unwrap();
            let is_corner = (x == 0 || x == grid.0.len() - 1) && (y == 0 || y == grid.0.len() - 1);
            let new_state = transition(*light, neighboring_lights, is_corner);
            *result.get_mut(y).unwrap().get_mut(x).unwrap() = new_state;
        }
    }
    Grid(result)
}

fn animate<F>(initial: &Grid<Light>, steps: usize, transition: F) -> Grid<Light>
where
    F: Fn(Light, usize, bool) -> Light,
{
    let mut grid = initial.clone();
    //    println!("Initial state:\n{}", grid);

    for _n in 1..=steps {
        grid = step(&grid, &transition);
        //        println!("After {n} step:\n{}", grid);
    }
    grid
}

fn part1(light: Light, neighbors: usize, _is_corner: bool) -> Light {
    match (light, neighbors) {
        (Light::On, 2 | 3) => Light::On,
        (Light::On, _) => Light::Off,
        (Light::Off, 3) => Light::On,
        (Light::Off, _) => Light::Off,
    }
}

fn part2(light: Light, neighbors: usize, is_corner: bool) -> Light {
    match (light, neighbors, is_corner) {
        (_, _, true) => Light::On,
        (Light::On, 2 | 3, _) => Light::On,
        (Light::On, _, _) => Light::Off,
        (Light::Off, 3, _) => Light::On,
        (Light::Off, _, _) => Light::Off,
    }
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let initial_grid: Grid<Light> = input.parse()?;

    let grid = animate(&initial_grid, 100, part1);
    let lights_on: usize = grid
        .0
        .iter()
        .map(|row| row.iter().filter(|light| **light == Light::On).count())
        .sum();
    println!("[Part 1]: After 100 steps {lights_on} lights are on");

    let grid = animate(&initial_grid, 100, part2);
    let lights_on: usize = grid
        .0
        .iter()
        .map(|row| row.iter().filter(|light| **light == Light::On).count())
        .sum();
    println!("[Part 2]: After 100 steps {lights_on} lights are on");

    Ok(())
}
