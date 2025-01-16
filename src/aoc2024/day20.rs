// https://adventofcode.com/2024/day/20

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use petgraph::{
    algo,
    graph::{DiGraph, NodeIndex},
    Graph,
};

use crate::utils::{get_lines, Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move {
    pos: (isize, isize),
}

#[derive(Debug)]
struct Input {
    track: HashMap<(isize, isize), char>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut track: HashMap<(isize, isize), char> = HashMap::new();

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            track.insert((x as isize, y as isize), c);
        }
    }

    Input { track }
}

fn print_track(track: &HashMap<(isize, isize), char>) {
    let max_x = track.keys().map(|(x, _)| *x).max().unwrap();
    let max_y = track.keys().map(|(_, y)| *y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(c) = track.get(&(x, y)) {
                print!("{}", c);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn build_graph(
    track: &HashMap<(isize, isize), char>,
) -> (Graph<Move, f64>, HashMap<Move, NodeIndex>) {
    // Create directed graph
    let mut graph = DiGraph::<Move, f64>::new();
    let mut node_indices: HashMap<Move, NodeIndex> = HashMap::new();

    // Add nodes and edges
    // First create all nodes
    for (&(x, y), &c) in track.iter() {
        if c != '#' {
            let point = Move { pos: (x, y) };
            let node_idx = graph.add_node(point);
            node_indices.insert(point, node_idx);
            //println!("Added node {:?}", graph[node_idx]);
        }
    }

    // Then a~dd all edges in a separate pass
    for (&(x, y), &c) in track.iter() {
        if c != '#' {
            for dir in Direction::all() {
                let point = Move { pos: (x, y) };
                let node_idx = node_indices[&point];

                // Forward movement
                let (dx, dy) = dir.to_delta();
                let (next_x, next_y) = (x + dx, y + dy);
                if let Some(&next_c) = track.get(&(next_x, next_y)) {
                    if next_c != '#' {
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
    }
    (graph, node_indices)
}

fn get_cheats(track: &HashMap<(isize, isize), char>) -> HashSet<(isize, isize)> {
    let mut cheats = HashSet::new();
    for ((track_x, track_y), track_entry) in track.iter() {
        if *track_entry != '#' {
            for dir in Direction::all() {
                let (dx, dy) = dir.to_delta();
                let (next_x, next_y) = (track_x + dx, track_y + dy);
                let (next_x2, next_y2) = (next_x + dx, next_y + dy);
                if let Some(&next_c) = track.get(&(next_x, next_y))
                    && let Some(&next_c2) = track.get(&(next_x2, next_y2))
                    && next_c == '#'
                    && next_c2 == '.'
                {
                    cheats.insert((next_x, next_y));
                }
            }
        }
    }
    cheats
}

fn get_distance(
    start: (isize, isize),
    end: (isize, isize),
    track: &HashMap<(isize, isize), char>,
) -> Option<usize> {
    let (graph, node_indices) = build_graph(track);

    let start_move = Move { pos: start };
    let end_move = Move { pos: end };

    let start_idx = node_indices[&start_move];
    let end_idx = node_indices[&end_move];

    if let Some((distance, _path)) = algo::astar(
        &graph,
        start_idx,
        |finish| finish == end_idx,
        |e| *e.weight() as usize,
        |_| 0,
    ) {
        Some(distance)
    } else {
        None
    }
}

fn get_num_cheats(input_file: &str, required_savings: usize, is_at_least: bool) -> usize {
    let input = parse_input(input_file);

    // Find start and end points
    let start = input
        .track
        .iter()
        .find(|(_, c)| **c == 'S')
        .map(|(&pos, _)| pos)
        .unwrap();
    let end = input
        .track
        .iter()
        .find(|(_, c)| **c == 'E')
        .map(|(&pos, _)| pos)
        .unwrap();

    let cheats = get_cheats(&input.track);

    //println!("Cheats: {:?}", cheats);

    let mut cheat_savings: HashMap<(isize, isize), usize> = HashMap::new();

    let no_cheat_distance = if let Some(distance) = get_distance(start, end, &input.track) {
        distance
    } else {
        return 0;
    };

    for cheat in cheats {
        let mut track = input.track.clone();
        track.insert(cheat, '.');

        //println!("***NEW CHEAT {:?} ***", cheat);
        //println!("");
        //print_track(&track);

        if let Some(distance) = get_distance(start, end, &track) {
            let cheat_saving = no_cheat_distance - distance;
            //println!("Cheat {:?} saving: {:?}", cheat, cheat_saving);
            cheat_savings.insert(cheat, cheat_saving);
        }

        //println!("");
    }

    cheat_savings
        .values()
        .filter(|&&c| {
            if is_at_least {
                c >= required_savings
            } else {
                c == required_savings
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_cheats_test01() {
        assert_eq!(14, get_num_cheats("input/2024/day20_test01.txt", 2, false));
    }

    #[test]
    fn test_get_num_cheats_test02() {
        assert_eq!(14, get_num_cheats("input/2024/day20_test01.txt", 4, false));
    }

    #[test]
    fn test_get_num_cheats_test03() {
        assert_eq!(2, get_num_cheats("input/2024/day20_test01.txt", 6, false));
    }

    #[test]
    fn test_get_num_cheats_test04() {
        assert_eq!(4, get_num_cheats("input/2024/day20_test01.txt", 8, false));
    }

    #[test]
    fn test_get_num_cheats_test05() {
        assert_eq!(2, get_num_cheats("input/2024/day20_test01.txt", 10, false));
    }

    #[test]
    fn test_get_num_cheats_test06() {
        assert_eq!(3, get_num_cheats("input/2024/day20_test01.txt", 12, false));
    }

    #[test]
    fn test_get_num_cheats_test07() {
        assert_eq!(1, get_num_cheats("input/2024/day20_test01.txt", 20, false));
    }

    #[test]
    fn test_get_num_cheats_test08() {
        assert_eq!(1, get_num_cheats("input/2024/day20_test01.txt", 36, false));
    }

    #[test]
    fn test_get_num_cheats_test09() {
        assert_eq!(1, get_num_cheats("input/2024/day20_test01.txt", 38, false));
    }

    #[test]
    fn test_get_num_cheats_test10() {
        assert_eq!(1, get_num_cheats("input/2024/day20_test01.txt", 40, false));
    }

    #[test]
    fn test_get_num_cheats_test11() {
        assert_eq!(1, get_num_cheats("input/2024/day20_test01.txt", 64, false));
    }

    #[test]
    fn test_get_num_cheats() {
        assert_eq!(1399, get_num_cheats("input/2024/day20.txt", 100, true));
    }
}
