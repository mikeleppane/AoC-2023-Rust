use std::collections::HashMap;

use aoc_2023_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day07.txt";

#[derive(Debug, Default)]
pub struct Day07 {
    hands: Vec<Hand>,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day07 {
    fn name(&self) -> (usize, usize) {
        (2023, 7)
    }

    fn parse(&mut self, input: Option<&str>) {
        let input = read_lines(input.unwrap_or(INPUT));
        for line in input {
            let line = line.trim();
            self.hands.push(Hand::from(line));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        sort_hands(&mut self.hands);
        output(
            self.hands
                .iter()
                .enumerate()
                .map(|(i, h)| (i as u32 + 1) * h.bid)
                .sum::<u32>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        sort_hands_with_joker(&mut self.hands);
        output(
            self.hands
                .iter()
                .enumerate()
                .map(|(i, h)| (i as u32 + 1) * h.bid)
                .sum::<u32>(),
        )
    }
}

// ---------------------------------------------------

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Card {
    A(u8),
    K(u8),
    Q(u8),
    J(u8),
    T(u8),
    Nine(u8),
    Eight(u8),
    Seven(u8),
    Six(u8),
    Five(u8),
    Four(u8),
    Three(u8),
    Two(u8),
}

impl Card {
    fn new(s: char) -> Self {
        match s {
            'A' => Card::A(14),
            'K' => Card::K(13),
            'Q' => Card::Q(12),
            'J' => Card::J(11),
            'T' => Card::T(10),
            '9' => Card::Nine(9),
            '8' => Card::Eight(8),
            '7' => Card::Seven(7),
            '6' => Card::Six(6),
            '5' => Card::Five(5),
            '4' => Card::Four(4),
            '3' => Card::Three(3),
            '2' => Card::Two(2),
            _ => panic!("Invalid card"),
        }
    }

    fn value(&self) -> u8 {
        match self {
            Card::A(v) => *v,
            Card::K(v) => *v,
            Card::Q(v) => *v,
            Card::J(v) => *v,
            Card::T(v) => *v,
            Card::Nine(v) => *v,
            Card::Eight(v) => *v,
            Card::Seven(v) => *v,
            Card::Six(v) => *v,
            Card::Five(v) => *v,
            Card::Four(v) => *v,
            Card::Three(v) => *v,
            Card::Two(v) => *v,
        }
    }

    fn is_joker(&self) -> bool {
        self.value() == 11
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Card::A(_) => "A",
            Card::K(_) => "K",
            Card::Q(_) => "Q",
            Card::J(_) => "J",
            Card::T(_) => "T",
            Card::Nine(_) => "9",
            Card::Eight(_) => "8",
            Card::Seven(_) => "7",
            Card::Six(_) => "6",
            Card::Five(_) => "5",
            Card::Four(_) => "4",
            Card::Three(_) => "3",
            Card::Two(_) => "2",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Default, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let mut hand = Self::new();
        let (cards, bid) = s.split_once(' ').unwrap();
        for c in cards.chars() {
            hand.add_card(Card::new(c));
        }
        hand.bid = bid.parse().unwrap();

        hand
    }
}

impl Hand {
    fn new() -> Self {
        Self::default()
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn replace_card(&mut self, index: usize, card: Card) {
        self.cards[index] = card;
    }

    fn joker_count(&self) -> u32 {
        self.cards.iter().filter(|c| c.value() == 11).count() as u32
    }

    fn is_five_of_a_kind(&self) -> bool {
        let mut counts = HashMap::new();
        for c in self.cards.iter() {
            *counts.entry(c.value()).or_insert(0) += 1;
        }
        counts.values().any(|&v| v == 5)
    }

    fn is_four_of_a_kind(&self) -> bool {
        let mut counts = HashMap::new();
        for c in self.cards.iter() {
            *counts.entry(c.value()).or_insert(0) += 1;
        }
        counts.values().any(|&v| v == 4)
    }

    fn is_three_of_a_kind(&self) -> bool {
        let mut counts = HashMap::new();
        for c in self.cards.iter() {
            *counts.entry(c.value()).or_insert(0) += 1;
        }
        counts.values().any(|&v| v == 3)
    }

    fn is_full_house(&self) -> bool {
        let mut counts = HashMap::new();
        for c in self.cards.iter() {
            *counts.entry(c.value()).or_insert(0) += 1;
        }
        counts.values().any(|&v| v == 3) && counts.values().any(|&v| v == 2)
    }

    fn is_two_pairs(&self) -> bool {
        let mut counts = HashMap::new();
        for c in self.cards.iter() {
            *counts.entry(c.value()).or_insert(0) += 1;
        }
        counts.values().filter(|&v| *v == 2).count() == 2
    }

    fn is_one_pair(&self) -> bool {
        let mut counts = HashMap::new();
        for c in self.cards.iter() {
            *counts.entry(c.value()).or_insert(0) += 1;
        }
        counts.values().any(|&v| v == 2)
    }

    fn get_highest_card_count_excluding_joker(&self) -> Option<Card> {
        let mut counts = HashMap::new();
        for c in self.cards.iter() {
            if c.is_joker() {
                continue;
            }
            *counts.entry(c).or_insert(0) += 1;
        }
        if counts.is_empty() {
            return None;
        }
        Some(
            counts
                .into_iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .unwrap()
                .0
                .clone(),
        )
    }

    fn make_best_possible_hand(&self) -> Self {
        let mut hand = Self::clone(self);
        let mut joker_count = self.joker_count();
        if joker_count > 0 {
            let highest_card_count = self.get_highest_card_count_excluding_joker();
            if let Some(card) = highest_card_count {
                while joker_count > 0 {
                    let pos = hand.cards.iter().position(|c| c.is_joker()).unwrap();
                    hand.replace_card(pos, card.clone());
                    joker_count -= 1;
                }
            }
        }
        hand
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cards = String::new();
        for c in self.cards.iter() {
            cards.push_str(&format!("{} ", c));
        }
        write!(f, "{}", cards)
    }
}

fn sort_hands_with_joker(hands: &mut [Hand]) {
    hands.sort_by(|a, b| {
        let a_new = a.make_best_possible_hand();
        let b_new = b.make_best_possible_hand();
        if a_new.is_five_of_a_kind() && !b_new.is_five_of_a_kind() {
            return std::cmp::Ordering::Greater;
        } else if !a_new.is_five_of_a_kind() && b_new.is_five_of_a_kind() {
            return std::cmp::Ordering::Less;
        }
        if a_new.is_four_of_a_kind() && !b_new.is_four_of_a_kind() {
            return std::cmp::Ordering::Greater;
        } else if !a_new.is_four_of_a_kind() && b_new.is_four_of_a_kind() {
            return std::cmp::Ordering::Less;
        }

        if a_new.is_full_house() && !b_new.is_full_house() {
            return std::cmp::Ordering::Greater;
        } else if !a_new.is_full_house() && b_new.is_full_house() {
            return std::cmp::Ordering::Less;
        }

        if a_new.is_three_of_a_kind() && !b_new.is_three_of_a_kind() {
            return std::cmp::Ordering::Greater;
        } else if !a_new.is_three_of_a_kind() && b_new.is_three_of_a_kind() {
            return std::cmp::Ordering::Less;
        }

        if a_new.is_two_pairs() && !b_new.is_two_pairs() {
            return std::cmp::Ordering::Greater;
        } else if !a_new.is_two_pairs() && b_new.is_two_pairs() {
            return std::cmp::Ordering::Less;
        }

        if a_new.is_one_pair() && !b_new.is_one_pair() {
            return std::cmp::Ordering::Greater;
        } else if !a_new.is_one_pair() && b_new.is_one_pair() {
            return std::cmp::Ordering::Less;
        }

        for (a_card, b_card) in a.cards.iter().zip(b.cards.iter()) {
            let a_value = if a_card.value() == 11 {
                1
            } else {
                a_card.value()
            };
            let b_value = if b_card.value() == 11 {
                1
            } else {
                b_card.value()
            };
            match a_value.cmp(&b_value) {
                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                _ => continue,
            }
        }
        std::cmp::Ordering::Equal
    });
}

fn sort_hands(hands: &mut [Hand]) {
    hands.sort_by(|a, b| {
        if a.is_five_of_a_kind() && !b.is_five_of_a_kind() {
            return std::cmp::Ordering::Greater;
        } else if !a.is_five_of_a_kind() && b.is_five_of_a_kind() {
            return std::cmp::Ordering::Less;
        }
        if a.is_four_of_a_kind() && !b.is_four_of_a_kind() {
            return std::cmp::Ordering::Greater;
        } else if !a.is_four_of_a_kind() && b.is_four_of_a_kind() {
            return std::cmp::Ordering::Less;
        }

        if a.is_full_house() && !b.is_full_house() {
            return std::cmp::Ordering::Greater;
        } else if !a.is_full_house() && b.is_full_house() {
            return std::cmp::Ordering::Less;
        }

        if a.is_three_of_a_kind() && !b.is_three_of_a_kind() {
            return std::cmp::Ordering::Greater;
        } else if !a.is_three_of_a_kind() && b.is_three_of_a_kind() {
            return std::cmp::Ordering::Less;
        }

        if a.is_two_pairs() && !b.is_two_pairs() {
            return std::cmp::Ordering::Greater;
        } else if !a.is_two_pairs() && b.is_two_pairs() {
            return std::cmp::Ordering::Less;
        }

        if a.is_one_pair() && !b.is_one_pair() {
            return std::cmp::Ordering::Greater;
        } else if !a.is_one_pair() && b.is_one_pair() {
            return std::cmp::Ordering::Less;
        }

        for (a_card, b_card) in a.cards.iter().zip(b.cards.iter()) {
            match a_card.value().cmp(&b_card.value()) {
                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                _ => continue,
            }
        }
        std::cmp::Ordering::Equal
    });
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day07-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day07::Day07;

    #[test]
    fn part1_test_works() {
        let mut day = Day07::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "6440")
    }

    #[test]
    fn part1_works() {
        let mut day = Day07::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "246163188")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day07::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "5905")
    }

    #[test]
    fn part2_works() {
        let mut day = Day07::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "245794069")
    }
}
