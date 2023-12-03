use aoc_2023_rust::{output, read_lines, Point, Runner};

const INPUT: &str = "input/day03.txt";

#[derive(Debug, Default)]
pub struct Day03 {
    part_numbers: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day03 {
    fn name(&self) -> (usize, usize) {
        (2023, 3)
    }

    fn parse(&mut self, input: Option<&str>) {
        let input = read_lines(input.unwrap_or(INPUT));
        self.part_numbers = Vec::new();
        self.symbols = Vec::new();

        for (x, line) in input.iter().enumerate() {
            let mut part_number = String::new();
            for (y, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    part_number.push(c);
                } else if c == '.' {
                    if !part_number.is_empty() {
                        let mut locations = Vec::new();
                        for loc in y - part_number.len()..y {
                            locations.push(Point::new(x as u32, loc as u32));
                        }
                        self.part_numbers
                            .push(PartNumber::new(part_number.parse().unwrap(), locations));
                        part_number.clear();
                    }
                } else {
                    if !part_number.is_empty() {
                        let mut locations = Vec::new();
                        for loc in y - part_number.len()..y {
                            locations.push(Point::new(x as u32, loc as u32));
                        }
                        self.part_numbers
                            .push(PartNumber::new(part_number.parse().unwrap(), locations));
                        part_number.clear();
                    }
                    self.symbols
                        .push(Symbol::new(c, Point::new(x as u32, y as u32)));
                }
                if y == line.len() - 1 && !part_number.is_empty() {
                    let mut locations = Vec::new();
                    for loc in y - part_number.len()..y {
                        locations.push(Point::new(x as u32, loc as u32));
                    }
                    self.part_numbers
                        .push(PartNumber::new(part_number.parse().unwrap(), locations));
                    part_number.clear();
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.part_numbers
                .iter()
                .filter(|p| self.symbols.iter().any(|s| p.is_adjacent(s)))
                .map(|p| p.num)
                .sum::<u32>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut gear_ratio = 0;
        self.symbols
            .iter()
            .filter(|s| s.symbol == '*')
            .for_each(|s| {
                let mut count = 0;
                let mut numbers = Vec::new();
                self.part_numbers.iter().for_each(|p| {
                    if p.is_adjacent(s) {
                        numbers.push(p.num);
                        count += 1;
                    }
                });
                if count == 2 {
                    gear_ratio += numbers[0] * numbers[1];
                }
            });
        output(gear_ratio)
    }
}

// ---------------------------------------------------

#[derive(Debug, Default)]
struct PartNumber {
    num: u32,
    locations: Vec<Point<u32>>,
}

impl std::fmt::Display for PartNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {:?}", self.num, self.locations)
    }
}

impl PartNumber {
    fn new(num: u32, locations: Vec<Point<u32>>) -> Self {
        Self { num, locations }
    }

    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        for loc in &self.locations {
            if u32::abs_diff(loc.x, symbol.location.x) <= 1
                && u32::abs_diff(loc.y, symbol.location.y) <= 1
            {
                return true;
            }
            if loc.dist(symbol.location.x, symbol.location.y) == 1 {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Default)]
struct Symbol {
    symbol: char,
    location: Point<u32>,
}

impl Symbol {
    fn new(symbol: char, location: Point<u32>) -> Self {
        Self { symbol, location }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at ({}, {})",
            self.symbol, self.location.x, self.location.y
        )
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day03-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day03::Day03;

    #[test]
    fn part1_test_works() {
        let mut day = Day03::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "4361")
    }

    #[test]
    fn part1_works() {
        let mut day = Day03::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "550064")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day03::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "467835")
    }

    #[test]
    fn part2_works() {
        let mut day = Day03::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "85010461")
    }
}
