fn parse_input(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .map(|line| {
            let (length1, remaining) = line.trim().split_once(' ').unwrap();
            let (length2, remaining) = remaining.trim().split_once(' ').unwrap();
            let length3 = remaining.trim();
            (
                length1.parse().unwrap(),
                length2.parse().unwrap(),
                length3.parse().unwrap(),
            )
        })
        .collect()
}

fn is_triangle((a, b, c): (u32, u32, u32)) -> bool {
    (a + b > c) && (b + c > a) && (c + a > b)
}

fn regroup(triangles: &[(u32, u32, u32)]) -> Vec<(u32, u32, u32)> {
    let mut result = vec![];
    for i in (2..triangles.len()).step_by(3) {
        let x = triangles[i - 2];
        let y = triangles[i - 1];
        let z = triangles[i];
        result.push((x.0, y.0, z.0));
        result.push((x.1, y.1, z.1));
        result.push((x.2, y.2, z.2));
    }
    result
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let triangles = parse_input(input);
    let possible_triangle_count = triangles
        .iter()
        .filter(|triangle| is_triangle(**triangle))
        .count();

    println!("Possible triangles (horizontal): {possible_triangle_count}");

    let possible_triangle_count = regroup(&triangles)
        .iter()
        .filter(|triangle| is_triangle(**triangle))
        .count();

    println!("Possible triangles (vertical): {possible_triangle_count}");

    Ok(())
}
