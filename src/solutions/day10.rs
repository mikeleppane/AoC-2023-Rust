use std::collections::{HashMap, HashSet};

use aoc_2023_rust::{output, read_lines, OutputStatus, Point, Runner};

const INPUT: &str = "input/day10.txt";

type Grid = HashMap<Point<i32>, Tile>;

#[derive(Debug, Default)]
pub struct Day10 {
    map: Grid,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day10 {
    fn name(&self) -> (usize, usize) {
        (2023, 10)
    }

    fn parse(&mut self, input: Option<&str>) {
        let input = read_lines(input.unwrap_or(INPUT));
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                self.map
                    .insert(Point::new(x as i32, y as i32), Tile::Pipe(c));
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let starting_position = *self
            .map
            .iter()
            .find(|(_, t)| **t == Tile::Pipe('S'))
            .unwrap()
            .0;
        let mut current_position = find_next_location_from_start(&self.map, starting_position);
        let mut main_loop: Vec<Point<i32>> = Vec::new();
        let mut visited: HashSet<Point<i32>> = HashSet::new();

        main_loop.push(starting_position);
        main_loop.push(current_position);

        visited.insert(starting_position);
        visited.insert(current_position);

        let neigbors = [
            (Point::new(0, 1), Direction::South),
            (Point::new(1, 0), Direction::East),
            (Point::new(-1, 0), Direction::West),
            (Point::new(0, -1), Direction::North),
        ];

        'outer: loop {
            let mut next_position = None;
            for (point, direction) in neigbors.iter() {
                let current_tile = self.map.get(&current_position).unwrap();
                let next_loc = current_position + *point;
                if main_loop.contains(&next_loc) {
                    continue;
                }
                if let Some(t) = self.map.get(&next_loc) {
                    if current_tile.can_connect(t, direction) {
                        current_position = next_loc;
                        next_position = Some(next_loc);
                        main_loop.push(next_loc);
                        visited.insert(next_loc);
                        break;
                    }
                }
            }
            if next_position.is_none() {
                break 'outer;
            }
        }
        let position = main_loop
            .iter()
            .skip(1)
            .zip(main_loop.iter().rev().skip(1))
            .position(|(a, b)| a.y == b.y)
            .unwrap();

        dbg!(position);

        output(main_loop.len() / 2)
    }

    fn part2(&mut self) -> Vec<String> {
        output(OutputStatus::Unsolved)
    }
}

// ---------------------------------------------------

fn find_next_location_from_start(map: &Grid, current_position: Point<i32>) -> Point<i32> {
    if let Some(t) = map.get(&(current_position + Point::new(0, 1))) {
        if t == &Tile::Pipe('|') || t == &Tile::Pipe('F') || t == &Tile::Pipe('7') {
            return current_position + Point::new(0, 1);
        }
    }
    if let Some(t) = map.get(&(current_position + Point::new(1, 0))) {
        if t == &Tile::Pipe('-') {
            return current_position + Point::new(1, 0);
        }
    }
    if let Some(t) = map.get(&(current_position + Point::new(-1, 0))) {
        if t == &Tile::Pipe('F') || t == &Tile::Pipe('L') {
            return current_position + Point::new(-1, 0);
        }
    }

    if let Some(t) = map.get(&(current_position + Point::new(0, -1))) {
        if t == &Tile::Pipe('|') || t == &Tile::Pipe('J') || t == &Tile::Pipe('L') {
            current_position + Point::new(-1, 0)
        } else {
            panic!("No tile found");
        }
    } else {
        panic!("No tile found");
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Pipe(char),
}

impl Tile {
    fn can_connect(&self, other: &Tile, direction: &Direction) -> bool {
        match direction {
            Direction::North => match self {
                Tile::Pipe('|') => {
                    if other == &Tile::Pipe('|')
                        || other == &Tile::Pipe('F')
                        || other == &Tile::Pipe('7')
                    {
                        return true;
                    }
                }
                Tile::Pipe('L') => {
                    if other == &Tile::Pipe('|')
                        || other == &Tile::Pipe('F')
                        || other == &Tile::Pipe('7')
                    {
                        return true;
                    }
                }
                Tile::Pipe('J') => {
                    if other == &Tile::Pipe('|')
                        || other == &Tile::Pipe('F')
                        || other == &Tile::Pipe('7')
                    {
                        return true;
                    }
                }
                _ => {}
            },
            Direction::South => match self {
                Tile::Pipe('|') => {
                    if other == &Tile::Pipe('|')
                        || other == &Tile::Pipe('L')
                        || other == &Tile::Pipe('J')
                    {
                        return true;
                    }
                }
                Tile::Pipe('F') => {
                    if other == &Tile::Pipe('|')
                        || other == &Tile::Pipe('L')
                        || other == &Tile::Pipe('J')
                    {
                        return true;
                    }
                }
                Tile::Pipe('7') => {
                    if other == &Tile::Pipe('|')
                        || other == &Tile::Pipe('L')
                        || other == &Tile::Pipe('J')
                    {
                        return true;
                    }
                }
                _ => {}
            },
            Direction::East => match self {
                Tile::Pipe('-') => {
                    if other == &Tile::Pipe('-')
                        || other == &Tile::Pipe('J')
                        || other == &Tile::Pipe('7')
                    {
                        return true;
                    }
                }
                Tile::Pipe('L') => {
                    if other == &Tile::Pipe('-')
                        || other == &Tile::Pipe('J')
                        || other == &Tile::Pipe('7')
                    {
                        return true;
                    }
                }
                Tile::Pipe('F') => {
                    if other == &Tile::Pipe('-')
                        || other == &Tile::Pipe('J')
                        || other == &Tile::Pipe('7')
                    {
                        return true;
                    }
                }
                _ => {}
            },
            Direction::West => match self {
                Tile::Pipe('-') => {
                    if other == &Tile::Pipe('-')
                        || other == &Tile::Pipe('F')
                        || other == &Tile::Pipe('L')
                    {
                        return true;
                    }
                }
                Tile::Pipe('J') => {
                    if other == &Tile::Pipe('-')
                        || other == &Tile::Pipe('F')
                        || other == &Tile::Pipe('L')
                    {
                        return true;
                    }
                }
                Tile::Pipe('7') => {
                    if other == &Tile::Pipe('-')
                        || other == &Tile::Pipe('F')
                        || other == &Tile::Pipe('L')
                    {
                        return true;
                    }
                }
                _ => {}
            },
        }

        false
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day10-test.txt";
    const TEST_INPUT2: &str = "input/day10-test2.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day10::Day10;

    #[test]
    fn part1_test_works() {
        let mut day = Day10::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "4")
    }

    #[test]
    fn part1_test2_works() {
        let mut day = Day10::new();
        day.parse(Some(TEST_INPUT2));
        let output = day.part1();
        assert_eq!(output[0], "8")
    }

    #[test]
    fn part1_works() {
        let mut day = Day10::new();
        day.parse(None);
        let _ = day.part1();
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day10::new();
        day.parse(Some(TEST_INPUT));
        let _ = day.part2();
    }

    #[test]
    fn part2_works() {
        let mut day = Day10::new();
        day.parse(None);
        let _ = day.part2();
    }
}
