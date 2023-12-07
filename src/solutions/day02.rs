use aoc_2023_rust::{output, read_lines, Runner};
use core::panic;
use std::str::FromStr;

const INPUT: &str = "input/day02.txt";

#[derive(Debug, Default)]
pub struct Day02 {
    games: Vec<Game>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day02 {
    fn name(&self) -> (usize, usize) {
        (2023, 2)
    }

    fn parse(&mut self, input: Option<&str>) {
        let input = input.unwrap_or(INPUT);
        self.games = Vec::new();
        for line in read_lines(input) {
            match Game::from_str(&line) {
                Ok(game) => self.games.push(game),
                Err(_) => panic!("Failed to parse game"),
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.games
                .iter()
                .filter(|g| g.is_possible())
                .map(|g| g.id as u32)
                .sum::<u32>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        output(
            self.games
                .iter()
                .map(|g| {
                    let max_blue = g.sets.iter().map(|s| s.blue).max().unwrap();
                    let max_red = g.sets.iter().map(|s| s.red).max().unwrap();
                    let max_green = g.sets.iter().map(|s| s.green).max().unwrap();
                    max_blue as u32 * max_red as u32 * max_green as u32
                })
                .sum::<u32>(),
        )
    }
}

// ---------------------------------------------------

type GameId = u8;

#[derive(Debug, Default)]
struct GameSet {
    blue: u8,
    red: u8,
    green: u8,
}

#[derive(Debug, Default)]
struct Game {
    id: u8,
    sets: Vec<GameSet>,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("Game: {}", self.id);
        for set in &self.sets {
            s.push_str(&format!("\n\t{:?}", set));
        }
        write!(f, "{}", s)
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.sets.iter().all(|s| {
            let (blue, red, green) = (s.blue, s.red, s.green);
            blue <= 14 && red <= 12 && green <= 13
        })
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game::default();
        let (g, rest) = s.split_once(':').unwrap();
        let (_, id) = g.split_once(' ').unwrap();
        let id = id.parse::<GameId>().unwrap();
        game.id = id;
        rest.split(';').for_each(|set| {
            let mut game_set = GameSet::default();
            set.split(',').for_each(|s| {
                let (count, color) = s.trim().split_once(' ').unwrap();
                let count = count.parse::<u8>().unwrap();
                match color {
                    "blue" => game_set.blue = count,
                    "red" => game_set.red = count,
                    "green" => game_set.green = count,
                    _ => (),
                }
            });
            game.sets.push(game_set);
        });
        Ok(game)
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day02-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day02::Day02;

    #[test]
    fn part1_test_works() {
        let mut day = Day02::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "8")
    }

    #[test]
    fn part1_works() {
        let mut day = Day02::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "2256")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day02::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "2286")
    }

    #[test]
    fn part2_works() {
        let mut day = Day02::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "74229")
    }
}
