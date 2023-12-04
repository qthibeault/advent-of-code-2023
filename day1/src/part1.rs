fn parse_digit<Chars>(cs: Chars) -> u32
where
    Chars: Iterator<Item = char>
{
    for c in cs {
        if c.is_numeric() {
            return c.to_digit(10).expect("Could not parse digit");
        }
    }

    panic!("No digit found")
}

fn parse_value(line: &str) -> u32 {
    let trimmed = line.trim();
    let d1 = parse_digit(trimmed.chars());
    let d2 = parse_digit(trimmed.chars().rev());

    d1 * 10 + d2
}

pub fn sum(doc: &str) -> u32 {
    doc.lines().map(parse_value).sum()
}

#[cfg(test)]
mod tests {
    use super::sum;

    static TEST_LINES: &str =
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    #[test]
    fn part1() {
        assert_eq!(sum(TEST_LINES), 142);
    }
}
