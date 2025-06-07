// https://adventofcode.com/2019/day/3

use std::collections::HashSet;

use crate::utils::{get_lines, manhattan_distance_i, MoveDir};

struct WireMove {
    dir: MoveDir,
    dist: isize,
}

struct Input {
    wire1_path: Vec<WireMove>,
    wire2_path: Vec<WireMove>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let wire1_path: Vec<WireMove> = parse_wire_path(&lines[0]);
    let wire2_path: Vec<WireMove> = parse_wire_path(&lines[1]);

    Input {
        wire1_path,
        wire2_path,
    }
}

fn parse_wire_path(line: &str) -> Vec<WireMove> {
    line.split(',')
        .map(|s| {
            let (dir, dist) = s.split_at(1);
            let dir = match dir {
                "U" => MoveDir::Up,
                "D" => MoveDir::Down,
                "L" => MoveDir::Left,
                "R" => MoveDir::Right,
                _ => panic!("Invalid direction"),
            };
            let dist = dist.parse::<isize>().expect("Invalid distance");
            WireMove { dir, dist }
        })
        .collect()
}

fn get_wire_moves(wire_path: Vec<WireMove>, start_pos: (isize, isize)) -> HashSet<(isize, isize)> {
    let mut wire_moves: HashSet<(isize, isize)> = HashSet::new();
    let mut wire_pos = start_pos;

    for wire_move in &wire_path {
        match wire_move.dir {
            MoveDir::Up => {
                let start_y = wire_pos.1;
                for i in 1..wire_move.dist + 1 {
                    let new_pos = (wire_pos.0, start_y + i);
                    wire_moves.insert(new_pos);
                    wire_pos = new_pos;
                }
            }
            MoveDir::Down => {
                let start_y = wire_pos.1;
                for i in 1..wire_move.dist + 1 {
                    let new_pos = (wire_pos.0, start_y - i);
                    wire_moves.insert(new_pos);
                    wire_pos = new_pos;
                }
            }
            MoveDir::Left => {
                let start_x = wire_pos.0;
                for i in 1..wire_move.dist + 1 {
                    let new_pos = (start_x - i, wire_pos.1);
                    wire_moves.insert(new_pos);
                    wire_pos = new_pos;
                }
            }
            MoveDir::Right => {
                let start_x = wire_pos.0;
                for i in 1..wire_move.dist + 1 {
                    let new_pos = (start_x + i, wire_pos.1);
                    wire_moves.insert(new_pos);
                    wire_pos = new_pos;
                }
            }
        }
    }

    wire_moves
}

pub fn get_closest_dist(input_file: &str) -> isize {
    let input = parse_input(input_file);

    let start_pos = (0isize, 0isize);

    let wire1_moves = get_wire_moves(input.wire1_path, start_pos);

    let wire2_moves = get_wire_moves(input.wire2_path, start_pos);

    let intersections: Vec<(isize, isize)> =
        wire1_moves.intersection(&wire2_moves).cloned().collect();

    let mut manhattan_distances = vec![];

    for intersection in intersections {
        manhattan_distances.push(manhattan_distance_i(start_pos, intersection));
    }

    return *manhattan_distances.iter().min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_closest_dist_test01() {
        assert_eq!(6, get_closest_dist("input/2019/day03_test01.txt"));
    }

    #[test]
    fn test_get_closest_dist_test02() {
        assert_eq!(159, get_closest_dist("input/2019/day03_test02.txt"));
    }

    #[test]
    fn test_get_closest_dist_test03() {
        assert_eq!(135, get_closest_dist("input/2019/day03_test03.txt"));
    }

    #[test]
    fn test_get_closest_dist() {
        assert_eq!(293, get_closest_dist("input/2019/day03.txt"));
    }
}
