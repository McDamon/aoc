// https://adventofcode.com/2024/day/8

use std::collections::{HashMap, HashSet};

use super::utils::get_lines;

struct Input {
    map: Vec<Vec<char>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let map_line: Vec<char> = line.chars().collect();
        map.push(map_line);
    }

    Input { map }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Node {
    pos: (usize, usize),
    val: char,
}

fn print_map(map: &[Vec<char>]) {
    println!("Grid:");
    for map_row in map.iter() {
        for map_entry in map_row {
            print!("{:#}", *map_entry);
        }
        println!();
    }
}

fn get_node_map(map: &[Vec<char>]) -> HashMap<char, Vec<Node>> {
    map.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c != '.')
                .map(move |(j, &val)| (val, Node { pos: (i, j), val }))
        })
        .fold(HashMap::new(), |mut acc, (key, node)| {
            acc.entry(key).or_default().push(node);
            acc
        })
}

fn get_antinode_pos(
    (row, col): (usize, usize),
    (other_row, other_col): (usize, usize),
    antinode_map: &[Vec<char>],
) -> Option<(usize, usize)> {
    let row_diff = row as i32 - other_row as i32;
    let col_diff = col as i32 - other_col as i32;
    if row_diff.is_negative() && col_diff.is_negative() {
        let new_row = row as i32 - row_diff.abs();
        let new_col = col as i32 - col_diff.abs();
        if new_row < 0 || new_col < 0 {
            None
        } else {
            Some((new_row as usize, new_col as usize))
        }
    } else if row_diff.is_negative() && col_diff.is_positive() {
        let new_row = row as i32 - row_diff.abs();
        let new_col = col as i32 + col_diff.abs();
        if new_row < 0 || new_col >= antinode_map[0].len() as i32 {
            None
        } else {
            Some((new_row as usize, new_col as usize))
        }
    } else if row_diff.is_positive() && col_diff.is_negative() {
        let new_row = row as i32 + row_diff.abs();
        let new_col = col as i32 - col_diff.abs();
        if new_row >= antinode_map.len() as i32 || new_col < 0 {
            None
        } else {
            Some((new_row as usize, new_col as usize))
        }
    } else if row_diff.is_positive() && col_diff.is_positive() {
        let new_row = row as i32 + row_diff.abs();
        let new_col = col as i32 + col_diff.abs();
        if new_row >= antinode_map.len() as i32 || new_col >= antinode_map[0].len() as i32 {
            None
        } else {
            Some((new_row as usize, new_col as usize))
        }
    } else {
        None
    }
}

fn get_unique_antinodes(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let node_map: HashMap<char, Vec<Node>> = get_node_map(&input.map);

    let mut unique_antinodes: u32 = 0;
    let mut antinode_pos_set: HashSet<(usize, usize)> = HashSet::new();
    let mut antinode_map = input.map.clone();

    for (_c, nodes) in node_map {
        for node in nodes.clone() {
            let other_nodes = nodes
                .iter()
                .filter(|&&n| n != node)
                .cloned()
                .collect::<Vec<Node>>();
            for other_node in other_nodes {
                let antinode_pos: Option<(usize, usize)> =
                    get_antinode_pos(node.pos, other_node.pos, &antinode_map);

                if let Some((antinode_row, antinode_col)) = antinode_pos
                    && let Some(map_node) = antinode_map
                        .get(antinode_row)
                        .and_then(|row| row.get(antinode_col))
                {
                    match *map_node {
                        '.' => {
                            if !antinode_pos_set.contains(&(antinode_row, antinode_col)) {
                                /*println!(
                                    "Found new antinode: {:?}, {:?}",
                                    *map_node,
                                    (antinode_row, antinode_col)
                                );*/
                                antinode_map[antinode_row][antinode_col] = '#';
                                unique_antinodes += 1;
                                antinode_pos_set.insert((antinode_row, antinode_col));
                            }
                        }
                        '#' => {
                            /*println!(
                                "Already occupied antinode: {:?}, {:?}",
                                *map_node,
                                (antinode_row, antinode_col)
                            );*/
                        }
                        _ => {
                            if !antinode_pos_set.contains(&(antinode_row, antinode_col)) {
                                /*println!(
                                    "Found overlapping antinode: {:?}, {:?}",
                                    *map_node,
                                    (antinode_row, antinode_col)
                                );*/
                                unique_antinodes += 1;
                                antinode_pos_set.insert((antinode_row, antinode_col));
                            }
                        }
                    }
                } else {
                    //println!("Antinode out of bounds: {:?}", (antinode_row, antinode_col));
                };
            }
        }
    }

    //print_map(&antinode_map);

    unique_antinodes
}

