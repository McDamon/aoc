// https://adventofcode.com/2022/day/5

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str};

use crate::utils::get_lines;

#[derive(Debug, PartialEq)]
pub struct Move {
    num_to_move: u32,
    from: u32,
    to: u32,
}

#[derive(Debug, PartialEq)]
pub struct Input {
    crates: HashMap<u32, Vec<char>>,
    moves: Vec<Move>,
}

pub fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        crates: parse_crates(iter.next().unwrap().to_owned()),
        moves: parse_moves(iter.next().unwrap().to_owned()),
    }
}

pub fn parse_crates(crates_lines: Vec<String>) -> HashMap<u32, Vec<char>> {
    lazy_static! {
        static ref RE_INDEX: Regex = Regex::new(r"(?P<index>[1-9])").unwrap();
        static ref RE_CRATE: Regex = Regex::new(r"(?P<crate>[A-Z])").unwrap();
    }

    let crates_lines_rev: Vec<String> = crates_lines.into_iter().rev().collect();

    let mut crates: HashMap<u32, Vec<char>> = HashMap::new();

    for crate_line in crates_lines_rev.into_iter() {
        let table_row = crate_line
            .as_bytes()
            .chunks(4)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        for (i, table_col) in table_row.iter().enumerate() {
            let caps_index = RE_INDEX.captures(table_col);
            let caps_crates = RE_CRATE.captures(table_col);

            if let Some(caps_index) = caps_index {
                let caps_index = caps_index["index"]
                    .chars()
                    .next()
                    .expect("the string is empty")
                    .to_digit(10)
                    .unwrap();
                crates.insert(caps_index, vec![]);
            }

            if let Some(caps_crates) = caps_crates {
                let crate_char = caps_crates["crate"]
                    .chars()
                    .next()
                    .expect("the string is empty");
                crates
                    .entry(i as u32 + 1)
                    .and_modify(|vec| vec.push(crate_char));
            }
        }
    }

    crates
}

fn parse_moves(moves_lines: Vec<String>) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    for moves_line in moves_lines.into_iter() {
        let mut moves_line_split = moves_line.split(' ');

        moves_line_split.next();
        let num_to_move_val = moves_line_split.next();
        moves_line_split.next();
        let from_val = moves_line_split.next();
        moves_line_split.next();
        let to_val = moves_line_split.next();

        moves.push(Move {
            num_to_move: num_to_move_val.unwrap().parse().unwrap(),
            from: from_val.unwrap().parse().unwrap(),
            to: to_val.unwrap().parse().unwrap(),
        })
    }

    moves
}

pub fn get_result_stack(crates: HashMap<u32, Vec<char>>) -> String {
    let mut result: Vec<char> = vec![];

    for key in crates.keys().sorted() {
        let stack = crates.get(key);
        if let Some(stack) = stack {
            let result_char = stack.last();
            if let Some(result_char) = result_char {
                result.push(*result_char)
            }
        }
    }

    result.iter().collect()
}

pub fn get_supply_stack(input_file: &str) -> String {
    let mut input = parse_input(input_file);

    for move_step in input.moves {
        let moves: Vec<u32> = (0..move_step.num_to_move).collect();
        for i in moves {
            println!(
                "{0}: moving item from stack {1} to stack {2}",
                i, move_step.from, move_step.to
            );

            let from_stack = input.crates.get_mut(&move_step.from);

            let mut pop_value: Option<char> = None;

            if let Some(from_stack) = from_stack {
                pop_value = from_stack.pop();
            }

            let to_stack = input.crates.get_mut(&move_step.to);

            if let Some(to_stack) = to_stack
                && let Some(pop_value) = pop_value
            {
                to_stack.push(pop_value);
            }
        }
    }

    println!("{:?}", input.crates);

    get_result_stack(input.crates)
}

pub fn get_supply_stack_alt(input_file: &str) -> String {
    let mut input = parse_input(input_file);

    for move_step in input.moves {
        println!("BEFORE: {:?}", input.crates);

        let from_stack = input.crates.get_mut(&move_step.from);

        let mut drainage_values: Option<Vec<char>> = None;

        if let Some(from_stack) = from_stack {
            let final_length = from_stack
                .len()
                .saturating_sub(move_step.num_to_move as usize);
            drainage_values = Some(from_stack.split_off(final_length));
        }

        let to_stack = input.crates.get_mut(&move_step.to);

        if let Some(to_stack) = to_stack
            && let Some(drainage_values) = drainage_values
        {
            to_stack.extend(drainage_values);
        }

        println!("AFTER: {:?}", input.crates);
    }

    get_result_stack(input.crates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let expected_crates = HashMap::from([
            (1, vec!['B', 'P', 'N', 'Q', 'H', 'D', 'R', 'T']),
            (2, vec!['W', 'G', 'B', 'J', 'T', 'V']),
            (3, vec!['N', 'R', 'H', 'D', 'S', 'V', 'M', 'Q']),
            (4, vec!['P', 'Z', 'N', 'M', 'C']),
            (5, vec!['D', 'Z', 'B']),
            (6, vec!['V', 'C', 'W', 'Z']),
            (7, vec!['G', 'Z', 'N', 'C', 'V', 'Q', 'L', 'S']),
            (8, vec!['L', 'G', 'J', 'M', 'D', 'N', 'V']),
            (9, vec!['T', 'P', 'M', 'F', 'Z', 'C', 'G']),
        ]);
        let input = parse_input("input/2022/day05.txt");
        assert_eq!(expected_crates, input.crates);
    }

    #[test]
    fn test_get_supply_stack_sample() {
        assert_eq!("CMZ", get_supply_stack("input/2022/day05_test01.txt"));
    }

    #[test]
    fn test_get_supply_stack() {
        assert_eq!("ZBDRNPMVH", get_supply_stack("input/2022/day05.txt"));
    }

    #[test]
    fn test_get_supply_stack_alt_sample() {
        assert_eq!("MCD", get_supply_stack_alt("input/2022/day05_test01.txt"));
    }

    #[test]
    fn test_get_supply_stack_alt() {
        assert_eq!("WDLPFNNNB", get_supply_stack_alt("input/2022/day05.txt"));
    }
}
