use std::collections::HashMap;

use aoc_2023_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day04.txt";

type CardId = u32;

#[derive(Debug, Default)]
pub struct Day04 {
    cards: HashMap<CardId, Card>,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day04 {
    fn name(&self) -> (usize, usize) {
        (2023, 4)
    }

    fn parse(&mut self, input: Option<&str>) {
        let input = read_lines(input.unwrap_or(INPUT));
        self.cards = HashMap::new();

        for line in input {
            let (card, rest) = line.split_once(':').unwrap();
            let id: u32 = card.split_once(' ').unwrap().1.trim().parse().unwrap();
            let (winning_numbers, own_numbers) = rest.trim().split_once('|').unwrap();
            let winning_numbers: Vec<u32> = winning_numbers
                .split(' ')
                .filter(|n| !n.trim().is_empty())
                .map(|n| n.trim().parse().unwrap())
                .collect();
            let own_numbers: Vec<u32> = own_numbers
                .split(' ')
                .filter(|n| !n.trim().is_empty())
                .map(|n| n.trim().parse().unwrap())
                .collect();
            self.cards
                .insert(id, Card::new(id, winning_numbers, own_numbers));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.cards
                .values()
                .map(|c| c.calculate_points())
                .sum::<u32>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;
        for card in self.cards.values() {
            total += 1;
            let matching_numbers = card.matching_numbers;
            if matching_numbers == 0 {
                continue;
            }
            total += matching_numbers;
            let next_cards = card.id + 1..=card.id + matching_numbers;
            for card_id in next_cards {
                total += find_all_copies(&self.cards, card_id);
            }
        }

        output(total)
    }
}

// ---------------------------------------------------

fn find_all_copies(cards: &HashMap<CardId, Card>, card_id: u32) -> u32 {
    let mut total = 0;
    if let Some(card) = cards.get(&card_id) {
        let matching_numbers = card.matching_numbers;
        if matching_numbers == 0 {
            return 0;
        }
        total += matching_numbers;
        let next_cards = card.id + 1..=card.id + matching_numbers;
        for next_card in next_cards {
            total += find_all_copies(cards, next_card);
        }
        return total;
    }
    0
}

#[derive(Debug, Default)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    own_numbers: Vec<u32>,
    matching_numbers: u32,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Card #{}", self.id)?;
        writeln!(f, "Winning Numbers: {:?}", self.winning_numbers)?;
        writeln!(f, "Own Numbers: {:?}", self.own_numbers)
    }
}

impl Card {
    fn new(id: u32, winning_numbers: Vec<u32>, own_numbers: Vec<u32>) -> Self {
        let mut card = Card {
            id,
            winning_numbers,
            own_numbers,
            matching_numbers: 0,
        };
        card.matching_numbers = card.matching_numbers();
        card
    }

    fn calculate_points(&self) -> u32 {
        let mut points = 0;
        for n in self.own_numbers.iter() {
            if self.winning_numbers.contains(n) {
                if points == 0 {
                    points += 1;
                } else {
                    points *= 2;
                }
            }
        }
        points
    }

    fn matching_numbers(&self) -> u32 {
        self.own_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day04-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day04::Day04;

    #[test]
    fn part1_test_works() {
        let mut day = Day04::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "13")
    }

    #[test]
    fn part1_works() {
        let mut day = Day04::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "17782")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day04::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "30")
    }

    #[test]
    fn part2_works() {
        let mut day = Day04::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "8477787")
    }
}
