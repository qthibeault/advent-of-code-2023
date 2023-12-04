fn parse_digit(line: &str) -> Option<u32> {
    match line.as_bytes() {
        [b'o', b'n', b'e', ..] | [b'1', ..] => Some(1),
        [b't', b'w', b'o', ..] | [b'2', ..] => Some(2),
        [b't', b'h', b'r', b'e', b'e', ..] | [b'3', ..] => Some(3),
        [b'f', b'o', b'u', b'r', ..] | [b'4', ..] => Some(4),
        [b'f', b'i', b'v', b'e', ..] | [b'5', ..] => Some(5),
        [b's', b'i', b'x', ..] | [b'6', ..] => Some(6),
        [b's', b'e', b'v', b'e', b'n', ..] | [b'7', ..] => Some(7),
        [b'e', b'i', b'g', b'h', b't', ..] | [b'8', ..] => Some(8),
        [b'n', b'i', b'n', b'e', ..] | [b'9', ..] => Some(9),
        _ => None
    }
}

fn parse_value(line: &str) -> u32 {
    let trimmed = line.trim();
    let mut start_idx = 0;

    let d1: u32 = loop {
        if start_idx >= trimmed.len() {
            panic!("No start value found");
        }

        let substr = trimmed.get(start_idx..).unwrap();
        if let Some(value) = parse_digit(substr) {
            break value;
        } else {
            start_idx += 1;
        }
    };

    let mut d2: u32 = d1;

    while start_idx < trimmed.len() {
        let substr = trimmed.get(start_idx..).unwrap();
        if let Some(value) = parse_digit(substr) {
            d2 = value;
        }

        start_idx += 1;
    }

    d1 * 10 + d2
}

pub fn sum(doc: &str) -> u32 {
    doc.lines().map(parse_value).sum()
}

#[cfg(test)]
mod tests {
    use super::sum;

    static TEST_LINES: &str = 
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

    #[test]
    fn test() {
        assert_eq!(sum(TEST_LINES), 281);
    }
}
