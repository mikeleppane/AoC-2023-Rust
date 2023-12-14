use std::path::Path;

use aoc_2023_rust::{output, read_lines, read_lines_from_string, Runner};

const INPUT: &str = "input/day14.txt";

#[derive(Debug, Default)]
pub struct Day14 {
    reflector: Reflector,
}

impl Day14 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day14 {
    fn name(&self) -> (usize, usize) {
        (2023, 14)
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

        self.reflector.width = puzzle_input[0].len() as u32;
        self.reflector.height = puzzle_input.len() as u32;

        for (_, line) in puzzle_input.iter().enumerate() {
            let mut row = Vec::new();
            for (_, c) in line.chars().enumerate() {
                let dish_type = match c {
                    'O' => DishType::RoundedRock,
                    '.' => DishType::Empty,
                    '#' => DishType::CubeRock,
                    _ => panic!("Unknown dish type"),
                };
                row.push(Dish { dish_type });
            }
            self.reflector.dishes.push(row);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        self.reflector.slide_to_north();
        output(self.reflector.calculate_load())
    }
    fn part2(&mut self) -> Vec<String> {
        for _ in 0..1_000 {
            self.reflector.cycle();
        }
        output(self.reflector.calculate_load())
    }
}

// ---------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DishType {
    RoundedRock,
    CubeRock,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Dish {
    dish_type: DishType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Reflector {
    dishes: Vec<Vec<Dish>>,
    width: u32,
    height: u32,
}

impl std::fmt::Display for Reflector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.dishes {
            for dish in row {
                let c = match dish.dish_type {
                    DishType::RoundedRock => 'O',
                    DishType::CubeRock => '#',
                    DishType::Empty => '.',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Reflector {
    fn calculate_load(&self) -> u32 {
        let mut total_load = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let dish = self.dishes[y as usize][x as usize];
                if dish.dish_type == DishType::RoundedRock {
                    total_load += self.height - y;
                }
            }
        }
        total_load
    }

    fn cycle(&mut self) {
        self.slide_to_north();
        self.slide_to_west();
        self.slide_to_south();
        self.slide_to_east();
    }

    fn slide_to_north(&mut self) {
        for y in 1..self.height {
            for x in 0..self.width {
                let dish = self.dishes[y as usize][x as usize];
                let mut next_y: i32 = y as i32;
                if dish.dish_type == DishType::RoundedRock {
                    loop {
                        if next_y > 0 {
                            let dish = self.dishes[(next_y - 1) as usize][x as usize];
                            if dish.dish_type == DishType::Empty {
                                next_y -= 1;
                                continue;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                if next_y != y as i32 {
                    self.dishes[y as usize][x as usize] = Dish {
                        dish_type: DishType::Empty,
                    };
                    self.dishes[next_y as usize][x as usize] = Dish {
                        dish_type: DishType::RoundedRock,
                    };
                }
            }
        }
    }

    fn slide_to_west(&mut self) {
        for y in 0..self.height {
            for x in 1..self.width {
                let dish = self.dishes[y as usize][x as usize];
                let mut next_x: i32 = x as i32;
                if dish.dish_type == DishType::RoundedRock {
                    loop {
                        if next_x > 0 {
                            let dish = self.dishes[y as usize][(next_x - 1) as usize];
                            if dish.dish_type == DishType::Empty {
                                next_x -= 1;
                                continue;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                if next_x != x as i32 {
                    self.dishes[y as usize][x as usize] = Dish {
                        dish_type: DishType::Empty,
                    };
                    self.dishes[y as usize][next_x as usize] = Dish {
                        dish_type: DishType::RoundedRock,
                    };
                }
            }
        }
    }

    fn slide_to_south(&mut self) {
        for y in (0..self.height - 1).rev() {
            for x in 0..self.width {
                let dish = self.dishes[y as usize][x as usize];
                let mut next_y: i32 = y as i32;
                if dish.dish_type == DishType::RoundedRock {
                    loop {
                        if next_y < self.height as i32 - 1 {
                            let dish = self.dishes[(next_y + 1) as usize][x as usize];
                            if dish.dish_type == DishType::Empty {
                                next_y += 1;
                                continue;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                if next_y != y as i32 {
                    self.dishes[y as usize][x as usize] = Dish {
                        dish_type: DishType::Empty,
                    };
                    self.dishes[next_y as usize][x as usize] = Dish {
                        dish_type: DishType::RoundedRock,
                    };
                }
            }
        }
    }

    fn slide_to_east(&mut self) {
        for y in 0..self.height {
            for x in (0..self.width - 1).rev() {
                let dish = self.dishes[y as usize][x as usize];
                let mut next_x: i32 = x as i32;
                if dish.dish_type == DishType::RoundedRock {
                    loop {
                        if next_x < self.width as i32 - 1 {
                            let dish = self.dishes[y as usize][(next_x + 1) as usize];
                            if dish.dish_type == DishType::Empty {
                                next_x += 1;
                                continue;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                if next_x != x as i32 {
                    self.dishes[y as usize][x as usize] = Dish {
                        dish_type: DishType::Empty,
                    };
                    self.dishes[y as usize][next_x as usize] = Dish {
                        dish_type: DishType::RoundedRock,
                    };
                }
            }
        }
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day14-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day14::Day14;

    #[test]
    fn part1_test_works() {
        let mut day = Day14::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "136")
    }

    #[test]
    fn part1_works() {
        let mut day = Day14::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "105784")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day14::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "64")
    }

    #[test]
    fn part2_works() {
        let mut day = Day14::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "91286")
    }
}
