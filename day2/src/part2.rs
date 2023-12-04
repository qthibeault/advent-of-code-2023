use crate::game::{Colors, Game};

fn min_possible(game: &Game) -> Colors {
    let mut colors = Colors::default();

    for draw in &game.draws {
        colors.n_red = u32::max(colors.n_red, draw.n_red);
        colors.n_green = u32::max(colors.n_green, draw.n_green);
        colors.n_blue = u32::max(colors.n_blue, draw.n_blue);
    }

    colors
}

pub fn power_sum(games: &[Game]) -> u32 {
    let mut sum: u32 = 0;

    for game in games {
        let min_set = min_possible(game);
        let set_power = min_set.power();
        sum += set_power;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::game::parse_games;
    use super::{Colors, min_possible, power_sum};

    static TEST_LINES: &str =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_min_possible() {
        let expected = vec![
            Colors::new(4, 2, 6),
            Colors::new(1, 3, 4),
            Colors::new(20, 13, 6),
            Colors::new(14, 3, 15),
            Colors::new(6, 3, 2)
        ];

        let actual: Vec<Colors> = parse_games(TEST_LINES)
            .iter()
            .map(min_possible)
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_power_sum() {
        assert_eq!(power_sum(&parse_games(TEST_LINES)), 2286);
    }
}
