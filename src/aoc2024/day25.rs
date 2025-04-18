// https://adventofcode.com/2024/day/25

use std::vec;

use crate::utils::get_lines;

#[derive(Debug, PartialEq, Clone, Copy)]
enum LockPin {
    Empty,
    Filled,
}

struct Input {
    schematics: Vec<Vec<Vec<LockPin>>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut schematics = vec![]; // Declare schematics as mutable

    lines
        .split(|line| line.trim().is_empty())
        .for_each(|schematic_str| {
            let schematic: Vec<Vec<LockPin>> = schematic_str
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => LockPin::Empty,
                            '#' => LockPin::Filled,
                            _ => panic!("Unexpected character: {}", c),
                        })
                        .collect()
                })
                .collect();

            schematics.push(schematic);
        });

    Input { schematics }
}

pub fn get_unique_lock_key_pairs(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut locks: Vec<Vec<[LockPin; 5]>> = vec![];
    let mut keys: Vec<Vec<[LockPin; 5]>> = vec![];

    for schematic in input.schematics {
        if let Some(first_line) = schematic.first() {
            if first_line.iter().all(|&pin| pin == LockPin::Filled) {
                locks.push(
                    schematic
                        .iter()
                        .map(|line| {
                            line.as_slice()
                                .try_into()
                                .expect("Each line must have exactly 5 elements")
                        })
                        .collect(),
                );
            }
        }
        if let Some(last_line) = schematic.last() {
            if last_line.iter().all(|&pin| pin == LockPin::Filled) {
                keys.push(
                    schematic
                        .iter()
                        .map(|line| {
                            line.as_slice()
                                .try_into()
                                .expect("Each line must have exactly 5 elements")
                        })
                        .collect(),
                );
            }
        }
    }

    println!("Locks: {:?}", locks);
    println!("Keys: {:?}", keys);

    let unique_lock_key_pairs = 0;

    unique_lock_key_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unique_lock_key_pairs_test01() {
        assert_eq!(3, get_unique_lock_key_pairs("input/2024/day25_test01.txt"));
    }

    #[test]
    fn test_get_unique_lock_key_pairs_test02() {
        assert_eq!(0, get_unique_lock_key_pairs("input/2024/day25_test02.txt"));
    }

    #[test]
    fn test_get_unique_lock_key_pairs() {
        assert_eq!(0, get_unique_lock_key_pairs("input/2024/day25.txt"));
    }
}
