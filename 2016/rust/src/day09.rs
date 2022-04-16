fn parse_marker(marker: &str) -> (usize, usize) {
    let (length, times) = marker.split_once('x').unwrap();
    (length.parse().unwrap(), times.parse().unwrap())
}

fn decompress_v1(compressed: &str) -> String {
    let mut decompressed = String::new();
    let mut chars = compressed.chars();
    while let Some(c) = chars.next() {
        if c == '(' {
            let marker: String = chars.by_ref().take_while(|c| *c != ')').collect();
            let (length, times) = parse_marker(marker.as_str());
            let data: String = chars.by_ref().take(length).collect();
            for _ in 0..times {
                decompressed.push_str(data.as_str());
            }
        } else {
            decompressed.push(c);
        }
    }
    decompressed
}

fn decompress_v2(compressed: &str) -> String {
    let mut decompressed = String::new();
    let mut chars = compressed.chars();
    while let Some(c) = chars.next() {
        if c == '(' {
            let marker: String = chars.by_ref().take_while(|c| *c != ')').collect();
            let (length, times) = parse_marker(marker.as_str());
            let data: String = chars.by_ref().take(length).collect();
            let decompressed_data = decompress_v2(data.as_str());
            for _ in 0..times {
                decompressed.push_str(decompressed_data.as_str());
            }
        } else {
            decompressed.push(c);
        }
    }
    decompressed
}

pub fn main(input: &str) -> anyhow::Result<()> {
    println!(
        "Decompressed (version 1) length: {}",
        decompress_v1(input.trim()).chars().count()
    );

    println!(
        "Decompressed (version 2) length: {}",
        decompress_v2(input.trim()).chars().count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_v1() {
        assert_eq!(decompress_v1("ADVENT"), "ADVENT");
        assert_eq!(decompress_v1("ADVENT"), "ADVENT");
        assert_eq!(decompress_v1("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress_v1("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress_v1("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress_v1("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress_v1("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn test_decompress_v2() {
        assert_eq!(decompress_v2("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress_v2("X(8x2)(3x3)ABCY"), "XABCABCABCABCABCABCY");
        assert_eq!(
            decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
            std::iter::repeat('A').take(241920).collect::<String>()
        );
        assert_eq!(
            decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
                .chars()
                .count(),
            445
        );
    }
}
