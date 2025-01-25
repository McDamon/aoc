// https://adventofcode.com/2024/day/21

use std::collections::HashMap;

use petgraph::{
    Graph, algo,
    graph::{DiGraph, NodeIndex},
};

use crate::utils::{Direction, get_lines};

enum ButtonPress {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move {
    pos: (usize, usize),
    button: char,
}

struct Input {
    codes: Vec<String>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut codes = vec![];

    for line in lines {
        codes.push(line);
    }

    Input { codes }
}

fn build_num_keypad() -> HashMap<(usize, usize), Option<char>> {
    let mut num_keypad = HashMap::new();

    // First column
    num_keypad.insert((0, 0), Some('7'));
    num_keypad.insert((0, 1), Some('4'));
    num_keypad.insert((0, 2), Some('1'));
    num_keypad.insert((0, 3), None);

    // Second column
    num_keypad.insert((1, 0), Some('8'));
    num_keypad.insert((1, 1), Some('5'));
    num_keypad.insert((1, 2), Some('2'));
    num_keypad.insert((1, 3), Some('0'));

    // Third column
    num_keypad.insert((2, 0), Some('9'));
    num_keypad.insert((2, 1), Some('6'));
    num_keypad.insert((2, 2), Some('3'));
    num_keypad.insert((2, 3), Some('A'));

    num_keypad
}

fn build_num_keypad_graph(
    num_keypad: &HashMap<(usize, usize), Option<char>>,
) -> (Graph<Move, f64>, HashMap<Move, NodeIndex>) {
    // Create directed graph
    let mut graph = DiGraph::<Move, f64>::new();
    let mut node_indices: HashMap<Move, NodeIndex> = HashMap::new();

    // Add nodes and edges

    // First create all nodes
    for (&(x, y), &maybe_button) in num_keypad.iter() {
        if let Some(button) = maybe_button {
            for dir in Direction::all() {
                let point = Move {
                    pos: (x, y),
                    button,
                };
                let node_idx = graph.add_node(point);
                node_indices.insert(point, node_idx);
                println!(
                    "Added node at ({}, {}) dir {:?} -> idx {:?}",
                    x, y, dir, node_idx
                );
            }
        }
    }

    // Then add all edges in a separate pass
    for (&(x, y), &maybe_button) in num_keypad.iter() {
        if let Some(button) = maybe_button {
            for dir in Direction::all() {
                let point = Move {
                    pos: (x, y),
                    button,
                };
                let node_idx = node_indices[&point];

                // Forward movement
                let (dx, dy) = dir.to_delta();
                let (next_x, next_y) = (x as isize + dx, y as isize + dy);
                if let Some(&maybe_next_button) =
                    num_keypad.get(&(next_x as usize, next_y as usize))
                {
                    if let Some(next_button) = maybe_next_button {
                        let next_move = Move {
                            pos: (next_x as usize, next_y as usize),
                            button: next_button,
                        };
                        let next_idx = node_indices[&next_move];
                        graph.add_edge(node_idx, next_idx, 1.0);
                        println!("Added edge {:?} -> {:?}", graph[node_idx], graph[next_idx]);
                    }
                }
            }
        }
    }

    (graph, node_indices)
}

fn print_num_keypad(num_keypad: &HashMap<(usize, usize), Option<char>>) {
    let max_x = num_keypad.keys().map(|(x, _)| *x).max().unwrap();
    let max_y = num_keypad.keys().map(|(_, y)| *y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(c) = num_keypad.get(&(x, y)) {
                if let Some(c) = c {
                    print!("{}", c)
                } else {
                    print!(" ")
                }
            }
        }
        println!();
    }
}

fn get_sum_complexity(input_file: &str, num_dir_keypads: usize) -> u32 {
    let input = parse_input(input_file);

    let num_keypad: HashMap<(usize, usize), Option<char>> = build_num_keypad();

    print_num_keypad(&num_keypad);

    let (num_keypad_graph, num_keypad_node_indices) = build_num_keypad_graph(&num_keypad);

    let start_move = Move {
        pos: (0, 2),
        button: '1',
    };
    let end_move = Move {
        pos: (0, 0),
        button: '7',
    };

    let start_idx = num_keypad_node_indices[&start_move];
    let end_idx = num_keypad_node_indices[&end_move];

    if let Some((distance, _path)) = algo::astar(
        &num_keypad_graph,
        start_idx,
        |finish| finish == end_idx,
        |e| *e.weight() as usize,
        |_| 0,
    ) {
        println!(
            "Found path from {:?} to {:?} with distance {}",
            start_move, end_move, distance
        )
    }

    let mut sum_complexity = 0;

    sum_complexity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_complexity_test01() {
        assert_eq!(126384, get_sum_complexity("input/2024/day21_test01.txt", 3));
    }

    #[test]
    fn test_get_sum_complexity() {
        assert_eq!(1151792, get_sum_complexity("input/2024/day21.txt", 3));
    }
}
