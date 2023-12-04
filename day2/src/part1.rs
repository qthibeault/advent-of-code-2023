use crate::game::{Colors, Game};

static COLOR_LIMITS: Colors = Colors {
    n_red: 12,
    n_green: 13,
    n_blue: 14
};

pub fn sum(games: &[Game]) -> u32 {
    let mut total: u32 = 0;

    for game in games {
        if game.is_possible(&COLOR_LIMITS) {
            total += game.id;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::sum;
    use crate::game::parse_games;

    #[test]
    fn test_sum() {
        let test_lines: &str =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = parse_games(test_lines);

        assert_eq!(sum(&games), 8);
    }
}
