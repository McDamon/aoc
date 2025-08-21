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

/*fn print_mem_map(mem_map: &HashMap<(isize, isize), char>) {
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
}*/

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
            let point = Move { pos: (x, y) };
            let node_idx = graph.add_node(point);
            node_indices.insert(point, node_idx);
            //println!("Added node {:?}", graph[node_idx]);
        }
    }

    // Then add all edges in a separate pass
    for (&(x, y), &c) in mem_map.iter() {
        if c != '#' {
            for dir in Direction::all() {
                let point = Move { pos: (x, y) };
                let node_idx = node_indices[&point];

                // Forward movement
                let (dx, dy) = dir.to_delta();
                let (next_x, next_y) = (x + dx, y + dy);
                if let Some(&next_c) = mem_map.get(&(next_x, next_y))
                    && next_c != '#' {
                        let next_move = Move {
                            pos: (next_x, next_y),
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

fn build_mem_map(
    corrupt_mem: &[(isize, isize)],
    x_len: isize,
    y_len: isize,
    bytes: usize,
) -> HashMap<(isize, isize), char> {
    let mut mem_map = HashMap::new();
    for y in 0..y_len {
        for x in 0..x_len {
            mem_map.insert((x, y), '.');
        }
    }

    for (count, (x, y)) in corrupt_mem.iter().enumerate() {
        if count <= bytes {
            mem_map.insert((*x, *y), '#');
        } else {
            break;
        }
    }

    mem_map
}

pub fn get_min_steps(input_file: &str, x_len: isize, y_len: isize, bytes: usize) -> usize {
    let input = parse_input(input_file);

    //println!("Corrupt mem: {:?}", input.corrupt_mem);

    let mem_map: HashMap<(isize, isize), char> =
        build_mem_map(&input.corrupt_mem, x_len, y_len, bytes);

    //print_mem_map(&mem_map);

    let (graph, node_indices) = build_graph(&mem_map);

    let start = Move { pos: (0, 0) };
    let start_idx = node_indices[&start];

    let end = Move {
        pos: (x_len - 1, y_len - 1),
    };
    let end_idx = node_indices[&end];

    if let Some((distance, _path)) = algo::astar(
        &graph,
        start_idx,
        |finish| finish == end_idx,
        |e| *e.weight() as usize,
        |_| 0,
    ) {
        /*println!(
            "Found path from {:?} to {:?} with distance {}",
            start, end, distance
        );
        for node in path {
            println!("  {:?}", graph[node]);
        }*/
        return distance;
    }

    panic!("did not find path")
}

pub fn get_coords_first_byte_to_prevent_exit(
    input_file: &str,
    x_len: isize,
    y_len: isize,
) -> (isize, isize) {
    let input = parse_input(input_file);

    for (byte, byte_pos) in input.corrupt_mem.iter().enumerate() {
        //println!("Testing byte {:?} at index {:?}", byte_pos, byte);

        let mem_map: HashMap<(isize, isize), char> =
            build_mem_map(&input.corrupt_mem, x_len, y_len, byte);

        //print_mem_map(&mem_map);

        let (graph, node_indices) = build_graph(&mem_map);

        let start = Move { pos: (0, 0) };
        let start_idx = node_indices[&start];

        let end = Move {
            pos: (x_len - 1, y_len - 1),
        };
        let end_idx = node_indices[&end];

        if let Some((_distance, _path)) = algo::astar(
            &graph,
            start_idx,
            |finish| finish == end_idx,
            |e| *e.weight() as usize,
            |_| 0,
        ) {
            /*println!(
                "Found path from {:?} to {:?} with distance {}",
                start, end, distance
            );*/
        } else {
            //println!("Found blocking byte {:?} at index {:?}", *byte_pos, byte);
            return *byte_pos;
        }
    }

    panic!("did not find byte to prevent exit")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_min_steps_test01() {
        assert_eq!(22, get_min_steps("input/2024/day18_test01.txt", 7, 7, 11));
    }

    #[test]
    fn test_get_min_steps() {
        assert_eq!(302, get_min_steps("input/2024/day18.txt", 71, 71, 1023));
    }

    #[test]
    fn test_get_coords_first_byte_to_prevent_exit_test01() {
        assert_eq!(
            (6, 1),
            get_coords_first_byte_to_prevent_exit("input/2024/day18_test01.txt", 7, 7)
        );
    }

    #[ignore]
    #[test]
    fn test_get_coords_first_byte_to_prevent_exit() {
        assert_eq!(
            (24, 32),
            get_coords_first_byte_to_prevent_exit("input/2024/day18.txt", 71, 71)
        );
    }
}
