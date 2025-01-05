// https://adventofcode.com/2024/day/16

use std::collections::HashMap;

use petgraph::{
    algo,
    graph::{DiGraph, NodeIndex},
};

use crate::utils::{Direction, get_lines};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Move {
    pos: (isize, isize),
    dir: Direction,
}

#[derive(Debug)]
struct Input {
    maze: HashMap<(isize, isize), char>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut maze: HashMap<(isize, isize), char> = HashMap::new();

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            maze.insert((x as isize, y as isize), c);
        }
    }

    Input { maze }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    dir: Direction,
}

fn get_lowest_score(input_file: &str) -> usize {
    let input = parse_input(input_file);

    // Find start and end points
    let start = input
        .maze
        .iter()
        .find(|(_, c)| **c == 'S')
        .map(|(&pos, _)| pos)
        .unwrap();
    let end = input
        .maze
        .iter()
        .find(|(_, c)| **c == 'E')
        .map(|(&pos, _)| pos)
        .unwrap();

    // Create directed graph
    let mut graph = DiGraph::<Point, isize>::new();
    let mut node_indices: HashMap<Point, NodeIndex> = HashMap::new();

    // Add nodes and edges
    // First create all nodes
    for (&(x, y), &c) in input.maze.iter() {
        if c != '#' {
            for dir in Direction::all() {
                let point = Point { x, y, dir };
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
    for (&(x, y), &c) in input.maze.iter() {
        if c != '#' {
            for dir in Direction::all() {
                let point = Point { x, y, dir };
                let node_idx = node_indices[&point];

                // Forward movement
                let (dx, dy) = dir.to_delta();
                let next_pos = (x + dx, y + dy);
                if let Some(&next_c) = input.maze.get(&next_pos) {
                    if next_c != '#' {
                        let next_point = Point {
                            x: next_pos.0,
                            y: next_pos.1,
                            dir,
                        };
                        let next_idx = node_indices[&next_point];
                        graph.add_edge(node_idx, next_idx, 1);
                        println!(
                            "Added forward edge {} -> {} (weight 1)",
                            node_idx.index(),
                            next_idx.index()
                        );
                    }
                }

                // Turn edges
                let left_point = Point {
                    x,
                    y,
                    dir: dir.turn_left(),
                };
                let right_point = Point {
                    x,
                    y,
                    dir: dir.turn_right(),
                };

                let left_idx = node_indices[&left_point];
                let right_idx = node_indices[&right_point];

                graph.add_edge(node_idx, left_idx, 1000);
                graph.add_edge(node_idx, right_idx, 1000);
                println!(
                    "Added turn edges for node {} (left: {}, right: {})",
                    node_idx.index(),
                    left_idx.index(),
                    right_idx.index()
                );
            }
        }
    }

    // Calculate shortest paths
    let mut min_length = isize::MAX;

    for end_dir in Direction::all() {
        let start_point = Point {
            x: start.0,
            y: start.1,
            dir: Direction::E,
        };
        let end_point = Point {
            x: end.0,
            y: end.1,
            dir: end_dir,
        };

        if let (Some(&start_idx), Some(&end_idx)) =
            (node_indices.get(&start_point), node_indices.get(&end_point))
        {
            let distances = algo::dijkstra(&graph, start_idx, Some(end_idx), |e| *e.weight());
            if let Some(&length) = distances.get(&end_idx) {
                min_length = min_length.min(length);
            }
        }
    }

    min_length as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_gps_test01() {
        assert_eq!(7036, get_lowest_score("input/2024/day16_test01.txt"));
    }

    #[test]
    fn test_get_sum_gps_test02() {
        assert_eq!(11048, get_lowest_score("input/2024/day16_test02.txt"));
    }

    #[test]
    fn test_get_sum_gps_test03() {
        assert_eq!(1005, get_lowest_score("input/2024/day16_test03.txt"));
    }

    #[test]
    fn test_get_sum_gps() {
        assert_eq!(79404, get_lowest_score("input/2024/day16.txt"));
    }
}
