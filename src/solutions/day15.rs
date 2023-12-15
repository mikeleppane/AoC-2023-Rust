use std::{
    collections::{HashMap, VecDeque},
    path::Path,
};

use aoc_2023_rust::{output, read_lines, read_lines_from_string, Runner};

const INPUT: &str = "input/day15.txt";

#[derive(Debug, Default)]
pub struct Day15 {
    init_sequence: InitializationSequence,
}

impl Day15 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day15 {
    fn name(&self) -> (usize, usize) {
        (2023, 15)
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
            puzzle_input = read_lines(INPUT);
        }

        self.init_sequence = InitializationSequence::from(puzzle_input[0].as_str());
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.init_sequence
                .steps
                .iter()
                .map(|s| s.hash_algorithm())
                .sum::<u32>(),
        )
    }
    fn part2(&mut self) -> Vec<String> {
        let mut facility = Facility::default();
        self.init_sequence.steps.iter().for_each(|s| {
            let len = Len::from(s);
            facility.add_box(&len);
        });
        output(facility.calculate_focusing_power())
    }
}

// ---------------------------------------------------

#[derive(Debug, Default, Clone)]
struct Step {
    string: String,
}

impl Step {
    fn new(string: &str) -> Self {
        Self {
            string: string.to_string(),
        }
    }
    fn hash_algorithm(&self) -> u32 {
        let mut current_value = 0;
        for c in self.string.chars() {
            let ascii_code = c as u32;
            current_value += ascii_code;
            current_value *= 17;
            current_value %= 256;
        }
        current_value
    }
}

#[derive(Debug, Default, Clone)]
struct InitializationSequence {
    steps: Vec<Step>,
}

impl From<&str> for InitializationSequence {
    fn from(input: &str) -> Self {
        let steps = input
            .split(',')
            .map(|s| Step {
                string: s.to_string(),
            })
            .collect();
        Self { steps }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Dash,
    Equals,
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        match input {
            "-" => Operation::Dash,
            "=" => Operation::Equals,
            _ => panic!("Invalid operation"),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Facility {
    boxes: HashMap<u32, Box>,
}

impl Facility {
    fn add_box(&mut self, len: &Len) {
        let box_id = len.box_id;
        self.boxes.entry(box_id).or_default();
        self.boxes.get_mut(&box_id).unwrap().add_len(len);
        if self.boxes.get(&box_id).unwrap().is_empty() {
            self.boxes.remove(&box_id);
        }
    }

    fn calculate_focusing_power(&self) -> u32 {
        self.boxes
            .values()
            .map(|b| b.calculate_focusing_power())
            .sum()
    }
}

#[derive(Debug, Default, Clone)]
struct Box {
    lens: VecDeque<Len>,
}

impl Box {
    fn add_len(&mut self, len: &Len) {
        match len.operation {
            Operation::Dash => {
                if let Some(pos) = self.lens.iter().position(|l| l.label == len.label) {
                    self.lens.remove(pos);
                }
            }
            Operation::Equals => {
                if let Some(pos) = self.lens.iter().position(|l| l.label == len.label) {
                    let lens = self.lens.get_mut(pos).unwrap();
                    lens.focal_length = len.focal_length;
                } else {
                    self.lens.push_back(len.clone());
                }
            }
        }
    }

    fn calculate_focusing_power(&self) -> u32 {
        let mut result = 0;
        for (i, len) in self.lens.iter().enumerate() {
            result += (len.box_id + 1) * (i + 1) as u32 * len.focal_length.unwrap_or(1);
        }
        result
    }

    fn is_empty(&self) -> bool {
        self.lens.is_empty()
    }
}

#[derive(Debug, Clone)]
struct Len {
    label: String,
    box_id: u32,
    focal_length: Option<u32>,
    operation: Operation,
}

impl From<&Step> for Len {
    fn from(step: &Step) -> Self {
        if step.string.contains('=') {
            let (label, focal) = step.string.split_once('=').unwrap();
            let box_id = Step::new(label).hash_algorithm();
            let focal_length = focal.parse::<u32>().unwrap();
            return Self {
                label: label.to_string(),
                box_id,
                focal_length: Some(focal_length),
                operation: Operation::Equals,
            };
        }

        let (label, _) = step.string.split_once('-').unwrap();
        let box_id = Step::new(label).hash_algorithm();
        Self {
            label: label.to_string(),
            box_id,
            focal_length: None,
            operation: Operation::Dash,
        }
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day15-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day15::Day15;

    #[test]
    fn part1_test_works() {
        let mut day = Day15::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "1320")
    }

    #[test]
    fn part1_works() {
        let mut day = Day15::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "516804")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day15::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "145")
    }

    #[test]
    fn part2_works() {
        let mut day = Day15::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "231844")
    }
}
