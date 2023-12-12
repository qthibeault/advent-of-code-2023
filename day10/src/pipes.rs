use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    row: i64,
    col: i64,
}

impl Position {
    fn up(&self) -> Self {
        Position {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn down(&self) -> Self {
        Position {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn left(&self) -> Self {
        Position {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn right(&self) -> Self {
        Position {
            row: self.row,
            col: self.col + 1,
        }
    }
}

struct Pipe {
    position: Position,
    ends: HashSet<Position>,
}

/// Map of the pipes on Metal Island
///
/// start is the position the animal enters the pipes.
/// ends is a mapping between a position and the ends of the pipe
/// connections is a mapping that represents the set of reachable positions from a given position
///
/// Given a map like so:
///
///        12345
///        .....1
///        .S-7.2
///        .|.|.3
///        .L-J.4
///        .....5
///
/// When starting at the S at (2,2) the reachable positions are { (2,3), (3,2) }. We can then
/// compute the loop by expanding the reachable positions in either direction until we find
/// encounter repeat positions.
#[derive(Debug, PartialEq, Eq)]
pub struct Pipes<'a> { 
    start: Position,
    connections: HashMap<Position, HashSet<Position>>,  // positions touching an end
    map: &'a str
}

impl<'a> From<&'a str> for Pipes<'a> {
    fn from(input: &'a str) -> Self {
        let mut start: Option<Position> = None;
        let map_line = |(row, line): (usize, &str)| -> std::vec::IntoIter<Pipe> {
            let mut pipes = Vec::new();

            for (col, c) in line.trim().chars().enumerate() {
                let pos = Position {
                    row: row.try_into().expect("Could not convert row to i64"),
                    col: col.try_into().expect("Could not convert col to i64"),
                };

                match c {
                    '|' => {
                        let pipe = Pipe {
                            ends: [pos.up(), pos.down()].into(),
                            position: pos,
                        };

                        pipes.push(pipe);
                    },
                    '-' => {
                        let pipe = Pipe {
                            ends: [pos.left(), pos.right()].into(),
                            position: pos,
                        };

                        pipes.push(pipe);
                    },
                    'F' => {
                        let pipe = Pipe {
                            ends: [pos.right(), pos.down()].into(),
                            position: pos,
                        };

                        pipes.push(pipe);
                    },
                    '7' => {
                        let pipe = Pipe {
                            ends: [pos.left(), pos.down()].into(),
                            position: pos,
                        };

                        pipes.push(pipe);
                    },
                    'L' => {
                        let pipe = Pipe {
                            ends: [pos.up(), pos.right()].into(),
                            position: pos,
                        };

                        pipes.push(pipe);
                    },
                    'J' => {
                        let pipe = Pipe {
                            ends: [pos.up(), pos.left()].into(),
                            position: pos,
                        };

                        pipes.push(pipe);
                    },
                    'S' => {
                        start = Some(pos);
                    },
                    _ => {},
                };


            }

            pipes.into_iter()
        };

        let pipes: Vec<_> = input
            .trim()
            .lines()
            .enumerate()
            .map(map_line)
            .flatten()
            .collect();

        let mut connections: HashMap<_, _> = pipes
            .iter()
            .map(|pipe| {
                let connections: HashSet<_> = pipes
                    .iter()
                    .filter_map(|other| {
                        if pipe.ends.contains(&other.position) && other.ends.contains(&pipe.position) {
                            Some(other.position)
                        } else {
                            None
                        }
                    })
                    .collect();

                (pipe.position, connections)
            })
            .collect();

        let start = start.expect("No start position detected");
        let start_connections = pipes
            .iter()
            .filter_map(|pipe| {
                if pipe.ends.contains(&start) {
                    Some(pipe.position)
                } else {
                    None
                }
            })
            .collect();

        connections.insert(start, start_connections);

        Self { start, connections, map: input }
    }
}

#[derive(Clone)]
struct Path {
    position: Position,
    seen: HashSet<Position>,
}

impl Path {
    fn start(p: Position) -> Self {
        Self {
            position: p.clone(),
            seen: HashSet::new(),
        }
    }

    fn step(&self, p: Position) -> Self {
        let mut seen = self.seen.clone();
        seen.insert(self.position);

        Self {
            position: p,
            seen,
        }
    }
}

impl<'a> Pipes<'a> {
    fn find_loop(&self) -> HashSet<Position> {
        let mut loop_positions = HashSet::new();
        loop_positions.insert(self.start);

        let mut position = self.connections[&self.start]
            .iter()
            .copied()
            .next()
            .expect("No position connects to start");

        while position != self.start {
            let next_position = self.connections[&position]
                .iter()
                .filter(|&p| !loop_positions.contains(p))
                .copied()
                .next();

            match next_position {
                Some(p) => {
                    loop_positions.insert(position);
                    position = p;
                },
                None => {
                    if self.connections[&self.start].contains(&position) {
                        loop_positions.insert(position);
                        position = self.start;
                    } else {
                        panic!("No next position found for {:?}", position);
                    }
                }
            }

        }

        loop_positions
    }

    pub fn max_steps(&self) -> usize {
        self.find_loop().len() / 2
    }

    pub fn contained_tiles(&self) -> HashSet<Position> {
        let loop_ = self.find_loop();
        let mut contained = HashSet::new();

        for (row, line) in self.map.trim().lines().enumerate() {
            let mut in_region = false;
            let mut last_corner = ' ';

            for (col, mut c) in line.trim().chars().enumerate() {
                let p = Position {
                    row: row.try_into().expect("Could not convert row to i64"),
                    col: col.try_into().expect("Could not convert col to i64"),
                };

                if "|-S7JFL".contains(c) && !loop_.contains(&p) {
                    continue;
                }

                if c == 'S' {
                    let connections = &self.connections[&p];
                    
                    if *connections == HashSet::from([p.left(), p.right()]) {
                        c = '-';
                    }

                    if *connections == HashSet::from([p.up(), p.down()]) {
                        c = '|';
                    }

                    if *connections == HashSet::from([p.right(), p.down()]) {
                        c = 'F';
                    }

                    if *connections == HashSet::from([p.left(), p.down()]) {
                        c = '7';
                    }

                    if *connections == HashSet::from([p.left(), p.up()]) {
                        c = 'J';
                    }

                    if *connections == HashSet::from([p.right(), p.up()]) {
                        c = 'L';
                    }
                }

                match c {
                    '|' => {
                        in_region = !in_region;
                    },
                    '-' => {},
                    'S' => {},
                    '7' => {
                        if last_corner == 'L' {
                            in_region = !in_region;
                        }

                        last_corner = '7';
                    },
                    'J' => {
                        if last_corner == 'F' {
                            in_region = !in_region;
                        }

                        last_corner = 'J';
                    },
                    'F' => {
                        last_corner = 'F';
                    },
                    'L' => {
                        last_corner = 'L';
                    }
                    _ => {
                        if in_region {
                            contained.insert(p);
                        }
                    }
                }
            }
        }

        contained
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use indoc::indoc;

    use super::{Pipes, Position};

    fn pipes() -> Pipes<'static> {
        let start = Position { row: 1, col: 1 };
        let connections: HashMap<Position, HashSet<Position>> = HashMap::from([
            (Position { row: 1, col: 1 }, [Position { row: 1, col: 2 }, Position { row: 2, col: 1 }].into()),
            (Position { row: 1, col: 2 }, [Position { row: 1, col: 3 }].into()),
            (Position { row: 1, col: 3 }, [Position { row: 1, col: 2 }, Position { row: 2, col: 3 }].into()),
            (Position { row: 2, col: 1 }, [Position { row: 3, col: 1 }].into()),
            (Position { row: 2, col: 3 }, [Position { row: 1, col: 3 }, Position { row: 3, col: 3 }].into()),
            (Position { row: 3, col: 1 }, [Position { row: 2, col: 1 }, Position { row: 3, col: 2 }].into()),
            (Position { row: 3, col: 2 }, [Position { row: 3, col: 1 }, Position { row: 3, col: 3 }].into()),
            (Position { row: 3, col: 3 }, [Position { row: 3, col: 2 }, Position { row: 2, col: 3 }].into()),
        ]);

        let map = indoc!{"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "};

        Pipes { start, connections, map }
    }

    #[test]
    fn parsing() {
        let input = indoc!{"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "};

        let parsed = Pipes::from(input);
        let expected = pipes();

        assert_eq!(parsed.start, expected.start);

        for (pos, expected_connections) in expected.connections {
            assert_eq!(parsed.connections[&pos], expected_connections, "Position {:?} does not match", pos);
        }
    }

    #[test]
    fn find_loop() {
        let expected = HashSet::from([
            Position { row: 1, col: 1 },
            Position { row: 1, col: 2 },
            Position { row: 1, col: 3 },
            Position { row: 2, col: 1 },
            Position { row: 2, col: 3 },
            Position { row: 3, col: 1 },
            Position { row: 3, col: 2 },
            Position { row: 3, col: 3 },
        ]);

        assert_eq!(pipes().find_loop(), expected);
    }

    #[test]
    fn max_steps() {
        assert_eq!(pipes().max_steps(), 4);
        
        let input = indoc!{"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        "};

        assert_eq!(Pipes::from(input).max_steps(), 4);

        let input = indoc!{"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};

        assert_eq!(Pipes::from(input).max_steps(), 8);

        let input = indoc!{"
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
        "};

        assert_eq!(Pipes::from(input).max_steps(), 8);
    }

    #[test]
    fn contained_tiles() {
        assert_eq!(pipes().contained_tiles().len(), 1, "pipes");

        let input = indoc!{"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "};

        let expected = HashSet::from([
            Position { row: 6, col: 2 },
            Position { row: 6, col: 3 },
            Position { row: 6, col: 7 },
            Position { row: 6, col: 8 },
        ]);

        assert_eq!(Pipes::from(input).contained_tiles(), expected, "input1");

        let input = indoc!{"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};

        let expected = HashSet::from([
            Position { row: 3, col: 14 },
            Position { row: 4, col: 7 },
            Position { row: 4, col: 8 },
            Position { row: 4, col: 9 },
            Position { row: 5, col: 7 },
            Position { row: 5, col: 8 },
            Position { row: 6, col: 6 },
            Position { row: 6, col: 14 },
        ]);

        assert_eq!(Pipes::from(input).contained_tiles(), expected, "input2");

        let input = indoc!{"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJIF7FJ-
            L---JF-JLJIIIIFJLJJ7
            |F|F-JF---7IIIL7L|7|
            |FFJF7L7F-JF7IIL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};

        let expected = HashSet::from([
            Position { row: 3, col: 14 },
            Position { row: 4, col: 10 },
            Position { row: 4, col: 11 },
            Position { row: 4, col: 12 },
            Position { row: 4, col: 13 },
            Position { row: 5, col: 11 },
            Position { row: 5, col: 12 },
            Position { row: 5, col: 13 },
            Position { row: 6, col: 13 },
            Position { row: 6, col: 14 },
        ]);

        // {
        //   Position { row: 3, col: 14 }
        //   Position { row: 4, col: 10 },
        //   Position { row: 4, col: 11 },
        //   Position { row: 4, col: 12 },
        //   Position { row: 4, col: 13 },
        //   Position { row: 5, col: 12 },
        //   Position { row: 5, col: 13 },
        //   Position { row: 5, col: 11 },
        //   Position { row: 9, col: 18 },
        // }

        assert_eq!(Pipes::from(input).contained_tiles(), expected, "input3");
    }
}
