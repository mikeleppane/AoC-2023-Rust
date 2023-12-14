use std::path::Path;

use aoc_2023_rust::{output, read_lines, read_lines_from_string, OutputStatus, Runner};
use itertools::Itertools;

//const INPUT: &str = "input/day12.txt";
const TEST_INPUT: &str = "input/day12-test.txt";

#[derive(Debug, Default)]
pub struct Day12 {
    springs: Vec<Spring>,
}

impl Day12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day12 {
    fn name(&self) -> (usize, usize) {
        (2023, 12)
    }

    fn parse(&mut self, input: Option<&str>) {
        let puzzle_input: Vec<String>;
        if let Some(input) = input {
            if Path::new(input).is_file() {
                puzzle_input = read_lines(input);
            } else {
                puzzle_input = read_lines_from_string(input);
            }
        } else {
            puzzle_input = read_lines(TEST_INPUT);
        }

        for line in puzzle_input {
            self.springs.push(Spring::from(line.as_str()));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(OutputStatus::Unsolved)
    }

    fn part2(&mut self) -> Vec<String> {
        output(OutputStatus::Unsolved)
    }
}

// ---------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

impl Default for SpringCondition {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Default)]
struct Spring {
    record: SpringRecord,
    damaged_springs: Vec<u32>,
}

impl From<&str> for Spring {
    fn from(s: &str) -> Self {
        let mut spring = Spring::new();
        let (spring_record, damaged_springs) = s.split_once(' ').unwrap();
        spring.damaged_springs = damaged_springs
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec();
        for c in spring_record.chars() {
            let condition = match c {
                '.' => SpringCondition::Operational,
                '#' => SpringCondition::Damaged,
                '?' => SpringCondition::Unknown,
                _ => panic!("Invalid spring condition"),
            };
            spring.record.data.push(condition);
        }
        spring
    }
}

impl Spring {
    fn new() -> Self {
        Self {
            record: SpringRecord { data: vec![] },
            damaged_springs: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct SpringRecord {
    data: Vec<SpringCondition>,
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day12-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day12::Day12;

    #[test]
    fn part1_test_works() {
        let mut day = Day12::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "21")
    }

    #[test]
    fn part1_works() {
        let mut day = Day12::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "10154062")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day12::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "82000210")
    }

    #[test]
    fn part2_works() {
        let mut day = Day12::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "553083047914")
    }
}
