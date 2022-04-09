use rayon::prelude::*;

fn calculate_presents_part1(house: u32) -> u32 {
    let present_factor = 10;
    let mut presents = 0;
    for elf in 1..=house {
        let visits_house = house % elf == 0;
        if visits_house {
            presents += elf * present_factor;
        }
    }
    presents
}

fn calculate_presents_part2(house: u32) -> u32 {
    let present_factor = 11;
    let max_visits = 50;
    let mut presents = 0;
    for elf in 1..=house {
        let visits_house = (house % elf == 0)
            && ((elf..=house)
                .step_by(elf as usize)
                .take(max_visits)
                .last()
                .unwrap()
                == house);
        if visits_house {
            presents += elf * present_factor;
        }
    }
    presents
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let expected_presents = input.trim().parse()?;
    let house = (1..(expected_presents / 10))
        .into_par_iter()
        .find_first(|house| calculate_presents_part1(*house) >= expected_presents)
        .unwrap();
    println!(
        "[Part 1]: House {} got {} presents. The first to have >= {} presents.",
        house,
        calculate_presents_part1(house),
        expected_presents
    );

    let house = (1..(expected_presents / 10))
        .into_par_iter()
        .find_first(|house| calculate_presents_part2(*house) >= expected_presents)
        .unwrap();
    println!(
        "[Part 2]: House {} got {} presents. The first to have >= {} presents.",
        house,
        calculate_presents_part2(house),
        expected_presents
    );

    Ok(())
}
