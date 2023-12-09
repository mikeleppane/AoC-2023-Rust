use aoc_2023_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day09.txt";

#[derive(Debug, Default)]
pub struct Day09 {
    dataset: Vec<History>,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day09 {
    fn name(&self) -> (usize, usize) {
        (2023, 7)
    }

    fn parse(&mut self, input: Option<&str>) {
        let input = read_lines(input.unwrap_or(INPUT));
        for line in input {
            let line = line.split(' ').collect::<Vec<&str>>();
            let history: History = History::new(
                line.iter()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
            self.dataset.push(history);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.dataset
                .iter()
                .map(|h| h.calculate_extrapolated_value())
                .sum::<i32>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        output(
            self.dataset
                .iter()
                .map(|h| h.calculate_extrapolated_value_backwards())
                .sum::<i32>(),
        )
    }
}

// ---------------------------------------------------

#[derive(Debug)]
struct History {
    data: Vec<i32>,
}

impl History {
    fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    fn calculate_extrapolated_value(&self) -> i32 {
        let mut sequences = vec![self.data.clone()];
        loop {
            let mut next_sequence = Vec::new();
            for sequence in sequences.last().unwrap().windows(2) {
                next_sequence.push(sequence[1] - sequence[0]);
            }
            if next_sequence.iter().all(|v| *v == 0) {
                sequences.push(next_sequence);
                break;
            }
            sequences.push(next_sequence);
        }

        let mut next_value = 0;
        for sequence in sequences.iter().rev().skip(1) {
            next_value += sequence[sequence.len() - 1];
        }

        next_value
    }

    fn calculate_extrapolated_value_backwards(&self) -> i32 {
        let mut sequences = vec![self.data.clone()];
        loop {
            let mut next_sequence = Vec::new();
            for sequence in sequences.last().unwrap().windows(2) {
                next_sequence.push(sequence[1] - sequence[0]);
            }
            if next_sequence.iter().all(|v| *v == 0) {
                sequences.push(next_sequence);
                break;
            }
            sequences.push(next_sequence);
        }

        let mut next_value = 0;
        for sequence in sequences.iter().rev().skip(1) {
            next_value = sequence[0] - next_value;
        }
        next_value
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day09-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day09::Day09;

    #[test]
    fn part1_test_works() {
        let mut day = Day09::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "114")
    }

    #[test]
    fn part1_works() {
        let mut day = Day09::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "1992273652")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day09::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "2")
    }

    #[test]
    fn part2_works() {
        let mut day = Day09::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "1012")
    }
}
