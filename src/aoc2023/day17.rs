// https://adventofcode.com/2023/day/17

use std::{collections::BinaryHeap, fs};

use itertools::Itertools;

use crate::utils::Direction;

#[derive(PartialEq, Eq)]
pub struct Node {
    pos: usize,
    dir: Option<Direction>,
    distance: usize,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Input {
    pub blocks: Vec<Vec<usize>>,
}

pub fn parse_input(input_file: &str) -> (Vec<usize>, usize, usize) {
    let input = fs::read_to_string(input_file).unwrap();
    let input = input.trim();
    let cols = input.find('\n').unwrap();
    let vals = input
        .lines()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .collect_vec();
    assert_eq!(vals.len() % cols, 0);
    let rows = vals.len() / cols;
    (vals, rows, cols)
}

pub fn parse_blocks(blocks_lines: Vec<String>) -> Vec<Vec<usize>> {
    let mut blocks: Vec<Vec<usize>> = Vec::new();
    for blocks_line in blocks_lines.into_iter() {
        let mut blocks_entries: Vec<usize> = Vec::new();
        for blocks_entry in blocks_line.chars() {
            blocks_entries.push(blocks_entry.to_digit(10).unwrap() as usize);
        }
        blocks.push(blocks_entries)
    }
    blocks
}

pub fn get_least_heat_loss<const MIN: usize, const MAX: usize>(input_file: &str) -> usize {
    let (tiles, rows, cols) = parse_input(input_file);
    let mut open = BinaryHeap::<Node>::new();
    let mut history = vec![(false, usize::MAX); tiles.len() * 4 * MAX];
    open.push(Node {
        pos: 0,
        dir: None,
        distance: 0,
        cost: 0,
    });
    while let Some(Node {
        pos,
        dir,
        distance,
        cost,
    }) = open.pop()
    {
        match dir {
            // Mark node as visited.
            Some(d) => history[pos * 4 * MAX + d.index() * MAX + distance].0 = true,
            None => {
                for d in 0..4 {
                    history[pos * 4 * MAX + d * MAX + distance].0 = true;
                }
            }
        };
        open.extend(
            [
                Direction::N,
                Direction::E,
                Direction::S,
                Direction::W,
            ]
            .iter()
            .filter_map(|&d| {
                let (same_dir, opp_dir) = match dir {
                    Some(pdir) => (pdir == d, pdir.opposite() == d),
                    None => (true, false),
                };
                if (distance < MIN && !same_dir)
                    || (distance > MAX - 1 && same_dir) // constraints
                    || opp_dir // no backtracking.
                    || match d { // don't go outside grid.
                    Direction::N => pos < cols,
                    Direction::E => pos % cols == cols - 1,
                    Direction::S => pos / cols == rows - 1,
                    Direction::W => pos % cols == 0,
                    Direction::Stop => false, // Stop does not move outside the grid.
                    }
                {
                    return None;
                }
                let npos = match d {
                    Direction::N => pos - cols,
                    Direction::E => pos + 1,
                    Direction::S => pos + cols,
                    Direction::W => pos - 1,
                    Direction::Stop => pos, // Handle Stop by keeping the position unchanged.
                };
                let ndist = 1 + if same_dir { distance } else { 0 };
                let nkey = npos * (4 * MAX) + d.index() * MAX + ndist;
                let ncost = cost + tiles[npos];
                let (visited, prevcost) = history[nkey];
                if visited || prevcost <= ncost {
                    return None;
                }
                history[nkey].1 = ncost;
                Some(Node {
                    pos: npos,
                    dir: Some(d),
                    distance: ndist,
                    cost: ncost,
                })
            }),
        );
    }
    // Get min cost of last tile.
    history[(tiles.len() - 1) * 4 * MAX..]
        .iter()
        .map(|(_visited, cost)| *cost)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_least_heat_loss_test01() {
        assert_eq!(
            102,
            get_least_heat_loss::<0, 3>("input/2023/day17_test01.txt")
        );
    }

    #[test]
    fn test_get_least_heat_loss_test02() {
        assert_eq!(
            14,
            get_least_heat_loss::<0, 3>("input/2023/day17_test02.txt")
        );
    }

    #[test]
    fn test_get_least_heat_loss_test03() {
        assert_eq!(
            28,
            get_least_heat_loss::<0, 3>("input/2023/day17_test03.txt")
        );
    }

    #[test]
    fn test_get_least_heat_loss_test04() {
        assert_eq!(
            11,
            get_least_heat_loss::<0, 3>("input/2023/day17_test04.txt")
        );
    }

    #[test]
    fn test_get_least_heat_loss_part01() {
        assert_eq!(1099, get_least_heat_loss::<0, 3>("input/2023/day17.txt"));
    }

    #[test]
    fn test_get_least_heat_loss_part02() {
        assert_eq!(1266, get_least_heat_loss::<4, 10>("input/2023/day17.txt"));
    }
}
