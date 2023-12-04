use std::ops::{Add, AddAssign};

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::{preceded, pair, delimited};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Colors {
    pub n_red: u32,
    pub n_green: u32,
    pub n_blue: u32,
}

impl Colors {
    fn red(n: u32) -> Self {
        Self { n_red: n, n_green: 0, n_blue: 0 }
    }

    fn green(n: u32) -> Self {
        Self { n_red: 0, n_green: n, n_blue: 0 }
    }

    fn blue(n: u32) -> Self {
        Self { n_red: 0, n_green: 0, n_blue: n }
    }
}

impl AddAssign for Colors {
    fn add_assign(&mut self, rhs: Colors) {
        self.n_red += rhs.n_red;
        self.n_green += rhs.n_green;
        self.n_blue += rhs.n_blue;
    }
}

impl Add for Colors {
    type Output = Self;

    fn add(mut self, rhs: Self) ->  Self::Output {
        self += rhs;
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    draws: Vec<Colors>,
}

impl Game {
    fn new<I>(id: u32, d: Colors, ds: I) -> Self
    where
        I: IntoIterator<Item = Colors>
    {
        let mut draws = Vec::new();
        draws.push(d);
        draws.extend(ds);

        Self { id, draws }
    }

    fn is_possible(&self, amounts: &Colors) -> bool {
        self.draws.iter().all(|draw| {
            draw.n_red <= amounts.n_red && draw.n_green <= amounts.n_green && draw.n_blue <= amounts.n_blue
        })
    }
}

fn parse_color(input: &str) -> IResult<&str, Colors> {
    let (input, value) = nom::character::complete::u32(input)?;
    let (input, _) = space1(input)?;

    let color_tags = (
        map(tag("red"), move |_| Colors::red(value)),
        map(tag("green"), move |_| Colors::green(value)),
        map(tag("blue"), move |_| Colors::blue(value)),
    );

    alt(color_tags)(input)
}

fn parse_draw(input: &str) -> IResult<&str, Colors> {
    let separator = pair(tag(","), space1);
    let (input, mut draw) = parse_color(input)?;
    let (rest, draws) = many0(preceded(separator, parse_draw))(input)?;

    for d in draws {
        draw += d;
    }

    Ok((rest, draw))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game"), preceded(space1, nom::character::complete::u32), tag(":"))(input)?;
    let (input, draw) = preceded(space1, parse_draw)(input)?;
    let (rest, draws) = many0(preceded(tag(";"), preceded(space1, parse_draw)))(input)?;
    let game = Game::new(id, draw, draws);

    Ok((rest, game))
}

static COLOR_LIMITS: Colors = Colors {
    n_red: 12,
    n_green: 13,
    n_blue: 14
};

pub fn sum(doc: &str) -> u32 {
    let mut total: u32 = 0;

    for line in doc.lines() {
        let (_, game) = parse_game(line.trim()).expect("Could not parse line");

        if game.is_possible(&COLOR_LIMITS) {
            println!("{:#?}", game);
            total += game.id;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::{Colors, Game, parse_game, sum};

    #[test]
    fn test_parse_game() -> Result<(), Box<dyn std::error::Error>> {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (_, parsed) = parse_game(line)?;
        let expected = Game {
            id: 1,
            draws: vec![
                Colors { n_red: 4, n_green: 0, n_blue: 3 },
                Colors { n_red: 1, n_green: 2, n_blue: 6 },
                Colors { n_red: 0, n_green: 2, n_blue: 0 },
            ],
        };

        assert_eq!(parsed, expected);
        Ok(())
    }

    #[test]
    fn test_sum() {
        let test_lines: &str =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(sum(test_lines), 8);
    }
}
