// https://adventofcode.com/2023/day/14

use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{Direction, get_lines};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rock {
    #[default]
    Cube = b'#',
    Rounded = b'O',
    Empty = b'.',
}

impl TryFrom<u8> for Rock {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Rock::Cube as u8 => Ok(Rock::Cube),
            x if x == Rock::Rounded as u8 => Ok(Rock::Rounded),
            x if x == Rock::Empty as u8 => Ok(Rock::Empty),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Input {
    pub rocks: Vec<Vec<Rock>>,
}

pub fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        rocks: parse_rocks(iter.next().unwrap().to_owned()),
    }
}

pub fn parse_rocks(rock_lines: Vec<String>) -> Vec<Vec<Rock>> {
    let mut rocks = Vec::new();
    for rock_line in rock_lines.into_iter() {
        let mut rock_entries: Vec<Rock> = Vec::new();
        for rock_entry in rock_line.chars() {
            match Rock::try_from(rock_entry as u8) {
                Ok(rock) => rock_entries.push(rock),
                Err(_) => panic!("Invalid rock"),
            }
        }
        rocks.push(rock_entries)
    }
    rocks
}

pub fn print_rocks(rocks: &[Vec<Rock>]) {
    println!("rocks:");
    for rock_row in rocks.iter() {
        for rock in rock_row {
            print!("{:#}", *rock as u8 as char);
        }
        println!();
    }
}

pub fn rocks_to_str(rocks: &[Vec<Rock>]) -> String {
    let mut str: String = String::new();
    for rock_row in rocks.iter() {
        for rock in rock_row {
            str += &(*rock as u8 as char).to_string();
        }
    }
    str
}

pub fn get_total_load(input_file: &str, dir: Direction, pre_cycle_len: u32, part_two: bool) -> u32 {
    let mut total_load: u32 = 0;

    let input = parse_input(input_file);

    if part_two {
        let mut rocks_states: HashMap<String, u32> = HashMap::new();
        let mut rocks = input.rocks.clone();
        let mut loads: Vec<u32> = vec![];
        for i in 0..pre_cycle_len {
            let (n_rocks, _n_load) = process_cycle(&rocks, Direction::N);
            rocks = n_rocks;
            let (w_rocks, _w_load) = process_cycle(&rocks, Direction::W);
            rocks = w_rocks;
            let (s_rocks, _s_load) = process_cycle(&rocks, Direction::S);
            rocks = s_rocks;
            let (e_rocks, e_load) = process_cycle(&rocks, Direction::E);
            rocks = e_rocks;

            let rocks_str = rocks_to_str(&rocks);

            loads.push(e_load);

            if rocks_states.contains_key(&rocks_str) {
                let remaining = (1_000_000_000 - i) % (i - rocks_states.get(&rocks_str).unwrap());
                println!("Found same state at {:?}, remaining {:?}", i, remaining);
                if remaining == 1 {
                    break;
                }
            }

            rocks_states.insert(rocks_str, i);
        }

        total_load = *loads.last().unwrap()
    } else {
        let (n_rocks, load) = process_cycle(&input.rocks, dir);
        print_rocks(&n_rocks);
        total_load += load;
    }

    total_load
}

fn iter_cols<T>(matrix: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut cols = vec![vec![]; matrix[0].len()];
    for row in matrix.iter() {
        for (i, item) in row.iter().enumerate() {
            cols[i].push(item.clone());
        }
    }
    cols
}

fn process_cycle(rocks: &[Vec<Rock>], dir: Direction) -> (Vec<Vec<Rock>>, u32) {
    let mut rocks_ord: Vec<Vec<Rock>> = Vec::new();

    match dir {
        Direction::N | Direction::S => {
            for col in iter_cols(rocks) {
                let mut rock_indices: Vec<usize> = vec![];
                for (pos, rock) in col.iter().enumerate() {
                    if *rock == Rock::Cube {
                        rock_indices.push(pos);
                    }
                }
                let col_vec = col.iter().cloned().collect_vec();
                let split_col_vecs: Vec<Vec<Rock>> = col_vec
                    .split(|x| *x == Rock::Cube)
                    .map(|x| x.into())
                    .collect();
                let mut col_ord: Vec<Rock> = vec![];
                for mut split_col_vec in split_col_vecs {
                    split_col_vec.sort();
                    if dir == Direction::N {
                        split_col_vec.reverse();
                    }
                    col_ord.append(&mut split_col_vec);
                }

                for rock_index in rock_indices {
                    col_ord.insert(rock_index, Rock::Cube);
                }

                rocks_ord.push(col_ord);
            }
        }
        Direction::E | Direction::W => {
            for row in rocks.iter() {
                let mut rock_indices: Vec<usize> = vec![];
                for (pos, rock) in row.clone().iter().enumerate() {
                    if *rock == Rock::Cube {
                        rock_indices.push(pos);
                    }
                }
                let row_vec = row.iter().cloned().collect_vec();
                let split_row_vecs: Vec<Vec<Rock>> = row_vec
                    .split(|x| *x == Rock::Cube)
                    .map(|x| x.into())
                    .collect();
                let mut row_ord: Vec<Rock> = vec![];
                for mut split_row_vec in split_row_vecs {
                    split_row_vec.sort();
                    if dir == Direction::W {
                        split_row_vec.reverse();
                    }
                    row_ord.append(&mut split_row_vec);
                }

                for rock_index in rock_indices {
                    row_ord.insert(rock_index, Rock::Cube);
                }

                rocks_ord.push(row_ord);
            }
        }
    }

    let mut total_load: u32 = 0;

    for (row, row_vec) in rocks_ord.iter().enumerate() {
        for entry in row_vec.iter() {
            if let Rock::Rounded = entry {
                let load = rocks_ord.len() as u32 - row as u32;
                total_load += load;
            }
        }
    }

    (rocks_ord, total_load)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_load_part01_test01() {
        assert_eq!(
            136,
            get_total_load("input/2023/day14_test01.txt", Direction::N, 0, false)
        );
    }

    #[test]
    fn test_get_total_load_part01_test02() {
        assert_eq!(
            66,
            get_total_load("input/2023/day14_test01.txt", Direction::S, 0, false)
        );
    }

    #[test]
    fn test_get_total_load_part01_test03() {
        assert_eq!(
            104,
            get_total_load("input/2023/day14_test01.txt", Direction::E, 0, false)
        );
    }

    #[test]
    fn test_get_total_load_part01_test04() {
        assert_eq!(
            104,
            get_total_load("input/2023/day14_test01.txt", Direction::W, 0, false)
        );
    }

    #[test]
    fn test_get_sum_reflections_part01() {
        assert_eq!(
            109345,
            get_total_load("input/2023/day14.txt", Direction::N, 0, false)
        );
    }

    #[test]
    fn test_get_total_load_part02_test01() {
        assert_eq!(
            64,
            get_total_load("input/2023/day14_test01.txt", Direction::N, 50, true)
        );
    }

    #[test]
    fn test_get_sum_reflections_part02() {
        assert_eq!(
            112452,
            get_total_load("input/2023/day14.txt", Direction::N, 1000, true)
        );
    }
}
