// https://adventofcode.com/2024/day/21

use std::collections::HashMap;

use itertools::Itertools;
use petgraph::{
    Graph, algo,
    graph::{DiGraph, NodeIndex},
};

use crate::utils::{Direction, get_all_paths, get_lines};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Button {
    Activate,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move {
    pos: (usize, usize),
    button: Button,
}

struct Input {
    codes: Vec<String>,
}

struct GraphData {
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

fn build_num_keypad() -> HashMap<(usize, usize), Option<Button>> {
    let mut num_keypad = HashMap::new();

    // First column
    num_keypad.insert((0, 0), Some(Button::Seven));
    num_keypad.insert((0, 1), Some(Button::Four));
    num_keypad.insert((0, 2), Some(Button::One));
    num_keypad.insert((0, 3), None);

    // Second column
    num_keypad.insert((1, 0), Some(Button::Eight));
    num_keypad.insert((1, 1), Some(Button::Five));
    num_keypad.insert((1, 2), Some(Button::Two));
    num_keypad.insert((1, 3), Some(Button::Zero));

    // Third column
    num_keypad.insert((2, 0), Some(Button::Nine));
    num_keypad.insert((2, 1), Some(Button::Six));
    num_keypad.insert((2, 2), Some(Button::Three));
    num_keypad.insert((2, 3), Some(Button::Activate));

    num_keypad
}

fn build_dir_keypad() -> HashMap<(usize, usize), Option<Button>> {
    let mut dir_keypad = HashMap::new();

    // First column
    dir_keypad.insert((0, 0), None);
    dir_keypad.insert((0, 1), Some(Button::Left));

    // Second column
    dir_keypad.insert((1, 0), Some(Button::Up));
    dir_keypad.insert((1, 1), Some(Button::Down));

    // Third column
    dir_keypad.insert((2, 0), Some(Button::Activate));
    dir_keypad.insert((2, 1), Some(Button::Right));

    dir_keypad
}

fn build_keypad_graph(
    keypad: &HashMap<(usize, usize), Option<Button>>,
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
                }
            }
        }
    }

    (graph, node_indices)
}

