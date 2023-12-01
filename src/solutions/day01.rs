use aoc_2023_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day01.txt";
const DIGITS_IN_LETTERS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];
pub struct Day01 {
    document: Vec<String>,
}

impl Day01 {
    pub fn new() -> Self {
        Self {
            document: Vec::new(),
        }
    }
}

impl Runner for Day01 {
    fn name(&self) -> (usize, usize) {
        (2023, 1)
    }

    fn parse(&mut self, input: Option<&str>) {
        if let Some(input) = input {
            self.document = read_lines(input);
            return;
        }
        self.document = read_lines(INPUT);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;
        for text in &self.document {
            let digits: Vec<u32> = text.chars().filter_map(|c| c.to_digit(10)).collect();
            total += format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<u32>()
                .unwrap();
        }

        output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;
        for text in &self.document {
            let indices = Indices::find_digit_indices(text);
            total += format!(
                "{}{}",
                indices.first().map(|i| i.find_digit_in_text(text)).unwrap(),
                indices.last().map(|i| i.find_digit_in_text(text)).unwrap(),
            )
            .parse::<u32>()
            .unwrap();
        }

        output(total)
    }
}

// ---------------------------------------------------

#[derive(Debug, Default)]
struct Indices {
    first: usize,
    second: usize,
}

impl std::fmt::Display for Indices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.first, self.second)
    }
}

impl Indices {
    fn new(first: usize, second: usize) -> Self {
        Self { first, second }
    }

    fn find_digit_indices(text: &str) -> Vec<Self> {
        let mut indices = Vec::<Indices>::new();
        for letter in DIGITS_IN_LETTERS {
            text.match_indices(letter.0).for_each(|(i, _)| {
                indices.push(Indices::new(i, letter.0.len() - 1));
            });
        }
        for (i, c) in text.chars().enumerate() {
            if c.is_ascii_digit() {
                indices.push(Indices::new(i, 1));
            }
        }
        indices.sort_by_key(|k| k.first);
        indices
    }

    fn find_digit_in_text(&self, text: &str) -> u32 {
        if self.second == 1 {
            return text.chars().nth(self.first).unwrap().to_digit(10).unwrap();
        }
        for d in DIGITS_IN_LETTERS {
            if d.0 == &text[self.first..=self.first + self.second] {
                return d.1;
            }
        }
        0
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day01-test.txt";
    const TEST_INPUT_P2: &str = "input/day01-test2.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day01::Day01;

    #[test]
    fn part1_test_works() {
        let mut day01 = Day01::new();
        day01.parse(Some(TEST_INPUT));
        let output = day01.part1();
        assert_eq!(output[0], "142")
    }

    #[test]
    fn part1_works() {
        let mut day01 = Day01::new();
        day01.parse(None);
        let output = day01.part1();
        assert_eq!(output[0], "54081")
    }

    #[test]
    fn part2_test_works() {
        let mut day01 = Day01::new();
        day01.parse(Some(TEST_INPUT_P2));
        let output = day01.part2();
        assert_eq!(output[0], "281")
    }

    #[test]
    fn part2_works() {
        let mut day01 = Day01::new();
        day01.parse(None);
        let output = day01.part2();
        assert_eq!(output[0], "54649")
    }
}
