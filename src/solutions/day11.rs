use std::{collections::HashMap, path::Path};

use aoc_2023_rust::{output, read_lines, read_lines_from_string, Point, Runner};
use itertools::Itertools;

const INPUT: &str = "input/day11.txt";

#[derive(Debug, Default)]
pub struct Day11 {
    universe: Universe,
}

impl Day11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day11 {
    fn name(&self) -> (usize, usize) {
        (2023, 11)
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
        self.universe.width = puzzle_input.first().unwrap().len() as u64;
        self.universe.height = puzzle_input.len() as u64;

        let mut number = 1;
        for (y, line) in puzzle_input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                self.universe.galaxies.push(Galaxy {
                    location: Point::new(x as i32, y as i32),
                    number,
                });
                number += 1;
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        self.universe.expand_universe(2);
        output(self.universe.calculate_total_distance_between_galaxies())
    }

    fn part2(&mut self) -> Vec<String> {
        self.universe.expand_universe(1_000_000);
        output(self.universe.calculate_total_distance_between_galaxies())
    }
}

// ---------------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
struct Universe {
    galaxies: Vec<Galaxy>,
    width: u64,
    height: u64,
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Universe with {} galaxies", self.galaxies.len())
    }
}

impl Universe {
    fn calculate_total_distance_between_galaxies(&self) -> u64 {
        let mut total_distance: i64 = 0;
        for pairs in self.galaxies.iter().map(|g| g.number).combinations(2) {
            let a = self.galaxies.iter().find(|g| g.number == pairs[0]).unwrap();
            let b = self.galaxies.iter().find(|g| g.number == pairs[1]).unwrap();
            total_distance += a.location.dist(b.location.x, b.location.y) as i64
        }
        total_distance as u64
    }

    fn expand_universe(&mut self, expansion_rate: u64) {
        let empty_spaces = self.find_empty_spaces();
        for (k, v) in empty_spaces.iter() {
            if k == "rows" {
                self.galaxies
                    .iter_mut()
                    .filter(|g| {
                        for y in v {
                            if g.location.y > *y as i32 {
                                return true;
                            }
                        }
                        false
                    })
                    .for_each(|g| {
                        let mut expand_rate: u64 = 0;
                        for y in v {
                            if y < &(g.location.y as u64) {
                                expand_rate += 1;
                            }
                        }
                        g.location.y += (expand_rate * (expansion_rate - 1)) as i32;
                    })
            } else {
                self.galaxies
                    .iter_mut()
                    .filter(|g| {
                        for x in v {
                            if g.location.x as u64 > *x {
                                return true;
                            }
                        }
                        false
                    })
                    .for_each(|g| {
                        let mut expand_rate = 0;
                        for x in v {
                            if x < &(g.location.x as u64) {
                                expand_rate += 1;
                            }
                        }
                        g.location.x += (expand_rate * (expansion_rate - 1)) as i32;
                    })
            }
        }
    }

    fn find_empty_spaces(&self) -> HashMap<String, Vec<u64>> {
        let mut empty_spaces = HashMap::new();
        for y in 0..self.height {
            let mut empty = true;
            for x in 0..self.width {
                let p = Point::new(x as i32, y as i32);
                if self.galaxies.iter().any(|g| g.location == p) {
                    empty = false;
                    break;
                }
            }
            if empty {
                empty_spaces
                    .entry("rows".to_string())
                    .or_insert(Vec::new())
                    .push(y);
            }
        }
        for x in 0..self.width {
            let mut empty = true;
            for y in 0..self.height {
                let p = Point::new(x as i32, y as i32);
                if self.galaxies.iter().any(|g| g.location == p) {
                    empty = false;
                    break;
                }
            }
            if empty {
                empty_spaces
                    .entry("cols".to_string())
                    .or_insert(Vec::new())
                    .push(x);
            }
        }
        empty_spaces
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Galaxy {
    location: Point<i32>,
    number: u32,
}

impl std::fmt::Display for Galaxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Galaxy at {}", self.location)
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day11-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day11::Day11;

    #[test]
    fn part1_test_works() {
        let mut day = Day11::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "374")
    }

    #[test]
    fn part1_works() {
        let mut day = Day11::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "10154062")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day11::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "82000210")
    }

    #[test]
    fn part2_works() {
        let mut day = Day11::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "553083047914")
    }
}
