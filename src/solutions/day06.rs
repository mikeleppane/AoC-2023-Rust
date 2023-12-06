use std::iter::zip;

use aoc_2023_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day06.txt";

#[derive(Debug, Default)]
pub struct Day06 {
    races: Vec<Race>,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day06 {
    fn name(&self) -> (usize, usize) {
        (2023, 6)
    }

    fn parse(&mut self, input: Option<&str>) {
        let input = read_lines(input.unwrap_or(INPUT));
        let mut times: Vec<u64> = Vec::new();
        let mut distances: Vec<u64> = Vec::new();
        for line in input {
            if line.starts_with("Time:") {
                let (_, timings) = line.split_once(':').unwrap();
                timings.split_whitespace().for_each(|t| {
                    times.push(t.trim().parse().unwrap());
                });
            }
            if line.starts_with("Distance:") {
                let (_, dist) = line.split_once(':').unwrap();
                dist.split_whitespace().for_each(|t| {
                    distances.push(t.trim().parse().unwrap());
                });
            }
        }
        for (time, distance) in zip(times, distances) {
            self.races.push(Race::new(time, distance))
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.races
                .iter()
                .map(|r| r.beat_record())
                .collect::<Vec<u64>>()
                .into_iter()
                .reduce(|a, b| a * b)
                .unwrap(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let time: u64 = self
            .races
            .iter()
            .map(|r| r.time)
            .fold("".to_string(), |acc, x| acc + &x.to_string())
            .parse()
            .unwrap();
        let distance: u64 = self
            .races
            .iter()
            .map(|r| r.distance)
            .fold("".to_string(), |acc, x| acc + &x.to_string())
            .parse()
            .unwrap();
        let race = Race::new(time, distance);

        output(race.beat_record())
    }
}

// ---------------------------------------------------

#[derive(Debug, Default)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn beat_record(&self) -> u64 {
        let mut beats = 0;
        for speed in 0..=self.time {
            let time = self.time - speed;
            let distance = time * speed;
            if distance > self.distance {
                beats += 1;
            }
        }
        beats
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day06-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day06::Day06;

    #[test]
    fn part1_test_works() {
        let mut day = Day06::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "288")
    }

    #[test]
    fn part1_works() {
        let mut day = Day06::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "449550")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day06::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "71503")
    }

    #[test]
    fn part2_works() {
        let mut day = Day06::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "28360140")
    }
}
