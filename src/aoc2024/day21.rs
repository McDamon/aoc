// https://adventofcode.com/2024/day/21

use std::{
    collections::{HashMap, VecDeque},
    vec,
};

use itertools::Itertools;
use petgraph::{
    Graph, algo,
    graph::{DiGraph, NodeIndex},
};

use crate::utils::{Direction, get_all_paths, get_lines};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move {
    pos: (usize, usize),
    button: char,
}

struct Input {
    codes: Vec<String>,
}

struct GraphData {
    cache: HashMap<Option<char>, (usize, usize)>,
    graph: Graph<Move, f64>,
    node_indices: HashMap<Move, NodeIndex>,
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

fn build_dir_keypad() -> HashMap<(usize, usize), Option<char>> {
    let mut dir_keypad = HashMap::new();

    // First column
    dir_keypad.insert((0, 0), None);
    dir_keypad.insert((0, 1), Some('<'));

    // Second column
    dir_keypad.insert((1, 0), Some('^'));
    dir_keypad.insert((1, 1), Some('v'));

    // Third column
    dir_keypad.insert((2, 0), Some('A'));
    dir_keypad.insert((2, 1), Some('>'));

    dir_keypad
}

fn build_keypad_graph(
    keypad: &HashMap<(usize, usize), Option<char>>,
) -> (Graph<Move, f64>, HashMap<Move, NodeIndex>) {
    // Create directed graph
    let mut graph = DiGraph::<Move, f64>::new();
    let mut node_indices: HashMap<Move, NodeIndex> = HashMap::new();

    // Add nodes and edges

    // First create all nodes
    for (&(x, y), &maybe_button) in keypad.iter() {
        if let Some(button) = maybe_button {
            for _dir in Direction::all() {
                let point = Move {
                    pos: (x, y),
                    button,
                };
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
    for (&(x, y), &maybe_button) in keypad.iter() {
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
                if let Some(&maybe_next_button) = keypad.get(&(next_x as usize, next_y as usize))
                    && let Some(next_button) = maybe_next_button
                {
                    let next_move = Move {
                        pos: (next_x as usize, next_y as usize),
                        button: next_button,
                    };
                    let next_idx = node_indices[&next_move];
                    graph.add_edge(node_idx, next_idx, 1.0);
                    //println!("Added edge {:?} -> {:?}", graph[node_idx], graph[next_idx]);
                }
            }
        }
    }

    (graph, node_indices)
}

/*fn print_keypad(keypad: &HashMap<(usize, usize), Option<char>>) {
    let max_x = keypad.keys().map(|(x, _)| *x).max().unwrap();
    let max_y = keypad.keys().map(|(_, y)| *y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(c) = keypad.get(&(x, y)) {
                if let Some(c) = c {
                    print!("{}", c)
                } else {
                    print!(" ")
                }
            }
        }
        println!();
    }
}*/

fn print_keypad_path(path: &VecDeque<Move>) {
    for m in path {
        print!("{} ", m.button);
    }
    println!();
}

fn get_move_direction(start: Move, end: Move) -> Option<Direction> {
    let (start_x, start_y) = start.pos;
    let (end_x, end_y) = end.pos;
    if start == end {
        None
    } else if end_x > start_x {
        Some(Direction::E)
    } else if end_x < start_x {
        Some(Direction::W)
    } else if end_y > start_y {
        Some(Direction::S)
    } else {
        Some(Direction::N)
    }
}

fn get_dir_move_entry(direction: Direction) -> char {
    match direction {
        Direction::N => '^',
        Direction::S => 'v',
        Direction::E => '>',
        Direction::W => '<',
    }
}

fn get_dir_path_from_keypad_path(
    keypad_path: &VecDeque<Move>,
    dir_keypad_cache: &HashMap<Option<char>, (usize, usize)>,
) -> VecDeque<Move> {
    let mut new_dir_keypad_path: VecDeque<Move> = VecDeque::new();
    for (start_num_move, end_num_move) in keypad_path.iter().tuple_windows() {
        if let Some(move_direction) = get_move_direction(*start_num_move, *end_num_move) {
            new_dir_keypad_path.push_back(Move {
                pos: dir_keypad_cache[&Some(get_dir_move_entry(move_direction))],
                button: get_dir_move_entry(move_direction),
            });
        } else {
            new_dir_keypad_path.push_back(Move {
                pos: dir_keypad_cache[&Some('A')],
                button: 'A',
            });
        }
    }
    new_dir_keypad_path.push_back(Move {
        pos: dir_keypad_cache[&Some('A')],
        button: 'A',
    });
    new_dir_keypad_path
}

fn get_shortest_paths_for_move(
    start_move: &Move,
    end_move: &Move,
    keypad_graph_data: &GraphData,
) -> Vec<Vec<Move>> {
    /*println!(
        "Finding shortest path from {:?} to {:?}",
        start_move, end_move
    );*/
    let start_idx = keypad_graph_data.node_indices[start_move];
    let end_idx = keypad_graph_data.node_indices[end_move];

    let keypad_costs = algo::dijkstra(&keypad_graph_data.graph, start_idx, Some(end_idx), |e| {
        *e.weight()
    });
    let all_paths = get_all_paths(&keypad_graph_data.graph, &keypad_costs, start_idx, end_idx);
    /*for path in &all_paths {
        println!("Path:");
        for node_index in path.iter() {
            println!("{:?}", keypad_graph_data.graph[*node_index]);
        }
    }*/
    all_paths
        .iter()
        .map(|path| {
            path.iter()
                .map(|&idx| keypad_graph_data.graph[idx])
                .collect()
        })
        .collect()
}

fn get_all_num_keypad_paths(code: &str, num_keypad_graph_data: &GraphData) -> Vec<VecDeque<Move>> {
    let mut all_num_keypad_paths: Vec<VecDeque<Move>> = vec![VecDeque::new()];

    for (start_num_move, end_num_move) in code.chars().tuple_windows() {
        let start_num_move_pos = num_keypad_graph_data.cache[&Some(start_num_move)];
        let end_num_move_pos = num_keypad_graph_data.cache[&Some(end_num_move)];
        let start_move = Move {
            pos: start_num_move_pos,
            button: start_num_move,
        };
        let end_move = Move {
            pos: end_num_move_pos,
            button: end_num_move,
        };
        let shortest_keypad_paths =
            get_shortest_paths_for_move(&start_move, &end_move, num_keypad_graph_data);

        let mut new_num_keypad_paths = Vec::new();
        for base_num_keypad_path in all_num_keypad_paths {
            for path in &shortest_keypad_paths {
                let mut combined = base_num_keypad_path.clone();
                combined.extend(path.iter().cloned());
                new_num_keypad_paths.push(combined);
            }
        }
        all_num_keypad_paths = new_num_keypad_paths;
    }

    all_num_keypad_paths
}

fn get_all_dir_keypad_paths(
    dir_keypad_path: &VecDeque<Move>,
    dir_keypad_graph_data: &GraphData,
) -> Vec<VecDeque<Move>> {
    let mut all_dir_keypad_paths: Vec<VecDeque<Move>> = vec![VecDeque::new()];

    for (start_dir_move, end_dir_move) in dir_keypad_path.into_iter().tuple_windows() {
        let start_dir_move_pos = dir_keypad_graph_data.cache[&Some(start_dir_move.button)];
        let end_dir_move_pos = dir_keypad_graph_data.cache[&Some(end_dir_move.button)];
        let start_move = Move {
            pos: start_dir_move_pos,
            button: start_dir_move.button,
        };
        let end_move = Move {
            pos: end_dir_move_pos,
            button: end_dir_move.button,
        };
        let shortest_keypad_paths =
            get_shortest_paths_for_move(&start_move, &end_move, dir_keypad_graph_data);

        let mut new_dir_keypad_paths = Vec::new();
        for base_dir_keypad_path in all_dir_keypad_paths {
            for path in &shortest_keypad_paths {
                let mut combined = base_dir_keypad_path.clone();
                combined.extend(path.iter().cloned());
                new_dir_keypad_paths.push(combined);
            }
        }
        all_dir_keypad_paths = new_dir_keypad_paths;
    }

    all_dir_keypad_paths
}

fn get_shortest_sequence_len(
    code: &str,
    num_keypad_graph_data: &GraphData,
    num_dir_keypads: usize,
    dir_keypad_graph_data: &GraphData,
) -> usize {
    let code_with_initial_pos: String = format!("A{}", code);

    let all_num_keypad_paths =
        get_all_num_keypad_paths(&code_with_initial_pos, num_keypad_graph_data);

    let mut all_dir_keypath_paths = vec![];

    for num_keypad_path in all_num_keypad_paths.iter() {
        let dir_keypad_path =
            get_dir_path_from_keypad_path(&num_keypad_path, &dir_keypad_graph_data.cache);
        all_dir_keypath_paths.push(dir_keypad_path);
    }

    let mut all_full_dir_keypath_paths: Vec<VecDeque<Move>> = vec![VecDeque::new()];

    for dir_keypad_path in &all_dir_keypath_paths {
        let full_dir_keypath_paths =
            get_all_dir_keypad_paths(&dir_keypad_path, &dir_keypad_graph_data);

        all_full_dir_keypath_paths.extend(full_dir_keypath_paths);
    }

    0
}

fn recursive_do_stuff(
    t: usize,
    num_dir_keypads: usize,
    all_full_dir_keypath_paths: &mut Vec<VecDeque<Move>>,
    dir_keypad_graph_data: &GraphData,
) {
    for i in 1..num_dir_keypads {
        if i == t {
            // Do stuff

            for dir_keypad_path in &all_full_dir_keypath_paths {
                let full_dir_keypath_paths =
                    get_all_dir_keypad_paths(&dir_keypad_path, &dir_keypad_graph_data);

                all_full_dir_keypath_paths.extend(full_dir_keypath_paths);
            }
        } else {
            recursive_do_stuff(
                t,
                num_dir_keypads,
                all_full_dir_keypath_paths,
                dir_keypad_graph_data,
            );
        }
    }
}
pub fn get_sum_complexity(input_file: &str, num_dir_keypads: usize) -> usize {
    let input = parse_input(input_file);

    let num_keypad: HashMap<(usize, usize), Option<char>> = build_num_keypad();
    let dir_keypad: HashMap<(usize, usize), Option<char>> = build_dir_keypad();

    let num_keypad_cache = num_keypad
        .iter()
        .map(|(k, v)| (*v, *k))
        .collect::<HashMap<_, _>>();
    let dir_keypad_cache = dir_keypad
        .iter()
        .map(|(k, v)| (*v, *k))
        .collect::<HashMap<_, _>>();

    let (num_keypad_graph, num_keypad_node_indices) = build_keypad_graph(&num_keypad);
    let (dir_keypad_graph, dir_keypad_node_indices) = build_keypad_graph(&dir_keypad);

    let mut sum_complexity = 0;

    for code in input.codes {
        let shortest_sequence_len = get_shortest_sequence_len(
            &code,
            &GraphData {
                cache: num_keypad_cache.clone(),
                graph: num_keypad_graph.clone(),
                node_indices: num_keypad_node_indices.clone(),
            },
            num_dir_keypads,
            &GraphData {
                cache: dir_keypad_cache.clone(),
                graph: dir_keypad_graph.clone(),
                node_indices: dir_keypad_node_indices.clone(),
            },
        );
        if let Ok(num_part) = code[0..code.len() - 1].parse::<usize>() {
            sum_complexity += shortest_sequence_len * num_part;
        }
    }

    sum_complexity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_complexity_test01() {
        assert_eq!(126384, get_sum_complexity("input/2024/day21_test01.txt", 2));
    }

    #[test]
    fn test_get_sum_complexity_test02() {
        assert_eq!(1972, get_sum_complexity("input/2024/day21_test02.txt", 2));
    }

    #[test]
    fn test_get_sum_complexity_test03() {
        assert_eq!(58800, get_sum_complexity("input/2024/day21_test03.txt", 2));
    }

    #[test]
    fn test_get_sum_complexity() {
        assert_eq!(1151792, get_sum_complexity("input/2024/day21.txt", 2));
    }
}
