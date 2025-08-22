// https://adventofcode.com/2023/day/14

use std::collections::HashMap;

use grid::Grid;
use itertools::Itertools;

use crate::utils::get_lines;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Rock {
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

#[derive(Debug, Default, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Input {
    rocks: Grid<Rock>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        rocks: parse_rocks(iter.next().unwrap().to_owned()),
    }
}

fn parse_rocks(rock_lines: Vec<String>) -> Grid<Rock> {
    let mut rocks = Grid::new(0, 0);
    for rock_line in rock_lines.into_iter() {
        let mut rock_entries: Vec<Rock> = Vec::new();
        for rock_entry in rock_line.chars() {
            match Rock::try_from(rock_entry as u8) {
                Ok(rock) => rock_entries.push(rock),
                Err(_) => panic!("Invalid rock"),
            }
        }
        rocks.push_row(rock_entries)
    }
    rocks
}

fn print_rock_vec(rock_vec: &Vec<Rock>) {
    println!("rock_vec:");
    for rock in rock_vec {
        print!("{:#}", *rock as u8 as char);
    }
    println!();
}

fn print_rocks(rocks: &Grid<Rock>) {
    println!("rocks:");
    for rock_row in rocks.iter_rows() {
        for rock in rock_row {
            print!("{:#}", *rock as u8 as char);
        }
        println!();
    }
}

fn rocks_to_str(rocks: &Grid<Rock>) -> String {
    let mut str: String = String::new();
    for rock_row in rocks.iter_rows() {
        for rock in rock_row {
            str += &(*rock as u8 as char).to_string();
        }
    }
    str
}

fn get_total_load(input_file: &str, dir: Direction, pre_cycle_len: u32, part_two: bool) -> u32 {
    let mut total_load: u32 = 0;

    let input = parse_input(input_file);

    if part_two {
        let mut rocks_states: HashMap<String, u32> = HashMap::new();
        let mut rocks = input.rocks.clone();
        let mut loads: Vec<u32> = vec![];
        for i in 0..pre_cycle_len {
            let (n_rocks, _n_load) = process_cycle(&rocks, Direction::North);
            rocks = n_rocks;
            let (w_rocks, _w_load) = process_cycle(&rocks, Direction::West);
            rocks = w_rocks;
            let (s_rocks, _s_load) = process_cycle(&rocks, Direction::South);
            rocks = s_rocks;
            let (e_rocks, e_load) = process_cycle(&rocks, Direction::East);
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

fn process_cycle(rocks: &Grid<Rock>, dir: Direction) -> (Grid<Rock>, u32) {
    let mut rocks_ord: Grid<Rock> = Grid::new(0, 0);

    match dir {
        Direction::North | Direction::South => {
            for col in rocks.iter_cols() {
                let mut rock_indices: Vec<usize> = vec![];
                for (pos, rock) in col.clone().enumerate() {
                    if *rock == Rock::Cube {
                        rock_indices.push(pos);
                    }
                }
                let col_vec = col.cloned().collect_vec();
                let split_col_vecs: Vec<Vec<Rock>> = col_vec
                    .split(|x| *x == Rock::Cube)
                    .map(|x| x.into())
                    .collect();
                let mut col_ord: Vec<Rock> = vec![];
                for mut split_col_vec in split_col_vecs {
                    split_col_vec.sort();
                    if dir == Direction::North {
                        split_col_vec.reverse();
                    }
                    col_ord.append(&mut split_col_vec);
                }

                for rock_index in rock_indices {
                    col_ord.insert(rock_index, Rock::Cube);
                }

                //print_rock_vec(&col_ord);
                rocks_ord.push_col(col_ord);
            }
        }
        Direction::East | Direction::West => {
            for row in rocks.iter_rows() {
                let mut rock_indices: Vec<usize> = vec![];
                for (pos, rock) in row.clone().enumerate() {
                    if *rock == Rock::Cube {
                        rock_indices.push(pos);
                    }
                }
                let row_vec = row.cloned().collect_vec();
                let split_row_vecs: Vec<Vec<Rock>> = row_vec
                    .split(|x| *x == Rock::Cube)
                    .map(|x| x.into())
                    .collect();
                let mut row_ord: Vec<Rock> = vec![];
                for mut split_row_vec in split_row_vecs {
                    split_row_vec.sort();
                    if dir == Direction::West {
                        split_row_vec.reverse();
                    }
                    row_ord.append(&mut split_row_vec);
                }

                for rock_index in rock_indices {
                    row_ord.insert(rock_index, Rock::Cube);
                }

                //print_rock_vec(&row_ord);
                rocks_ord.push_row(row_ord);
            }
        }
    }

    let mut total_load: u32 = 0;

    for ((row, _col), entry) in rocks_ord.indexed_iter() {
        if let Rock::Rounded = entry {
            let load = rocks_ord.rows() as u32 - row as u32;
            total_load += load;
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
            get_total_load("input/2023/day14_test01.txt", Direction::North, 0, false)
        );
    }

    #[test]
    fn test_get_total_load_part01_test02() {
        assert_eq!(
            66,
            get_total_load("input/2023/day14_test01.txt", Direction::South, 0, false)
        );
    }

    #[test]
    fn test_get_total_load_part01_test03() {
        assert_eq!(
            104,
            get_total_load("input/2023/day14_test01.txt", Direction::East, 0, false)
        );
    }

    #[test]
    fn test_get_total_load_part01_test04() {
        assert_eq!(
            104,
            get_total_load("input/2023/day14_test01.txt", Direction::West, 0, false)
        );
    }

    #[test]
    fn test_get_sum_reflections_part01() {
        assert_eq!(
            109345,
            get_total_load("input/2023/day14.txt", Direction::North, 0, false)
        );
    }

    #[test]
    fn test_get_total_load_part02_test01() {
        assert_eq!(
            64,
            get_total_load("input/2023/day14_test01.txt", Direction::North, 50, true)
        );
    }

    #[test]
    fn test_get_sum_reflections_part02() {
        assert_eq!(
            112452,
            get_total_load("input/2023/day14.txt", Direction::North, 1000, true)
        );
    }
}
