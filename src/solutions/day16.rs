use std::{collections::HashSet, path::Path};

use aoc_2023_rust::{output, read_lines, read_lines_from_string, Point, Runner};

const INPUT: &str = "input/day16.txt";

#[derive(Debug, Default)]
pub struct Day16 {
    contraption: Contraption,
}

impl Day16 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day16 {
    fn name(&self) -> (usize, usize) {
        (2023, 16)
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

        self.contraption.width = puzzle_input[0].len() as u16;
        self.contraption.height = puzzle_input.len() as u16;

        for (y, line) in puzzle_input.iter().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let position = Point::new(x as i16, y as i16);
                let contraption_type = match c {
                    '.' => ContraptionType::Empty,
                    '/' => ContraptionType::MirrorRight,
                    '\\' => ContraptionType::MirrorLeft,
                    '|' => ContraptionType::SplitterV,
                    '-' => ContraptionType::SplitterH,
                    _ => panic!("Unknown contraption type"),
                };
                row.push(Tile {
                    position,
                    contraption_type,
                    is_energized: false,
                });
            }
            self.contraption.tiles.push(row);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.contraption
                .run(vec![(Point::new(0, 0), vec![Direction::Right])]),
        )
    }
    fn part2(&mut self) -> Vec<String> {
        output(self.contraption.run(generate_edge_tiles(
            self.contraption.width,
            self.contraption.height,
        )))
    }
}

// ---------------------------------------------------

