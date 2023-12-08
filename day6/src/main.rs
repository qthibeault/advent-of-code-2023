mod race;

use indoc::indoc;

use race::{Race, parse_races, parse_big_race};

fn part1(races: &[Race]) -> usize {
    races
        .iter()
        .map(|r| r.n_better_times())
        .product()
}

fn main() {
    let input: &str = indoc!{"
        Time:        42     68     69     85
        Distance:   284   1005   1122   1341
    "};

    let races = parse_races(input);
    let p1 = part1(&races);

    let race = parse_big_race(input);
    let p2 = race.n_better_times();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
