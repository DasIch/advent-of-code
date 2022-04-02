use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn parse_ingredient(s: &str) -> Ingredient {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (\d+)$"
        ).unwrap();
    }

    let captures = RE.captures(s).unwrap();
    let capacity = captures.get(2).unwrap().as_str().parse().unwrap();
    let durability = captures.get(3).unwrap().as_str().parse().unwrap();
    let flavor = captures.get(4).unwrap().as_str().parse().unwrap();
    let texture = captures.get(5).unwrap().as_str().parse().unwrap();
    let calories = captures.get(6).unwrap().as_str().parse().unwrap();
    Ingredient {
        capacity,
        durability,
        flavor,
        texture,
        calories,
    }
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let ingredients: Vec<Ingredient> = input.lines().map(parse_ingredient).collect();
    assert_eq!(ingredients.len(), 4);

    let mut max_total_score = i64::MIN;
    let mut max_500_calories_score = i64::MIN;
    for i in 0..100 {
        for j in 0..(100 - i) {
            for k in 0..(100 - i - j) {
                let l = 100 - i - j - k;
                assert!(i + j + k + l == 100);

                let capacity = ingredients[0].capacity * i
                    + ingredients[1].capacity * j
                    + ingredients[2].capacity * k
                    + ingredients[3].capacity * l;
                let durability = ingredients[0].durability * i
                    + ingredients[1].durability * j
                    + ingredients[2].durability * k
                    + ingredients[3].durability * l;
                let flavor = ingredients[0].flavor * i
                    + ingredients[1].flavor * j
                    + ingredients[2].flavor * k
                    + ingredients[3].flavor * l;
                let texture = ingredients[0].texture * i
                    + ingredients[1].texture * j
                    + ingredients[2].texture * k
                    + ingredients[3].texture * l;
                let calories = ingredients[0].calories * i
                    + ingredients[1].calories * j
                    + ingredients[2].calories * k
                    + ingredients[3].calories * l;

                let total_score = {
                    if capacity >= 0 && durability >= 0 && flavor >= 0 && texture >= 0 {
                        capacity * durability * flavor * texture
                    } else {
                        0
                    }
                };
                if total_score > max_total_score {
                    max_total_score = total_score;
                }
                if calories == 500 && total_score > max_total_score {
                    max_500_calories_score = total_score;
                }
            }
        }
    }

    println!("Total score of highest scoring cookie: {max_total_score}");
    println!("Total score of highest scoring cookie with 500 calories: {max_500_calories_score}");
    Ok(())
}
