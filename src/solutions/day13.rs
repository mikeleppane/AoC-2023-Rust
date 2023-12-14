use std::{collections::HashMap, fs::read_to_string, vec};

use aoc_2023_rust::{output, OutputStatus, Point, Runner};
use itertools::Itertools;

const INPUT: &str = "input/day13.txt";

#[derive(Debug, Default)]
pub struct Day13 {
    patterns: Vec<Pattern>,
}

impl Day13 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day13 {
    fn name(&self) -> (usize, usize) {
        (2023, 13)
    }

    fn parse(&mut self, _: Option<&str>) {
        let input = read_to_string(INPUT)
            .expect("unable to open file")
            .trim()
            .to_owned();
        for pattern in input.split("\n\n").collect::<Vec<_>>() {
            self.patterns.push(Pattern::from(pattern));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut c1 = 0;
        let mut c2 = 0;
        for p in self.patterns.iter() {
            let (h, v) = p.find_reflections();
            if h == 0 {
                c1 += v;
            } else {
                c2 += v;
            }
        }
        output(c1 + c2 * 100)
    }
    fn part2(&mut self) -> Vec<String> {
        output(OutputStatus::Unsolved)
    }
}

// ---------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Type {
    Ash,
    Rock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PatternData {
    point: Point<i32>,
    p_type: Type,
}

impl PatternData {
    fn new(point: Point<i32>, p_type: Type) -> Self {
        Self { point, p_type }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Pattern {
    data: Vec<Vec<PatternData>>,
    width: u32,
    height: u32,
}

impl Pattern {
    fn find_reflections(&self) -> (u32, u32) {
        let mut reflections = HashMap::new();
        reflections.insert("horizontal".to_string(), Vec::new());
        reflections.insert("vertical".to_string(), Vec::new());
        for i in 0..self.data.len() - 1 {
            for j in i + 1..self.data.len() {
                if self.data[i].iter().map(|p| p.p_type).collect_vec()
                    == self.data[j].iter().map(|p| p.p_type).collect_vec()
                {
                    reflections
                        .entry("horizontal".to_string())
                        .or_insert_with(Vec::new)
                        .push((i as u32, j as u32));
                }
            }
        }
        for i in 0..self.width - 1 {
            for j in i + 1..self.width {
                let mut data1 = Vec::new();
                let mut data2 = Vec::new();
                for row in &self.data {
                    data1.push(&row[i as usize]);
                    data2.push(&row[j as usize]);
                }
                if data1.iter().map(|p| p.p_type).collect_vec()
                    == data2.iter().map(|p| p.p_type).collect_vec()
                {
                    reflections
                        .entry("vertical".to_string())
                        .or_insert_with(Vec::new)
                        .push((i, j));
                }
            }
        }
        let verticals = reflections
            .get("vertical")
            .unwrap()
            .iter()
            .flat_map(|(i, j)| vec![i, j])
            .dedup()
            .sorted()
            .collect_vec()
            .windows(2)
            .filter(|w| w[1].abs_diff(*w[0]) == 1)
            .count();
        let horizontals = reflections
            .get("horizontal")
            .unwrap()
            .iter()
            .flat_map(|(i, j)| vec![i, j])
            .dedup()
            .sorted()
            .collect_vec()
            .windows(2)
            .filter(|w| w[1].abs_diff(*w[0]) == 1)
            .count();
        if verticals >= horizontals {
            return (
                0,
                *reflections
                    .get("vertical")
                    .unwrap()
                    .iter()
                    .filter(|d| d.0.abs_diff(d.1) == 1)
                    .map(|d| d.1)
                    .collect_vec()
                    .iter()
                    .max()
                    .unwrap_or(&0),
            );
        }
        return (
            1,
            *reflections
                .get("horizontal")
                .unwrap()
                .iter()
                .filter(|d| d.0.abs_diff(d.1) == 1)
                .map(|d| d.1)
                .collect_vec()
                .iter()
                .max()
                .unwrap_or(&0),
        );
    }
}

impl From<&str> for Pattern {
    fn from(input: &str) -> Self {
        let mut pattern = Self::default();
        for (y, line) in input.lines().enumerate() {
            let mut data = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                let point = Point::new(x as i32, y as i32);
                match ch {
                    '#' => data.push(PatternData::new(point, Type::Rock)),
                    '.' => data.push(PatternData::new(point, Type::Ash)),
                    _ => panic!("invalid input"),
                };
            }
            pattern.data.push(data);
        }
        pattern.width = input.lines().next().unwrap().len() as u32;
        pattern.height = input.lines().count() as u32;
        pattern
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day13-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day13::Day13;

    #[test]
    fn part1_test_works() {
        let mut day = Day13::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "405")
    }

    #[test]
    fn part1_works() {
        let mut day = Day13::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "4")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day13::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "82000210")
    }

    #[test]
    fn part2_works() {
        let mut day = Day13::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "553083047914")
    }
}
