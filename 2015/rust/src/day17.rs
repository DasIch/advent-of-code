use itertools::Itertools;

pub fn main(input: &str) -> anyhow::Result<()> {
    let target_capacity = 150;
    let containers: Vec<u64> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let mut combinations = vec![];
    for k in 1..containers.len() {
        for combination in containers.clone().into_iter().enumerate().combinations(k) {
            let mut prefix = vec![];
            let mut total_capacity = 0;
            for container in &combination {
                prefix.push(*container);
                total_capacity += container.1;
                if total_capacity >= target_capacity {
                    break;
                }
            }
            if total_capacity == target_capacity && !combinations.contains(&prefix) {
                combinations.push(prefix);
            }
        }
    }

    println!("Possible different combinations: {}", combinations.len());

    let min_containers = combinations
        .iter()
        .map(|combination| combination.len())
        .min()
        .unwrap();
    let combinations_with_min_containers = combinations
        .iter()
        .filter(|combination| combination.len() == min_containers)
        .count();
    println!("Possible containers with min container count ({min_containers}): {combinations_with_min_containers}");

    Ok(())
}
