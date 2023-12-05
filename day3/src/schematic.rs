use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Symbol {
    value: char,
    pos: Position,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Number<'a> {
    value: &'a str,
    row: usize,
    start_col: usize,
}

impl<'a> Number<'a> {
    fn adjacent_spaces(&self) -> HashSet<Position> {
        let mut spaces: HashSet<Position> = HashSet::new();

        if self.start_col > 0 {
            let x = self.start_col - 1;

            if self.row > 0 {
                spaces.insert(Position { x, y: self.row - 1 });
            }

            spaces.insert(Position { x, y: self.row });
            spaces.insert(Position { x, y: self.row + 1 });
        }

        for i in 0..self.value.len() {
            let x = self.start_col + i; 

            if self.row > 0 {
                spaces.insert(Position { x, y: self.row - 1 });
            }

            spaces.insert(Position { x, y: self.row + 1 });
        }

        {
            let x = self.start_col + self.value.len();
            
            if self.row > 0 {
                spaces.insert(Position { x, y: self.row - 1});
            }

            spaces.insert(Position { x, y: self.row });
            spaces.insert(Position { x, y: self.row + 1 });
        }

        spaces
    }
}

#[derive(Debug)]
pub struct Schematic<'a> {
    numbers: HashSet<Number<'a>>,
    symbols: HashSet<Symbol>,
}

impl<'a> Schematic<'a> {
    pub fn from_str(input: &'a str) -> Self {
        let mut numbers = HashSet::new();
        let mut symbols = HashSet::new();

        for (nrow, line) in input.lines().enumerate() {
            let line = line.trim();
            let mut start: Option<usize> = None;

            for (ncol, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    if start.is_none() {
                        start = Some(ncol);
                    }
                } else {
                    if let Some(s) = start {
                        let substr = line.get(s..ncol).expect("Could not extract number substring");
                        let n = Number {
                            value: substr,
                            row: nrow,
                            start_col: s,
                        };
                        
                        numbers.insert(n);
                        start = None;
                    }

                    if c != '.' {
                        let s = Symbol {
                            value: c,
                            pos: Position {x: ncol, y: nrow },
                        };

                        symbols.insert(s);
                    }
                }
            }

            if let Some(s) = start {
                let substr = line.get(s..).expect("Could not extract number substring");
                let n = Number {
                    value: substr,
                    row: nrow,
                    start_col: s,
                };

                numbers.insert(n);
            }
        }

        Self { numbers, symbols }
    }

    pub fn part_numbers(&self) -> Vec<usize> {
        self.numbers
            .iter()
            .filter(|num| {
                let spaces = num.adjacent_spaces();
                self.symbols.iter().any(|sym| spaces.contains(&sym.pos))
            })
            .map(|num| usize::from_str(num.value).expect("Could not parse part number"))
            .collect()
    }

    pub fn symbols(&self) -> HashSet<char> {
        self.symbols.iter().map(|sym| sym.value).collect()
    }
}

impl Default for Schematic<'static> {
    fn default() -> Self {
        let schematic_str =
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..";

        Schematic::from_str(schematic_str)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{Number, Position, Schematic};

    #[test]
    fn test_adjacent_spaces() {
        let number = Number { value: "467", row: 0, start_col: 0 };
        let expected = HashSet::from([
            Position { x: 0, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 1 },
            Position { x: 3, y: 1 },
            Position { x: 3, y: 0 },
        ]);

        assert_eq!(number.adjacent_spaces(), expected);

        let number = Number { value: "663", row: 2, start_col: 6 };
        let expected = HashSet::from([
            Position { x: 5, y: 1 },
            Position { x: 5, y: 2 },
            Position { x: 5, y: 3 },
            Position { x: 6, y: 1 },
            Position { x: 6, y: 3 },
            Position { x: 7, y: 1 },
            Position { x: 7, y: 3 },
            Position { x: 8, y: 1 },
            Position { x: 8, y: 3 },
            Position { x: 9, y: 1 },
            Position { x: 9, y: 2 },
            Position { x: 9, y: 3 },
        ]);

        assert_eq!(number.adjacent_spaces(), expected);
    }

    #[test]
    fn test_part_numbers() {
        let schematic = Schematic::default();
        let part_numbers = schematic.part_numbers();
        let expected: Vec<usize> = vec![467, 35, 633, 617, 592, 755, 664, 598];

        assert_eq!(part_numbers, expected);
    }
}