fn get_shortest_paths_for_move(
    start_move: &Move,
    end_move: &Move,
    keypad_graph_data: &GraphData,
) -> Vec<Vec<Move>> {
    let start_idx = keypad_graph_data.node_indices[start_move];
    let end_idx = keypad_graph_data.node_indices[end_move];

    let keypad_costs = algo::dijkstra(&keypad_graph_data.graph, start_idx, Some(end_idx), |e| {
        *e.weight()
    });
    let all_paths = get_all_paths(&keypad_graph_data.graph, &keypad_costs, start_idx, end_idx);
    all_paths
        .iter()
        .map(|path| {
            path.iter()
                .map(|&idx| keypad_graph_data.graph[idx])
                .collect()
        })
        .collect()
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

fn get_dir_move_entry(direction: Direction) -> Button {
    match direction {
        Direction::N => Button::Up,
        Direction::S => Button::Down,
        Direction::E => Button::Right,
        Direction::W => Button::Left,
        Direction::Stop => panic!("Direction::Stop is not a valid move"),
    }
}

fn get_dir_path_from_keypad_path(keypad_path: &[Move]) -> Vec<Button> {
    let mut new_dir_keypad_path: Vec<Button> = vec![];
    for (start_num_move, end_num_move) in keypad_path.iter().tuple_windows() {
        if let Some(move_direction) = get_move_direction(*start_num_move, *end_num_move) {
            new_dir_keypad_path.push(get_dir_move_entry(move_direction));
        }
    }
    new_dir_keypad_path
}

fn build_move_cache(
    keypad: &HashMap<(usize, usize), Option<Button>>,
    keypad_graph_data: &GraphData,
) -> HashMap<(Move, Move), Vec<Vec<Button>>> {
    let mut move_cache = HashMap::new();

    for (&(x1, y1), &maybe_button1) in keypad.iter() {
        if let Some(button1) = maybe_button1 {
            for (&(x2, y2), &maybe_button2) in keypad.iter() {
                if let Some(button2) = maybe_button2 {
                    let start_move = Move {
                        pos: (x1, y1),
                        button: button1,
                    };
                    let end_move = Move {
                        pos: (x2, y2),
                        button: button2,
                    };
                    let paths =
                        get_shortest_paths_for_move(&start_move, &end_move, keypad_graph_data);

                    let dir_paths: Vec<Vec<Button>> = paths
                        .iter()
                        .map(|path| get_dir_path_from_keypad_path(path))
                        .collect();

                    move_cache.insert((start_move, end_move), dir_paths);
                }
            }
        }
    }

    move_cache
}

fn build_path_cache(
    move_cache: &HashMap<(Move, Move), Vec<Vec<Button>>>,
) -> HashMap<(Button, Button), Vec<Vec<Button>>> {
    let mut path_cache = HashMap::new();

    for (&(start_move, end_move), paths) in move_cache.iter() {
        let start_button = start_move.button;
        let end_button = end_move.button;
        let button_paths = paths.iter().map(|path| path.to_vec()).collect();
        path_cache.insert((start_button, end_button), button_paths);
    }

    path_cache
}

fn build_keypad_seq(
    keys: &[Button],
    index: usize,
    prev_key: Button,
    curr_path: &[Button],
    keypad_path_cache: &HashMap<(Button, Button), Vec<Vec<Button>>>,
) -> Vec<Vec<Button>> {
    let mut result_path = vec![];

    if index == keys.len() {
        result_path.push(curr_path.to_vec());
        return result_path;
    }

    let curr_key = keys[index];
    let paths = &keypad_path_cache[&(prev_key, curr_key)];
    for path in paths {
        let mut new_path = curr_path.to_vec();
        new_path.extend_from_slice(path);
        new_path.push(Button::Activate);
        result_path.extend(build_keypad_seq(
            keys,
            index + 1,
            curr_key,
            &new_path,
            keypad_path_cache,
        ));
    }

    result_path
}

fn get_buttons_for_code(code: &str) -> Vec<Button> {
    code.chars()
        .map(|c| {
            match c {
                '0' => Some(Button::Zero),
                '1' => Some(Button::One),
                '2' => Some(Button::Two),
                '3' => Some(Button::Three),
                '4' => Some(Button::Four),
                '5' => Some(Button::Five),
                '6' => Some(Button::Six),
                '7' => Some(Button::Seven),
                '8' => Some(Button::Eight),
                '9' => Some(Button::Nine),
                'A' => Some(Button::Activate),
                _ => panic!("Invalid button"),
            }
            .unwrap()
        })
        .collect()
}

fn get_shortest_seq_len(
    buttons: &[Button],
    depth: usize,
    keypad_path_cache: &HashMap<(Button, Button), Vec<Vec<Button>>>,
    move_cache: &mut HashMap<(Vec<Button>, usize), usize>,
) -> usize {
    if depth == 0 {
        return buttons.len();
    }

    if move_cache.contains_key(&((*buttons).to_vec(), depth)) {
        return move_cache[&((*buttons).to_vec(), depth)];
    }

    let mut total = 0;

    buttons
        .split_inclusive(|&button| button == Button::Activate)
        .for_each(|sub_buttons| {
            let curr_path = vec![];
            let result_paths = build_keypad_seq(
                sub_buttons,
                0,
                Button::Activate,
                &curr_path,
                keypad_path_cache,
            );

            let shortest_seq_lens: Vec<usize> = result_paths
                .iter()
                .map(|result_path| {
                    get_shortest_seq_len(result_path, depth - 1, keypad_path_cache, move_cache)
                })
                .collect();

            if let Some(min_len) = shortest_seq_lens.into_iter().min() {
                total += min_len;
            }
        });

    move_cache.insert((buttons.to_vec(), depth), total);

    total
}

pub fn get_sum_complexity(input_file: &str, depth: usize) -> usize {
    let input = parse_input(input_file);

    let num_keypad: HashMap<(usize, usize), Option<Button>> = build_num_keypad();
    let dir_keypad: HashMap<(usize, usize), Option<Button>> = build_dir_keypad();

    let (num_keypad_graph, num_keypad_node_indices) = build_keypad_graph(&num_keypad);
    let (dir_keypad_graph, dir_keypad_node_indices) = build_keypad_graph(&dir_keypad);

    let num_keypad_move_cache = build_move_cache(
        &num_keypad,
        &GraphData {
            graph: num_keypad_graph.clone(),
            node_indices: num_keypad_node_indices.clone(),
        },
    );
    let num_keypad_path_cache = build_path_cache(&num_keypad_move_cache);
    let dir_keypad_move_cache = build_move_cache(
        &dir_keypad,
        &GraphData {
            graph: dir_keypad_graph.clone(),
            node_indices: dir_keypad_node_indices.clone(),
        },
    );
    let dir_keypad_path_cache = build_path_cache(&dir_keypad_move_cache);

    let keypad_path_cache = num_keypad_path_cache
        .into_iter()
        .chain(dir_keypad_path_cache)
        .collect();

    let mut move_cache: HashMap<(Vec<Button>, usize), usize> = HashMap::new();

    let mut sum_complexity = 0;

    for code in input.codes {
        let buttons = get_buttons_for_code(&code);
        let shortest_seq_len =
            get_shortest_seq_len(&buttons, depth, &keypad_path_cache, &mut move_cache);
        if let Ok(num_part) = code[0..code.len() - 1].parse::<usize>() {
            sum_complexity += shortest_seq_len * num_part;
        }
    }

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
    fn test_get_sum_complexity_test02() {
        assert_eq!(1972, get_sum_complexity("input/2024/day21_test02.txt", 3));
    }

    #[test]
    fn test_get_sum_complexity_test03() {
        assert_eq!(58800, get_sum_complexity("input/2024/day21_test03.txt", 3));
    }

    #[test]
    fn test_get_sum_complexity_test04() {
        assert_eq!(12172, get_sum_complexity("input/2024/day21_test04.txt", 3));
    }

    #[test]
    fn test_get_sum_complexity_test05() {
        assert_eq!(29184, get_sum_complexity("input/2024/day21_test05.txt", 3));
    }

    #[test]
    fn test_get_sum_complexity_test06() {
        assert_eq!(24256, get_sum_complexity("input/2024/day21_test06.txt", 3));
    }

    #[test]
    fn test_get_sum_complexity_test07() {
        assert_eq!(0, get_sum_complexity("input/2024/day21_test07.txt", 3));
    }

    #[test]
    fn test_get_sum_complexity_test08() {
        assert_eq!(9990, get_sum_complexity("input/2024/day21_test08.txt", 3));
    }

    #[test]
    fn test_get_sum_complexity() {
        assert_eq!(184180, get_sum_complexity("input/2024/day21.txt", 3));
    }

    #[test]
    fn test_get_sum_complexity_part_two_test01() {
        assert_eq!(
            154115708116294,
            get_sum_complexity("input/2024/day21_test01.txt", 26)
        );
    }

    #[test]
    fn test_get_sum_complexity_part_two_test02() {
        assert_eq!(
            2379451789590,
            get_sum_complexity("input/2024/day21_test02.txt", 26)
        );
    }

    #[test]
    fn test_get_sum_complexity_part_two_test03() {
        assert_eq!(
            70797185862200,
            get_sum_complexity("input/2024/day21_test03.txt", 26)
        );
    }

    #[test]
    fn test_get_sum_complexity_part_two_test04() {
        assert_eq!(
            14543936021812,
            get_sum_complexity("input/2024/day21_test04.txt", 26)
        );
    }

    #[test]
    fn test_get_sum_complexity_part_two_test05() {
        assert_eq!(
            36838581189648,
            get_sum_complexity("input/2024/day21_test05.txt", 26)
        );
    }

    #[test]
    fn test_get_sum_complexity_part_two_test06() {
        assert_eq!(
            29556553253044,
            get_sum_complexity("input/2024/day21_test06.txt", 26)
        );
    }

    #[test]
    fn test_get_sum_complexity_part_two_test07() {
        assert_eq!(0, get_sum_complexity("input/2024/day21_test07.txt", 26));
    }

    #[test]
    fn test_get_sum_complexity_part_two_test08() {
        assert_eq!(
            11835830901420,
            get_sum_complexity("input/2024/day21_test08.txt", 26)
        );
    }

    #[test]
    fn test_get_sum_complexity_part_two() {
        assert_eq!(
            231309103124520,
            get_sum_complexity("input/2024/day21.txt", 26)
        );
    }
}
