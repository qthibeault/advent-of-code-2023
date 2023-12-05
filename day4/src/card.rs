use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::multi::many1;
use nom::sequence::{delimited, pair, preceded};

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    id: u32,
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl Card {
    pub fn points(&self) -> u32 {
        let n_winners: u32 = self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
            .try_into()
            .expect("Count could not be converted into a u32");

        match n_winners {
            0 => 0,
            n => u32::pow(2, n - 1)
        }
    }

    pub fn from_str<'a>(input: &'a str) -> Result<Self, nom::Err<nom::error::Error<&'a str>>> {
        let input = input.trim();
        let (input, id) = delimited(tag("Card"), preceded(space1, nom::character::complete::u32), tag(":"))(input)?;
        let (input, winning_numbers) = many1(preceded(space1, nom::character::complete::u32))(input)?;
        let (input, _) = pair(space1, tag("|"))(input)?;
        let (input, numbers) = many1(preceded(space1, nom::character::complete::u32))(input)?;
        let (_, _) = eof(input)?;

        Ok(Card { id, numbers, winning_numbers })
    }
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn from_str() {
        let line = "Card   1: 24 76 32 40 51 61 89  6 30 60 | 30 69 24 86  6  8 92 61 51 88 63 67 32 62 15 49 22 77 40 27 89 60 76 58 79";
        let expected = Card {
            id: 1,
            numbers: vec![30, 69, 24, 86, 6, 8, 92, 61, 51, 88, 63, 67, 32, 62, 15, 49, 22, 77, 40, 27, 89, 60, 76, 58, 79],
            winning_numbers: vec![24, 76, 32, 40, 51, 61, 89, 6, 30, 60],
        };

        assert_eq!(Ok(expected), Card::from_str(line));

        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let expected = Card {
            id: 2,
            numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            winning_numbers: vec![13, 32, 20, 16, 61],
        };

        assert_eq!(Ok(expected), Card::from_str(line));
    }

    #[test]
    fn points() {
        let c1 = Card {
            id: 1,
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            winning_numbers: vec![41, 48, 83, 86, 17],
        };

        let c2 = Card {
            id: 2,
            numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            winning_numbers: vec![13, 32, 20, 16, 61],
        };

        let c3 = Card {
            id: 4,
            numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
            winning_numbers: vec![41, 92, 73, 84, 69],
        };

        assert_eq!(c1.points(), 8);
        assert_eq!(c2.points(), 2);
        assert_eq!(c3.points(), 1);
    }
}
