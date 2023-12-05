use std::str::FromStr;

use aoc_2023_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day05.txt";

#[derive(Debug, Default)]
pub struct Day05 {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day05 {
    fn name(&self) -> (usize, usize) {
        (2023, 5)
    }

    fn parse(&mut self, input: Option<&str>) {
        let mut input = read_lines(input.unwrap_or(INPUT));
        while !input.is_empty() {
            let line = input.remove(0);
            if line.starts_with("seeds") {
                let (_, numbers) = line.split_once(':').unwrap();
                self.seeds = numbers
                    .trim()
                    .split(' ')
                    .map(|n| n.parse().unwrap())
                    .collect();
                continue;
            }
            if line.contains("map") {
                let (dest_type, source_type) = line.split_once("-to-").unwrap();
                let dest_type = dest_type.trim();
                let (source_type, _) = source_type.trim().split_once(' ').unwrap();
                let mut ranges = Vec::new();
                loop {
                    if !input.is_empty() && input[0].contains("map") {
                        break;
                    }
                    if input.is_empty() {
                        break;
                    }
                    let line = input.remove(0);
                    if line.trim().is_empty() {
                        break;
                    }
                    ranges.push(line.parse().unwrap());
                }
                self.maps.push(Map::new(
                    dest_type.to_string(),
                    source_type.to_string(),
                    ranges,
                ));
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.seeds
                .iter()
                .map(|s| {
                    let mut original_seed = *s;
                    for map in &self.maps {
                        for range in &map.source_maps {
                            let next_seed = range.map_source_to_destination(original_seed);
                            if next_seed != original_seed {
                                original_seed = next_seed;
                                break;
                            }
                        }
                    }
                    original_seed
                })
                .min()
                .unwrap(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut min_location = usize::MAX;
        for index in (0..self.seeds.len()).step_by(2) {
            let seed_start = self.seeds[index];
            let seed_length = self.seeds[index + 1];
            for seed in seed_start..seed_start + seed_length {
                let mut original_seed = seed;
                for map in &self.maps {
                    for range in &map.source_maps {
                        let next_seed = range.map_source_to_destination(original_seed);
                        if next_seed != original_seed {
                            original_seed = next_seed;
                            break;
                        }
                    }
                }
                min_location = min_location.min(original_seed);
            }
        }
        output(min_location)
    }
}

// ---------------------------------------------------

#[derive(Debug, Default)]
struct Map {
    dest_type: String,
    source_type: String,
    source_maps: Vec<Range>,
}

impl Map {
    fn new(dest_type: String, source_type: String, source_maps: Vec<Range>) -> Self {
        Self {
            dest_type,
            source_type,
            source_maps,
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {:#?}",
            self.dest_type, self.source_type, self.source_maps,
        )
    }
}

#[derive(Debug, Default)]
struct Range {
    source: usize,
    destination: usize,
    length: usize,
}

impl Range {
    fn new(source: usize, destination: usize, length: usize) -> Self {
        Self {
            source,
            destination,
            length,
        }
    }

    fn map_source_to_destination(&self, source: usize) -> usize {
        if source < self.source || source >= self.source + self.length {
            return source;
        }
        let index = source - self.source;
        self.destination + index
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.source, self.destination, self.length)
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let destination = parts.next().unwrap().parse().unwrap();
        let source = parts.next().unwrap().parse().unwrap();
        let length = parts.next().unwrap().parse().unwrap();
        Ok(Self::new(source, destination, length))
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day05-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day05::Day05;

    #[test]
    fn part1_test_works() {
        let mut day = Day05::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "35")
    }

    #[test]
    fn part1_works() {
        let mut day = Day05::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "662197086")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day05::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "46")
    }

    #[test]
    fn part2_works() {
        let mut day = Day05::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "8477787")
    }
}
