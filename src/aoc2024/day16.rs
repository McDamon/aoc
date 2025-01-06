// https://adventofcode.com/2024/day/16

use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::{
    algo,
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

fn build_paths_from_predecessors(
    end: NodeIndex,
    target_distance: f64,
    distances: &[f64],
    predecessors: &[Option<NodeIndex>],
) -> Vec<Vec<NodeIndex>> {
    let mut all_paths = Vec::new();
    let mut queue = VecDeque::new();

    // Find start node(s)
    for (idx, pred) in predecessors.iter().enumerate() {
        if pred.is_none() {
            queue.push_back((vec![NodeIndex::new(idx)], 0.0));
        }
    }

    while let Some((path, current_distance)) = queue.pop_front() {
        let current = *path.last().unwrap();

        if current == end && current_distance == target_distance {
            all_paths.push(path);
            continue;
        }

        if current_distance > target_distance {
            continue;
        }

        let current_dist = distances[current.index()];
        for (idx, &pred) in predecessors.iter().enumerate() {
            if let Some(pred_node) = pred {
                if pred_node == current {
                    let mut new_path = path.clone();
                    new_path.push(NodeIndex::new(idx));
                    let step = distances[idx] - current_dist;
                    queue.push_back((new_path, current_distance + step));
                }
            }
        }
    }

    all_paths
}

fn print_maze(maze: &HashMap<(isize, isize), char>) {
    let max_x = maze.keys().map(|(x, _)| *x).max().unwrap();
    let max_y = maze.keys().map(|(_, y)| *y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(c) = maze.get(&(x, y)) {
                print!("{}", c);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn get_lowest_score(input_file: &str) -> (usize, usize) {
    let input = parse_input(input_file);

    print_maze(&input.maze);

    // Find start and end points
    let (start_x, start_y) = input
        .maze
        .iter()
        .find(|(_, c)| **c == 'S')
        .map(|(&pos, _)| pos)
        .unwrap();
    let (end_x, end_y) = input
        .maze
        .iter()
        .find(|(_, c)| **c == 'E')
        .map(|(&pos, _)| pos)
        .unwrap();

    // Create directed graph
    let mut graph = DiGraph::<Move, f64>::new();
    let mut node_indices: HashMap<Move, NodeIndex> = HashMap::new();

    // Add nodes and edges
    // First create all nodes
    for (&(x, y), &c) in input.maze.iter() {
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
    for (&(x, y), &c) in input.maze.iter() {
        if c != '#' {
            for dir in Direction::all() {
                let point = Move { pos: (x, y), dir };
                let node_idx = node_indices[&point];

                // Forward movement
                let (dx, dy) = dir.to_delta();
                let (next_x, next_y) = (x + dx, y + dy);
                if let Some(&next_c) = input.maze.get(&(next_x, next_y)) {
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

                graph.add_edge(node_idx, left_idx, 1000.0);
                graph.add_edge(node_idx, right_idx, 1000.0);
                /*println!(
                    "Added turn edges for node {} (left: {}, right: {})",
                    node_idx.index(),
                    left_idx.index(),
                    right_idx.index()
                );*/
            }
        }
    }

    let mut min_length = isize::MAX;

    let mut tiles: HashSet<(isize, isize)> = HashSet::new();

    let start_move = Move {
        pos: (start_x, start_y),
        dir: Direction::E,
    };

    if let Some(&start_idx) = node_indices.get(&start_move)
        && let Ok(paths) = algo::bellman_ford(&graph, start_idx)
    {
        for end_dir in Direction::all() {
            let end_move = Move {
                pos: (end_x, end_y),
                dir: end_dir,
            };

            if let Some(&end_idx) = node_indices.get(&end_move) {
                let distance = paths.distances[end_idx.index()];
                let new_min_length = min_length.min(distance as isize);
                if new_min_length < min_length {
                    min_length = new_min_length;
                }
            }
        }

        for end_dir in Direction::all() {
            let end_move = Move {
                pos: (end_x, end_y),
                dir: end_dir,
            };

            if let Some(&end_idx) = node_indices.get(&end_move) {
                let distance = paths.distances[end_idx.index()];
                if distance == min_length as f64 {
                    let all_paths = build_paths_from_predecessors(
                        end_idx,
                        distance,
                        &paths.distances,
                        &paths.predecessors,
                    );

                    for path in all_paths.iter() {
                        println!("*** Path: ***");
                        for path_entry in path {
                            println!(
                                "Path entry: {:?} -> {:?}",
                                graph[*path_entry].pos, graph[*path_entry].dir
                            )
                        }
                    }

                    tiles.extend(all_paths.into_iter().flatten().map(|idx| graph[idx].pos));
                }
            }
        }
    }

    (min_length as usize, tiles.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lowest_score_test01() {
        assert_eq!(7036, get_lowest_score("input/2024/day16_test01.txt").0);
    }

    #[test]
    fn test_get_lowest_score_test02() {
        assert_eq!(11048, get_lowest_score("input/2024/day16_test02.txt").0);
    }

    #[test]
    fn test_get_lowest_score_test03() {
        assert_eq!(21148, get_lowest_score("input/2024/day16_test03.txt").0);
    }

    #[test]
    fn test_get_lowest_score_test04() {
        assert_eq!(4013, get_lowest_score("input/2024/day16_test04.txt").0);
    }

    #[test]
    fn test_get_lowest_score_test05() {
        assert_eq!(21110, get_lowest_score("input/2024/day16_test05.txt").0);
    }

    #[test]
    fn test_get_lowest_score_test06() {
        assert_eq!(9029, get_lowest_score("input/2024/day16_test06.txt").0);
    }

    #[test]
    fn test_get_lowest_score_test07() {
        assert_eq!(4011, get_lowest_score("input/2024/day16_test07.txt").0);
    }

    #[test]
    fn test_get_lowest_score() {
        assert_eq!(79404, get_lowest_score("input/2024/day16.txt").0);
    }

    #[test]
    fn test_get_num_tiles_test01() {
        assert_eq!(45, get_lowest_score("input/2024/day16_test01.txt").1);
    }

    #[test]
    fn test_get_num_tiles_test02() {
        assert_eq!(64, get_lowest_score("input/2024/day16_test02.txt").1);
    }

    #[test]
    fn test_get_num_tiles_test03() {
        assert_eq!(149, get_lowest_score("input/2024/day16_test03.txt").1);
    }

    #[test]
    fn test_get_num_tiles_test04() {
        assert_eq!(14, get_lowest_score("input/2024/day16_test04.txt").1);
    }

    #[test]
    fn test_get_num_tiles_test05() {
        assert_eq!(264, get_lowest_score("input/2024/day16_test05.txt").1);
    }

    #[test]
    fn test_get_num_tiles_test06() {
        assert_eq!(62, get_lowest_score("input/2024/day16_test06.txt").1);
    }

    #[test]
    fn test_get_num_tiles_test07() {
        assert_eq!(17, get_lowest_score("input/2024/day16_test07.txt").1);
    }

    #[test]
    fn test_get_num_tiles() {
        assert_eq!(0, get_lowest_score("input/2024/day16.txt").1);
    }
}