fn generate_edge_tiles(width: u16, height: u16) -> Vec<(Point<i16>, Vec<Direction>)> {
    let mut edge_tiles = Vec::new();
    for y in [0, height - 1].iter() {
        for x in 0..width {
            if *y == 0 && x == 0 {
                edge_tiles.push((
                    Point::new(x as i16, *y as i16),
                    vec![Direction::Right, Direction::Down],
                ));
            } else if *y == 0 && x == width - 1 {
                edge_tiles.push((
                    Point::new(x as i16, *y as i16),
                    vec![Direction::Left, Direction::Down],
                ));
            } else if *y == height - 1 && x == 0 {
                edge_tiles.push((
                    Point::new(x as i16, *y as i16),
                    vec![Direction::Right, Direction::Up],
                ));
            } else if *y == height - 1 && x == width - 1 {
                edge_tiles.push((
                    Point::new(x as i16, *y as i16),
                    vec![Direction::Left, Direction::Up],
                ));
            } else if *y == 0 {
                edge_tiles.push((Point::new(x as i16, *y as i16), vec![Direction::Down]));
            } else {
                edge_tiles.push((Point::new(x as i16, *y as i16), vec![Direction::Up]));
            }
        }
    }

    for x in [0, width - 1].iter() {
        for y in 1..height - 1 {
            if *x == 0 {
                edge_tiles.push((Point::new(*x as i16, y as i16), vec![Direction::Right]));
            } else {
                edge_tiles.push((Point::new(*x as i16, y as i16), vec![Direction::Left]));
            }
        }
    }
    edge_tiles
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ContraptionType {
    Empty,
    MirrorRight,
    MirrorLeft,
    SplitterV,
    SplitterH,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tile {
    position: Point<i16>,
    contraption_type: ContraptionType,
    is_energized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    position: Point<i16>,
    direction: Direction,
}

impl Beam {
    fn next_position(&self, width: u16, height: u16) -> Option<Point<i16>> {
        let mut next_position = self.position;
        match self.direction {
            Direction::Up => {
                next_position.y -= 1;
            }
            Direction::Down => {
                next_position.y += 1;
            }
            Direction::Left => {
                next_position.x -= 1;
            }
            Direction::Right => {
                next_position.x += 1;
            }
        }
        if next_position.x < 0
            || next_position.x >= width as i16
            || next_position.y < 0
            || next_position.y >= height as i16
        {
            None
        } else {
            Some(next_position)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Contraption {
    tiles: Vec<Vec<Tile>>,
    beams: Vec<Beam>,
    width: u16,
    height: u16,
}

impl std::fmt::Display for Contraption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for row in self.tiles.iter() {
            for tile in row.iter() {
                if tile.is_energized {
                    output.push('#');
                } else {
                    match tile.contraption_type {
                        ContraptionType::Empty => output.push('.'),
                        ContraptionType::MirrorRight => output.push('/'),
                        ContraptionType::MirrorLeft => output.push('\\'),
                        ContraptionType::SplitterV => output.push('|'),
                        ContraptionType::SplitterH => output.push('-'),
                    }
                }
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

impl Contraption {
    fn get_tile_type(&self, position: Point<i16>) -> ContraptionType {
        self.tiles[position.y as usize][position.x as usize].contraption_type
    }

    fn energize_tile(&mut self, position: Point<i16>) {
        self.tiles[position.y as usize][position.x as usize].is_energized = true;
    }

    fn add_first_beam(&mut self, position: Point<i16>, direction: Direction) {
        self.beams.push(Beam {
            position,
            direction,
        });
    }

    fn calculate_energized_tiles(&self) -> u32 {
        self.tiles
            .iter()
            .flatten()
            .filter(|t| t.is_energized)
            .count() as u32
    }

    fn revert_energized_tiles(&mut self) {
        for row in self.tiles.iter_mut() {
            for tile in row.iter_mut() {
                tile.is_energized = false;
            }
        }
    }

    fn run(&mut self, starting_tiles: Vec<(Point<i16>, Vec<Direction>)>) -> u32 {
        let mut max_energized_tiles = 0;
        for starting_tile in &starting_tiles {
            self.add_first_beam(starting_tile.0, starting_tile.1[0]);
            let mut created_beams = HashSet::new();
            let mut same_energized_tiles = 0;
            let mut total_energized_tiles = 0;
            loop {
                let mut beans_to_be_removed = Vec::new();
                for beam_i in 0..self.beams.len() {
                    let mut new_bean = Vec::new();
                    created_beams.insert(self.beams[beam_i]);
                    let current_direction = &self.beams[beam_i].direction;
                    let current_position = self.beams[beam_i].position;
                    match self.get_tile_type(current_position) {
                        ContraptionType::Empty => {}
                        ContraptionType::MirrorRight => match current_direction {
                            Direction::Up => {
                                self.beams[beam_i].direction = Direction::Right;
                            }
                            Direction::Down => {
                                self.beams[beam_i].direction = Direction::Left;
                            }
                            Direction::Left => {
                                self.beams[beam_i].direction = Direction::Down;
                            }
                            Direction::Right => {
                                self.beams[beam_i].direction = Direction::Up;
                            }
                        },
                        ContraptionType::MirrorLeft => match current_direction {
                            Direction::Up => {
                                self.beams[beam_i].direction = Direction::Left;
                            }
                            Direction::Down => {
                                self.beams[beam_i].direction = Direction::Right;
                            }
                            Direction::Left => {
                                self.beams[beam_i].direction = Direction::Up;
                            }
                            Direction::Right => {
                                self.beams[beam_i].direction = Direction::Down;
                            }
                        },
                        ContraptionType::SplitterV => match current_direction {
                            Direction::Up => {}
                            Direction::Down => {}
                            Direction::Left => {
                                let beam = Beam {
                                    position: current_position,
                                    direction: Direction::Up,
                                };
                                if created_beams.insert(beam) {
                                    new_bean.push(beam);
                                }
                                self.beams[beam_i].direction = Direction::Down;
                            }
                            Direction::Right => {
                                let beam = Beam {
                                    position: current_position,
                                    direction: Direction::Up,
                                };
                                if created_beams.insert(beam) {
                                    new_bean.push(beam);
                                }
                                self.beams[beam_i].direction = Direction::Down;
                            }
                        },
                        ContraptionType::SplitterH => match current_direction {
                            Direction::Up => {
                                let beam = Beam {
                                    position: current_position,
                                    direction: Direction::Left,
                                };
                                if !created_beams.contains(&beam) {
                                    new_bean.push(beam);
                                }
                                self.beams[beam_i].direction = Direction::Right;
                            }
                            Direction::Down => {
                                let beam = Beam {
                                    position: current_position,
                                    direction: Direction::Left,
                                };
                                if created_beams.insert(beam) {
                                    new_bean.push(beam);
                                }
                                self.beams[beam_i].direction = Direction::Right;
                            }
                            Direction::Left => {}
                            Direction::Right => {}
                        },
                    }

                    self.energize_tile(current_position);

                    if !new_bean.is_empty() {
                        if let Some(pos) = new_bean[0].next_position(self.width, self.height) {
                            self.beams.push(Beam {
                                position: pos,
                                direction: new_bean[0].direction,
                            });
                        }
                    }
                    if let Some(pos) = self.beams[beam_i].next_position(self.width, self.height) {
                        self.beams[beam_i].position = pos;
                    } else {
                        beans_to_be_removed.push(beam_i);
                    }
                }

                for i in beans_to_be_removed.iter().rev() {
                    self.beams.remove(*i);
                }

                let energized_tiles = self.calculate_energized_tiles();

                if total_energized_tiles == energized_tiles && same_energized_tiles == 10 {
                    self.revert_energized_tiles();
                    self.beams.clear();
                    max_energized_tiles = max_energized_tiles.max(total_energized_tiles);
                    break;
                } else if total_energized_tiles == energized_tiles {
                    same_energized_tiles += 1;
                } else {
                    total_energized_tiles = energized_tiles;
                }
            }
        }
        max_energized_tiles
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day16-test.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day16::Day16;

    #[test]
    fn part1_test_works() {
        let mut day = Day16::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "46")
    }

    #[test]
    fn part1_works() {
        let mut day = Day16::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "8901")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day16::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "51")
    }

    #[test]
    fn part2_works() {
        let mut day = Day16::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "9064")
    }
}
