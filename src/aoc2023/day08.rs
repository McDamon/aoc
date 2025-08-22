// https://adventofcode.com/2023/day/8

use regex::Regex;

use crate::utils::get_lines;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Input {
    pub instructions: Vec<Instruction>,
    pub tree_node_names: Vec<String>,
    pub tree_nodes: HashMap<String, (String, String)>,
}

pub fn parse_input(input_file: &str) -> Input {
    lazy_static! {
        static ref RE_TREENODE: Regex =
            Regex::new(r"(?P<root>\w+)\s+=\s+\((?P<left>\w+),\s+(?P<right>\w+)\)").unwrap();
    }

    let lines = get_lines(input_file);

    let instructions = parse_instructions(lines.first().unwrap());

    let mut tree_node_names: Vec<String> = vec![];
    let mut tree_nodes: HashMap<String, (String, String)> = HashMap::new();

    for line in lines.into_iter().skip(2) {
        let caps_tree_nodes = RE_TREENODE.captures(&line);
        if let Some(caps_tree_nodes) = caps_tree_nodes {
            let root = caps_tree_nodes["root"].to_string();
            let left = caps_tree_nodes["left"].to_string();
            let right = caps_tree_nodes["right"].to_string();
            tree_node_names.push(root.clone());
            tree_nodes.insert(root, (left, right));
        }
    }

    tree_node_names.sort();

    Input {
        instructions,
        tree_node_names,
        tree_nodes,
    }
}

fn parse_instructions(instructions_str: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for instruction_str in instructions_str.chars() {
        instructions.push(parse_instruction(instruction_str));
    }
    instructions
}

fn parse_instruction(instruction_char: char) -> Instruction {
    match instruction_char {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => panic!("Invalid instruction: {}", instruction_char),
    }
}

pub fn get_num_steps(input_file: &str) -> u64 {
    let mut num_steps: u64 = 0;
    let input = parse_input(input_file);
    let mut current_node = Some(input.tree_node_names.first().unwrap().clone());
    for instruction in input.instructions.iter().cycle() {
        current_node = traverse_tree(
            &input.tree_nodes,
            current_node.unwrap(),
            input.tree_node_names.last().unwrap().clone(),
            instruction,
        );
        num_steps += 1;
        if current_node.is_none() {
            break;
        }
    }

    num_steps
}

pub fn get_num_steps_end_with_z(input_file: &str) -> u64 {
    let mut num_steps_vec: Vec<u64> = vec![];

    let input = parse_input(input_file);
    let start_nodes: Vec<String> = input
        .tree_node_names
        .into_iter()
        .filter(|e| e.ends_with('A'))
        .collect();
    for start_node in start_nodes {
        let mut num_steps: u64 = 0;
        let mut current_node = Some(start_node);
        for instruction in input.instructions.iter().cycle() {
            current_node =
                traverse_tree_ends_with_z(&input.tree_nodes, current_node.unwrap(), instruction);
            num_steps += 1;
            if current_node.is_none() {
                num_steps_vec.push(num_steps);
                break;
            }
        }
    }

    num_steps_vec.into_iter().fold(1, lcm)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn traverse_tree(
    tree_nodes: &HashMap<String, (String, String)>,
    current_node: String,
    leaf_node: String,
    instruction: &Instruction,
) -> Option<String> {
    match tree_nodes.get(&current_node) {
        Some((left, right)) => {
            if *instruction == Instruction::Left && *current_node != *left && *left != leaf_node {
                return Some(left.clone());
            } else if *instruction == Instruction::Right
                && *current_node != *right
                && *right != leaf_node
            {
                return Some(right.clone());
            }
        }
        None => panic!("Invalid current_node"),
    }
    None
}

fn traverse_tree_ends_with_z(
    tree_nodes: &HashMap<String, (String, String)>,
    current_node: String,
    instruction: &Instruction,
) -> Option<String> {
    match tree_nodes.get(&current_node) {
        Some((left, right)) => {
            if *instruction == Instruction::Left
                && *current_node != *left
                && !left.ends_with('Z')
                && !current_node.ends_with('Z')
            {
                return Some(left.clone());
            } else if *instruction == Instruction::Right
                && *current_node != *right
                && !right.ends_with('Z')
                && !current_node.ends_with('Z')
            {
                return Some(right.clone());
            }
        }
        None => panic!("Invalid current_node"),
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_steps_test01() {
        assert_eq!(2, get_num_steps("input/2023/day08_test01.txt"));
    }

    #[test]
    fn test_get_num_steps_test02() {
        assert_eq!(6, get_num_steps("input/2023/day08_test02.txt"));
    }

    #[test]
    fn test_get_num_steps() {
        assert_eq!(21883, get_num_steps("input/2023/day08.txt"));
    }

    #[test]
    fn test_get_num_steps_end_with_z_test03() {
        assert_eq!(6, get_num_steps_end_with_z("input/2023/day08_test03.txt"));
    }

    #[test]
    fn test_get_num_steps_end_with_z() {
        assert_eq!(
            12833235391111,
            get_num_steps_end_with_z("input/2023/day08.txt")
        );
    }
}
