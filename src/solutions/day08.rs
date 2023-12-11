use std::collections::HashMap;

use aoc_2023_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day08.txt";

#[derive(Debug, Default)]
pub struct Day08 {
    document: Document,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day08 {
    fn name(&self) -> (usize, usize) {
        (2023, 8)
    }

    fn parse(&mut self, input: Option<&str>) {
        let input = read_lines(input.unwrap_or(INPUT));
        self.document.instructions = input
            .first()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => panic!("Invalid instruction"),
            })
            .collect();
        for line in input {
            if line.contains('(') {
                let (node, nodes) = line.split_once(" = ").unwrap();

                let nodes = nodes
                    .chars()
                    .skip(1)
                    .take(nodes.len() - 2)
                    .collect::<String>();
                let (node_a, node_b) = nodes.split_once(", ").unwrap();
                self.document
                    .nodes
                    .insert(Node::new(node), (Node::new(node_a), Node::new(node_b)));
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.document.find_zzz_node())
    }

    fn part2(&mut self) -> Vec<String> {
        output(self.document.find_simultaneously_all_nodes_ending_with_z())
    }
}

// ---------------------------------------------------

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    name: String,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name[..self.name.len() - 1])
    }
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn is_leaf(&self) -> bool {
        self.name.as_str() == "ZZZ"
    }
}

#[derive(Debug, Default)]
struct Document {
    instructions: Vec<Instruction>,
    nodes: HashMap<Node, (Node, Node)>,
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instruction in &self.instructions {
            match instruction {
                Instruction::Left => write!(f, "left ")?,
                Instruction::Right => write!(f, "right ")?,
            }
        }
        writeln!(f)?;
        for (name, node) in &self.nodes {
            write!(f, "{} {} {} ", name, node.0, node.1)?;
        }
        Ok(())
    }
}

impl Document {
    fn get_all_node_names(&self) -> Vec<String> {
        self.nodes
            .keys()
            .map(|node| node.name.clone())
            .collect::<Vec<String>>()
    }

    fn get_all_nodes_ending_with_a(&self) -> Vec<&Node> {
        self.nodes
            .keys()
            .filter(|node| node.name.ends_with('A'))
            .collect()
    }

    fn find_simultaneously_all_nodes_ending_with_z(&self) -> u32 {
        let current_nodes = self.get_all_nodes_ending_with_a();
        let all_nodes: Vec<String> = self.get_all_node_names();
        let mut node_positions: Vec<[u16; 2]> = Vec::new();
        for nodes in self.nodes.values() {
            let node_left = all_nodes.iter().position(|n| n == &nodes.0.name).unwrap();
            let node_right = all_nodes.iter().position(|n| n == &nodes.1.name).unwrap();
            node_positions.push([node_left as u16, node_right as u16]);
        }
        let mut current_node_positions: Vec<u16> = Vec::new();
        for node in current_nodes.iter() {
            let node_position = all_nodes.iter().position(|n| n == &node.name).unwrap();
            current_node_positions.push(node_position as u16);
        }

        let instructions: Vec<usize> = self
            .instructions
            .iter()
            .map(|instruction| match instruction {
                Instruction::Left => 0,
                Instruction::Right => 1,
            })
            .collect();

        let mut steps = 0;
        for instruction in instructions.iter().cycle() {
            current_node_positions = current_node_positions
                .iter()
                .map(|node| node_positions[*node as usize][*instruction])
                .collect();
            steps += 1;
            if current_node_positions
                .iter()
                .all(|node| all_nodes[*node as usize].ends_with('Z'))
            {
                return steps;
            }
        }
        steps
    }

    fn find_zzz_node(&self) -> u32 {
        let mut current_node = Node::new("AAA");
        let mut steps = 0;
        for instruction in self.instructions.iter().cycle() {
            match instruction {
                Instruction::Left => {
                    current_node = self.nodes.get(&current_node).unwrap().0.clone();
                }
                Instruction::Right => {
                    current_node = self.nodes.get(&current_node).unwrap().1.clone();
                }
            }
            steps += 1;
            if current_node.is_leaf() {
                return steps;
            }
        }
        steps
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "input/day08-test.txt";
    const TEST_INPUT_2: &str = "input/day08-test2.txt";
    const TEST_INPUT_3: &str = "input/day08-test3.txt";
    use aoc_2023_rust::Runner;

    use crate::solutions::day08::Day08;

    #[test]
    fn part1_test_works() {
        let mut day = Day08::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "2")
    }

    #[test]
    fn part1_test2_works() {
        let mut day = Day08::new();
        day.parse(Some(TEST_INPUT_2));
        let output = day.part1();
        assert_eq!(output[0], "6")
    }

    #[test]
    fn part1_works() {
        let mut day = Day08::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "17621")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day08::new();
        day.parse(Some(TEST_INPUT_3));
        let output = day.part2();
        assert_eq!(output[0], "6")
    }

    #[test]
    fn part2_works() {
        let mut day = Day08::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "245794069")
    }
}
