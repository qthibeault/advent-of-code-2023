#[derive(Debug)]
pub struct Race {
    pub duration_ms: f64,
    pub record_mm: f64,
}

// distance = press_time * run_time
// run_time = time - press_time
// distance = press_time * (time - press_time)
// 0 = press_time * time - press_time * press_time - distance
// press_time = [-time +- sqrt(time^2 - 4 * -1 * -distance)] / 2 * -1

impl Race {
    pub fn n_better_times(&self) -> usize {
        let duration = self.duration_ms;
        let distance = self.record_mm;

        let det = f64::sqrt(duration.powi(2) - 4.0 * distance);
        let r1 = (duration - det) / 2.0;
        let r1 = (r1.floor() as usize) + 1;

        let r2: f64 = (duration + det) / 2.0;
        let r2 = (r2.ceil() as usize) - 1;

        r2 - r1 + 1
    }
}

fn parse_split_number(line: &str) -> f64 {
    let mut combined = String::new();

    for s in line.split_whitespace().skip(1) {
        combined.push_str(s);
    }

    combined.parse().expect("Could not parse time")
}

pub fn parse_big_race(input: &str) -> Race {
    let lines: Vec<&str> = input.lines().collect();
    
    assert_eq!(lines.len(), 2);

    let time = parse_split_number(lines[0]);
    let distance = parse_split_number(lines[1]);

    Race {
        duration_ms: time,
        record_mm: distance
    }
}

pub fn parse_races(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    
    assert_eq!(lines.len(), 2);

    let times_ms: Vec<f64> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("Could not parse number"))
        .collect();

    let distances_mm: Vec<f64> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("Could not parse number"))
        .collect();

    assert_eq!(times_ms.len(), distances_mm.len());

    times_ms
        .into_iter()
        .zip(distances_mm)
        .map(|(duration_ms, record_mm)| Race { duration_ms, record_mm })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::Race;

    #[test]
    fn n_better_times() {
        let race = Race { duration_ms: 30f64, record_mm: 200f64 };
        assert_eq!(race.n_better_times(), 9);

        let race = Race { duration_ms: 7f64, record_mm: 9f64 };
        assert_eq!(race.n_better_times(), 4);

        let race = Race { duration_ms: 15f64, record_mm: 40f64 };
        assert_eq!(race.n_better_times(), 8);
    }
}
