#[derive(Debug, PartialEq, Eq)]
pub struct Report {
    dataset: Vec<History>,
}

impl Report {
    pub fn extrapolate_end(&self) -> impl Iterator<Item = i64> + '_ {
        self.dataset
            .iter()
            .map(|history| history.extrapolate_end())
    }
}

impl From<&str> for Report {
    fn from(input: &str) -> Self {

        Self {
            dataset: input
                .trim()
                .lines()
                .map(|line| History::from(line))
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct History {
    values: Vec<i64>,
}

fn differences(values: &[i64]) -> Vec<i64> {
    let mut iter = values.iter().peekable();
    let mut result = Vec::new();

    while let Some(&current) = iter.next() {
        if let Some(&&next) = iter.peek() {
            result.push(next - current);
        }
    }

    result
}

impl History {
    fn new<I>(values: I) -> Self
    where
        I: IntoIterator<Item = i64>
    {
        Self {
            values: Vec::from_iter(values),
        }
    }

    fn extrapolate_end(&self) -> i64 {
        let mut diff_seq = differences(&self.values);
        let mut ends = Vec::new();

        ends.push(self.values.last().copied().unwrap());

        while !diff_seq.iter().all(|d| *d == 0) {
            ends.push(*diff_seq.last().unwrap());
            diff_seq = differences(&diff_seq);
        }

        ends.into_iter().sum()
    }
}

impl From<&str> for History {
    fn from(input: &str) -> Self {
        Self {
            values: input
                .trim()
                .split(' ')
                .map(|part| part.parse().unwrap())
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::{History, Report}; 

    #[test]
    fn extrapolate() {
        let s1 = History::new([0, 3, 6, 9, 12, 15]);
        let s2 = History::new([1, 3, 6, 10, 15, 21]);
        let s3 = History::new([10, 13, 16, 21, 30, 45]);

        assert_eq!(s1.extrapolate_end(), 18);
        assert_eq!(s2.extrapolate_end(), 28);
        assert_eq!(s3.extrapolate_end(), 68);
    }

    #[test]
    fn parsing() {
        let input = indoc!{"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};

        let expected = Report {
            dataset: vec![
                History::new([0, 3, 6, 9, 12, 15]),
                History::new([1, 3, 6, 10, 15, 21]),
                History::new([10, 13, 16, 21, 30, 45]),
            ],
        };

        assert_eq!(Report::from(input), expected);
    }
}