fn get_antinode_pos_vec(
    (row, col): (usize, usize),
    (other_row, other_col): (usize, usize),
    antinode_map: &[Vec<char>],
) -> Vec<(usize, usize)> {
    let mut antinode_pos_vec = vec![];
    antinode_pos_vec.push((row, col));
    antinode_pos_vec.push((other_row, other_col));
    let row_diff = row as i32 - other_row as i32;
    let col_diff = col as i32 - other_col as i32;
    let mut count = 0;
    loop {
        let (curr_row, curr_col) = (row as i32 + row_diff * count, col as i32 + col_diff * count);
        let (next_row, next_col) = (
            other_row as i32 + row_diff * count,
            other_col as i32 + col_diff * count,
        );
        let antinode_pos = get_antinode_pos(
            (curr_row as usize, curr_col as usize),
            (next_row as usize, next_col as usize),
            antinode_map,
        );
        if let Some((antinode_row, antinode_col)) = antinode_pos {
            antinode_pos_vec.push((antinode_row, antinode_col));
            if antinode_row == other_row && antinode_col == other_col {
                break;
            }
        } else {
            break;
        }

        count += 1;
    }
    antinode_pos_vec
}

fn get_unique_antinodes_with_hr(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let node_map: HashMap<char, Vec<Node>> = get_node_map(&input.map);

    let mut unique_antinodes: u32 = 0;
    let mut antinode_pos_set: HashSet<(usize, usize)> = HashSet::new();
    let mut antinode_map = input.map.clone();

    for (_c, nodes) in node_map {
        for node in nodes.clone() {
            let other_nodes = nodes
                .iter()
                .filter(|&&n| n != node)
                .cloned()
                .collect::<Vec<Node>>();
            for other_node in other_nodes {
                let antinode_pos_vec: Vec<(usize, usize)> =
                    get_antinode_pos_vec(node.pos, other_node.pos, &antinode_map);

                for (antinode_row, antinode_col) in antinode_pos_vec {
                    if let Some(map_node) = antinode_map
                        .get(antinode_row)
                        .and_then(|row| row.get(antinode_col))
                    {
                        match *map_node {
                            '.' => {
                                if !antinode_pos_set.contains(&(antinode_row, antinode_col)) {
                                    /*println!(
                                        "Found new antinode: {:?}, {:?}",
                                        *map_node,
                                        (antinode_row, antinode_col)
                                    );*/
                                    antinode_map[antinode_row][antinode_col] = '#';
                                    unique_antinodes += 1;
                                    antinode_pos_set.insert((antinode_row, antinode_col));
                                }
                            }
                            '#' => {
                                /*println!(
                                    "Already occupied antinode: {:?}, {:?}",
                                    *map_node,
                                    (antinode_row, antinode_col)
                                );*/
                            }
                            _ => {
                                if !antinode_pos_set.contains(&(antinode_row, antinode_col)) {
                                    /*println!(
                                        "Found overlapping antinode: {:?}, {:?}",
                                        *map_node,
                                        (antinode_row, antinode_col)
                                    );*/
                                    unique_antinodes += 1;
                                    antinode_pos_set.insert((antinode_row, antinode_col));
                                }
                            }
                        }
                    } else {
                        //println!("Antinode out of bounds: {:?}", (antinode_row, antinode_col));
                    };
                }
            }
        }
    }

    //print_map(&antinode_map);

    unique_antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unique_antinodes_test01() {
        assert_eq!(14, get_unique_antinodes("input/day08_test01.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_test02() {
        assert_eq!(2, get_unique_antinodes("input/day08_test02.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_test03() {
        assert_eq!(4, get_unique_antinodes("input/day08_test03.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_test04() {
        assert_eq!(4, get_unique_antinodes("input/day08_test04.txt"));
    }

    #[test]
    fn test_get_unique_antinodes() {
        assert_eq!(261, get_unique_antinodes("input/day08.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_with_hr_test01() {
        assert_eq!(34, get_unique_antinodes_with_hr("input/day08_test01.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_with_hr_test05() {
        assert_eq!(9, get_unique_antinodes_with_hr("input/day08_test05.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_with_hr_test06() {
        assert_eq!(9, get_unique_antinodes_with_hr("input/day08_test06.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_with_hr() {
        assert_eq!(898, get_unique_antinodes_with_hr("input/day08.txt"));
    }
}
