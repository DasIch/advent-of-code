use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug)]
struct GiftBox {
    length: u64,
    width: u64,
    height: u64,
}

impl GiftBox {
    fn surface_area(&self) -> u64 {
        (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length)
    }

    fn required_wrapping_paper(&self) -> u64 {
        let side_areas = [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        let smallest_side_area = side_areas.iter().min().unwrap();
        self.surface_area() + smallest_side_area
    }

    fn volume(&self) -> u64 {
        self.length * self.width * self.height
    }

    fn required_ribbon(&self) -> u64 {
        let perimeters = [
            2 * self.length + 2 * self.width,
            2 * self.width + 2 * self.height,
            2 * self.height + 2 * self.length,
        ];
        let smallest_perimeter = perimeters.iter().min().unwrap();
        smallest_perimeter + self.volume()
    }
}

impl FromStr for GiftBox {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [length, width, height] = s.split('x').collect::<Vec<_>>()[..] {
            let length = length.parse()?;
            let width = width.parse()?;
            let height = height.parse()?;
            return Ok(GiftBox {
                length,
                width,
                height,
            });
        }
        Err(anyhow!("Invalid input: {}", s))
    }
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let giftboxes = input
        .lines()
        .map(GiftBox::from_str)
        .collect::<anyhow::Result<Vec<_>>>()?;
    let wrapping_paper: u64 = giftboxes.iter().map(GiftBox::required_wrapping_paper).sum();
    let ribbon: u64 = giftboxes.iter().map(GiftBox::required_ribbon).sum();
    println!("Square feet of wrapping paper: {}", wrapping_paper);
    println!("Feet of ribbon: {}", ribbon);
    Ok(())
}
