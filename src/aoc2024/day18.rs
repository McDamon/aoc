// https://adventofcode.com/2024/day/18

use std::collections::HashMap;

use petgraph::{
    Graph, algo,
    graph::{DiGraph, NodeIndex},
};

use crate::utils::{Direction, get_lines};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move {
    pos: (isize, isize),
    dir: Direction,
}

#[derive(Debug)]
struct Input {
    corrupt_mem: Vec<(isize, isize)>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut corrupt_mem: Vec<(isize, isize)> = vec![];

    for line in lines {
        if let Some((x_str, y_str)) = line.split_once(',') {
            let x = x_str.parse::<isize>().unwrap();
            let y = y_str.parse::<isize>().unwrap();
            corrupt_mem.push((x, y));
        }
    }

    Input { corrupt_mem }
}

fn print_mem_map(mem_map: &HashMap<(isize, isize), char>) {
    let max_x = mem_map.keys().map(|(x, _)| *x).max().unwrap();
    let max_y = mem_map.keys().map(|(_, y)| *y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(c) = mem_map.get(&(x, y)) {
                print!("{}", c);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn build_graph(
    mem_map: &HashMap<(isize, isize), char>,
) -> (Graph<Move, f64>, HashMap<Move, NodeIndex>) {
    // Create directed graph
    let mut graph = DiGraph::<Move, f64>::new();
    let mut node_indices: HashMap<Move, NodeIndex> = HashMap::new();

    // Add nodes and edges
    // First create all nodes
    for (&(x, y), &c) in mem_map.iter() {
        if c != '#' {
            for dir in Direction::all() {
                let point = Move { pos: (x, y), dir };
                let node_idx = graph.add_node(point);
                node_indices.insert(point, node_idx);
                /*println!(
                    "Added node at ({}, {}) dir {:?} -> idx {:?}",
                    x, y, dir, node_idx
                );*/
            }
        }
    }

    // Then add all edges in a separate pass
    for (&(x, y), &c) in mem_map.iter() {
        if c != '#' {
            for dir in Direction::all() {
                let point = Move { pos: (x, y), dir };
                let node_idx = node_indices[&point];

                // Forward movement
                let (dx, dy) = dir.to_delta();
                let (next_x, next_y) = (x + dx, y + dy);
                if let Some(&next_c) = mem_map.get(&(next_x, next_y)) {
                    if next_c != '#' {
                        let next_move = Move {
                            pos: (next_x, next_y),
                            dir,
                        };
                        let next_idx = node_indices[&next_move];
                        graph.add_edge(node_idx, next_idx, 1.0);
                        /*println!(
                            "Added forward edge {} -> {} (weight 1)",
                            node_idx.index(),
                            next_idx.index()
                        );*/
                    }
                }

                // Turn edges
                let left_move = Move {
                    pos: (x, y),
                    dir: dir.turn_left(),
                };
                let right_move = Move {
                    pos: (x, y),
                    dir: dir.turn_right(),
                };

                let left_idx = node_indices[&left_move];
                let right_idx = node_indices[&right_move];

                graph.add_edge(node_idx, left_idx, 1.0);
                graph.add_edge(node_idx, right_idx, 1.0);
                /*println!(
                    "Added turn edges for node {} (left: {}, right: {})",
                    node_idx.index(),
                    left_idx.index(),
                    right_idx.index()
                );*/
            }
        }
    }
    (graph, node_indices)
}

fn build_mem_map(
    corrupt_mem: &Vec<(isize, isize)>,
    x_len: isize,
    y_len: isize,
    bytes: usize,
) -> HashMap<(isize, isize), char> {
    let mut mem_map = HashMap::new();
    for y in 0..y_len {
        for x in 0..x_len {
            mem_map.insert((x as isize, y as isize), '.');
        }
    }

    let mut count = 0;
    for (x, y) in corrupt_mem {
        if count < bytes {
            mem_map.insert((*x, *y), '#');
        }
        count += 1;
    }

    mem_map
}

fn get_min_steps(input_file: &str, x_len: isize, y_len: isize, bytes: usize) -> usize {
    let input = parse_input(input_file);

    println!("Corrupt mem: {:?}", input.corrupt_mem);

    let mem_map: HashMap<(isize, isize), char> =
        build_mem_map(&input.corrupt_mem, x_len, y_len, bytes);

    print_mem_map(&mem_map);

    let (graph, node_indices) = build_graph(&mem_map);

    for start_dir in Direction::all() {
        for end_dir in Direction::all() {
            let start = Move {
                pos: (0, 0),
                dir: start_dir,
            };
            let start_idx = node_indices[&start];

            let end = Move {
                pos: (x_len - 1, y_len - 1),
                dir: end_dir,
            };
            let end_idx = node_indices[&end];

            if let Some((distance, path)) = algo::astar(
                &graph,
                start_idx,
                |finish| finish == end_idx,
                |e| *e.weight() as usize,
                |_| 1,
            ) {
                println!("Found path from {:?} to {:?} with distance {}", start, end, distance);
                for node in path {
                    println!("  {:?}", graph[node]);
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_min_steps_test01() {
        assert_eq!(0, get_min_steps("input/2024/day18_test01.txt", 7, 7, 12));
    }

    #[test]
    fn test_get_min_steps() {
        assert_eq!(0, get_min_steps("input/2024/day18.txt", 71, 71, 1024));
    }
}
