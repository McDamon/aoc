// https://adventofcode.com/2023/day/3

use std::collections::{HashMap, VecDeque};

use crate::utils::get_lines;

#[derive(Debug, Default)]
pub struct SchematicEntry {
    pub digit: Option<u32>,
    pub is_symbol: bool,
    pub is_gear: bool,
}

#[derive(Debug)]
pub struct Input {
    pub engine_schematic: Vec<Vec<SchematicEntry>>,
}

pub fn parse_input(input_file: &str, use_gear: bool) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        engine_schematic: parse_engine_schematic(iter.next().unwrap().to_owned(), use_gear),
    }
}

pub fn parse_engine_schematic(
    engine_schematic_lines: Vec<String>,
    use_gear: bool,
) -> Vec<Vec<SchematicEntry>> {
    let mut engine_schematic = Vec::new();
    for engine_schematic_line in engine_schematic_lines.into_iter() {
        let mut engine_schematic_entries: Vec<SchematicEntry> = Vec::new();
        for engine_schematic_entry in engine_schematic_line.chars() {
            match engine_schematic_entry {
                '0'..='9' => engine_schematic_entries.push(SchematicEntry {
                    digit: engine_schematic_entry.to_digit(10),
                    is_symbol: false,
                    is_gear: false,
                }),
                '.' => engine_schematic_entries.push(SchematicEntry {
                    digit: None,
                    is_symbol: false,
                    is_gear: false,
                }),
                '*' => {
                    if use_gear {
                        engine_schematic_entries.push(SchematicEntry {
                            digit: None,
                            is_symbol: false,
                            is_gear: true,
                        })
                    } else {
                        engine_schematic_entries.push(SchematicEntry {
                            digit: None,
                            is_symbol: true,
                            is_gear: false,
                        })
                    }
                }
                _ => engine_schematic_entries.push(SchematicEntry {
                    digit: None,
                    is_symbol: true,
                    is_gear: false,
                }),
            }
        }
        engine_schematic.push(engine_schematic_entries)
    }
    engine_schematic
}

pub fn get_sum_part_nums(input_file: &str) -> u32 {
    let mut sum_part_nums: u32 = 0;
    let mut num_queue = VecDeque::<u32>::new();
    let mut adj_sym_count: u32 = 0;

    let input = parse_input(input_file, false);
    for (row, row_entries) in input.engine_schematic.iter().enumerate() {
        for (col, entry) in row_entries.iter().enumerate() {
            match entry.digit {
                Some(digit) => {
                    adj_sym_count += num_adj_sym(row as i32, col as i32, &input.engine_schematic);
                    num_queue.push_back(digit);
                }
                None => {
                    let num_to_add = num_queue.iter().fold(0, |acc, elem| acc * 10 + elem);
                    if num_to_add > 0 && adj_sym_count > 0 {
                        sum_part_nums += num_to_add;
                        adj_sym_count = 0;
                    }
                    num_queue.clear();
                }
            }
        }
        // Handle the number at the end of the line if it exists
        let num_to_add = num_queue.iter().fold(0, |acc, elem| acc * 10 + elem);
        if num_to_add > 0 && adj_sym_count > 0 {
            sum_part_nums += num_to_add;
        }
        num_queue.clear();
        adj_sym_count = 0;
    }
    sum_part_nums
}

fn num_adj_sym(row: i32, col: i32, engine_schematic: &[Vec<SchematicEntry>]) -> u32 {
    let mut num_adj_sym: u32 = 0;
    for i in (row - 1)..(row + 2) {
        for j in (col - 1)..(col + 2) {
            let entry = engine_schematic
                .get(i as usize)
                .and_then(|r| r.get(j as usize));
            if let Some(entry) = entry
                && entry.is_symbol
            {
                num_adj_sym += 1;
            };
        }
    }
    num_adj_sym
}

pub fn get_sum_gear_ratios(input_file: &str) -> u32 {
    let mut sum_gear_ratios: u32 = 0;
    let mut num_queue = VecDeque::<u32>::new();
    let mut num_adj_stars = Vec::<(usize, usize)>::new();
    let mut gear_entries: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let input = parse_input(input_file, true);
    for (row, row_entries) in input.engine_schematic.iter().enumerate() {
        for (col, entry) in row_entries.iter().enumerate() {
            match entry.digit {
                Some(digit) => {
                    num_adj_stars.append(&mut add_adj_stars(
                        row as i32,
                        col as i32,
                        &input.engine_schematic,
                    ));
                    num_queue.push_back(digit);
                }
                None => {
                    let num_to_add = num_queue.iter().fold(0, |acc, elem| acc * 10 + elem);
                    num_adj_stars.dedup();
                    if num_to_add > 0 {
                        for star_pos in num_adj_stars.iter() {
                            gear_entries.entry(*star_pos).or_default().push(num_to_add);
                        }
                        num_adj_stars.clear();
                    }
                    num_queue.clear();
                }
            }
        }
        // Handle the number at the end of the line if it exists
        let num_to_add = num_queue.iter().fold(0, |acc, elem| acc * 10 + elem);
        num_adj_stars.dedup();
        if num_to_add > 0 {
            for star_pos in num_adj_stars.iter() {
                gear_entries.entry(*star_pos).or_default().push(num_to_add);
            }
        }
        num_adj_stars.clear();
        num_queue.clear();
    }

    for (_, values) in gear_entries.iter() {
        if values.len() == 2 {
            sum_gear_ratios += values[0] * values[1]
        }
    }
    sum_gear_ratios
}

fn add_adj_stars(
    row: i32,
    col: i32,
    engine_schematic: &[Vec<SchematicEntry>],
) -> Vec<(usize, usize)> {
    let mut num = Vec::<(usize, usize)>::new();
    for i in (row - 1)..(row + 2) {
        for j in (col - 1)..(col + 2) {
            let entry = engine_schematic
                .get(i as usize)
                .and_then(|r| r.get(j as usize));
            if let Some(entry) = entry
                && entry.is_gear
            {
                num.push((i as usize, j as usize));
            };
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_part_nums_test01() {
        assert_eq!(4361, get_sum_part_nums("input/2023/day03_test01.txt"));
    }

    #[test]
    fn test_get_sum_part_nums_test02() {
        assert_eq!(380, get_sum_part_nums("input/2023/day03_test02.txt"));
    }

    #[test]
    fn test_get_sum_part_nums() {
        assert_eq!(521601, get_sum_part_nums("input/2023/day03.txt"));
    }

    #[test]
    fn test_get_sum_gear_ratios_test01() {
        assert_eq!(467835, get_sum_gear_ratios("input/2023/day03_test01.txt"));
    }

    #[test]
    fn test_get_sum_gear_ratios() {
        assert_eq!(80694070, get_sum_gear_ratios("input/2023/day03.txt"));
    }
}
