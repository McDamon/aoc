// https://adventofcode.com/2023/day/10

use enum_iterator::Sequence;
use grid::Grid;
use indextree::{Arena, NodeEdge, NodeId};
use itertools::Itertools;

use crate::utils::get_lines;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Sequence)]
#[repr(u8)]
enum Pipe {
    #[default]
    Ground = b'.',
    VerticalNS = b'|',
    HorizontalEW = b'-',
    NE90DegLSym = b'L',
    NW90DegJSym = b'J',
    SW90Deg7Sym = b'7',
    SE90DegFSym = b'F',
    StartPos = b'S',
    Inside = b'1',
    Outside = b'0',
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl TryFrom<u8> for Pipe {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Pipe::Ground as u8 => Ok(Pipe::Ground),
            x if x == Pipe::VerticalNS as u8 => Ok(Pipe::VerticalNS),
            x if x == Pipe::HorizontalEW as u8 => Ok(Pipe::HorizontalEW),
            x if x == Pipe::NE90DegLSym as u8 => Ok(Pipe::NE90DegLSym),
            x if x == Pipe::NW90DegJSym as u8 => Ok(Pipe::NW90DegJSym),
            x if x == Pipe::SW90Deg7Sym as u8 => Ok(Pipe::SW90Deg7Sym),
            x if x == Pipe::SE90DegFSym as u8 => Ok(Pipe::SE90DegFSym),
            x if x == Pipe::StartPos as u8 => Ok(Pipe::StartPos),
            x if x == Pipe::Inside as u8 => Ok(Pipe::Inside),
            x if x == Pipe::Outside as u8 => Ok(Pipe::Outside),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Input {
    tiles: Grid<Pipe>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        tiles: parse_tiles(iter.next().unwrap().to_owned()),
    }
}

fn parse_tiles(tiles_lines: Vec<String>) -> Grid<Pipe> {
    let mut tiles = Grid::new(0, 0);
    for tiles_line in tiles_lines.into_iter() {
        let mut tiles_entries: Vec<Pipe> = Vec::new();
        for tiles_entry in tiles_line.chars() {
            match Pipe::try_from(tiles_entry as u8) {
                Ok(pipe) => tiles_entries.push(pipe),
                Err(_) => panic!("Invalid pipe"),
            }
        }
        tiles.push_row(tiles_entries)
    }
    tiles
}

#[derive(Debug, Copy, Clone)]
struct Entry {
    pipe: Pipe,
    pos: (usize, usize),
    direction: Option<Direction>,
}

fn get_farthest_steps(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let mut start_pos: Option<(usize, usize)> = None;
    for ((row, col), pipe) in input.tiles.indexed_iter() {
        if *pipe == Pipe::StartPos {
            start_pos = Some((row, col));
        }
    }

    if let Some((row, col)) = start_pos {
        let mut arena: Arena<Entry> = Arena::new();
        let root_node = arena.new_node(Entry {
            pipe: Pipe::StartPos,
            pos: (row, col),
            direction: None,
        });

        build_tree(&input.tiles, &mut arena, None, root_node);

        let traverser = root_node.traverse(&arena);
        let mut node_ids: Vec<NodeId> = vec![];
        for ev in traverser {
            match ev {
                indextree::NodeEdge::Start(id) => node_ids.push(id),
                _ => break,
            };
        }
        let res = node_ids.len() / 2;
        println!("{}", res);
        res
    } else {
        panic!("Invalid start node");
    }
}

fn build_tree(
    tiles: &Grid<Pipe>,
    arena: &mut Arena<Entry>,
    maybe_prev_index: Option<NodeId>,
    current_index: NodeId,
) {
    let maybe_node = arena.get_mut(current_index);
    if let Some(current_node) = maybe_node {
        let current_path = current_node.get().pipe;
        let current_row: i64 = current_node.get().pos.0 as i64;
        let current_col: i64 = current_node.get().pos.1 as i64;
        // Loop through all the potential new nodes to check next direction
        for test_row in (current_row - 1)..(current_row + 2) {
            for test_col in (current_col - 1)..(current_col + 2) {
                let maybe_direction = get_direction(
                    (current_row as i32, current_col as i32),
                    (test_row as i32, test_col as i32),
                );
                if let Some(next_direction) = maybe_direction {
                    // Don't consider our current node, and only consider NSEW
                    if !(current_row == test_row && current_col == test_col)
                        && (next_direction == Direction::N
                            || next_direction == Direction::S
                            || next_direction == Direction::E
                            || next_direction == Direction::W)
                    {
                        if let Some(next_pipe) = tiles.get(test_row, test_col) {
                            if let Some(prev_index) = maybe_prev_index {
                                let maybe_prev_node = arena.get(prev_index);
                                if let Some(prev_node) = maybe_prev_node {
                                    if prev_node.get().pos != (test_row as usize, test_col as usize)
                                        && is_pipe_connected(
                                            current_path,
                                            *next_pipe,
                                            next_direction,
                                        )
                                    {
                                        let new_node = arena.new_node(Entry {
                                            pipe: *next_pipe,
                                            pos: (test_row as usize, test_col as usize),
                                            direction: Some(next_direction),
                                        });
                                        current_index.append(new_node, arena);
                                        build_tree(tiles, arena, Some(current_index), new_node)
                                    }
                                }
                            } else if is_pipe_connected(current_path, *next_pipe, next_direction) {
                                let new_node = arena.new_node(Entry {
                                    pipe: *next_pipe,
                                    pos: (test_row as usize, test_col as usize),
                                    direction: Some(next_direction),
                                });
                                current_index.append(new_node, arena);
                                build_tree(tiles, arena, Some(current_index), new_node)
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_pipe_connected(current_pipe: Pipe, next_pipe: Pipe, next_direction: Direction) -> bool {
    fn is_north_pipe(next_pipe: Pipe) -> bool {
        matches!(
            next_pipe,
            Pipe::VerticalNS | Pipe::SW90Deg7Sym | Pipe::SE90DegFSym
        )
    }
    fn is_south_pipe(next_pipe: Pipe) -> bool {
        matches!(
            next_pipe,
            Pipe::VerticalNS | Pipe::NE90DegLSym | Pipe::NW90DegJSym
        )
    }
    fn is_east_pipe(next_pipe: Pipe) -> bool {
        matches!(
            next_pipe,
            Pipe::HorizontalEW | Pipe::NW90DegJSym | Pipe::SW90Deg7Sym
        )
    }
    fn is_west_pipe(next_pipe: Pipe) -> bool {
        matches!(
            next_pipe,
            Pipe::HorizontalEW | Pipe::NE90DegLSym | Pipe::SE90DegFSym
        )
    }
    match current_pipe {
        Pipe::Ground => false,
        Pipe::VerticalNS => match next_direction {
            Direction::N => is_north_pipe(next_pipe),
            Direction::S => is_south_pipe(next_pipe),
            _ => false,
        },
        Pipe::HorizontalEW => match next_direction {
            Direction::E => is_east_pipe(next_pipe),
            Direction::W => is_west_pipe(next_pipe),
            _ => false,
        },
        Pipe::NE90DegLSym => match next_direction {
            Direction::N => is_north_pipe(next_pipe),
            Direction::E => is_east_pipe(next_pipe),
            _ => false,
        },
        Pipe::NW90DegJSym => match next_direction {
            Direction::N => is_north_pipe(next_pipe),
            Direction::W => is_west_pipe(next_pipe),
            _ => false,
        },
        Pipe::SW90Deg7Sym => match next_direction {
            Direction::S => is_south_pipe(next_pipe),
            Direction::W => is_west_pipe(next_pipe),
            _ => false,
        },
        Pipe::SE90DegFSym => match next_direction {
            Direction::E => is_east_pipe(next_pipe),
            Direction::S => is_south_pipe(next_pipe),
            _ => false,
        },
        Pipe::StartPos => match next_direction {
            Direction::N => is_north_pipe(next_pipe),
            Direction::E => is_east_pipe(next_pipe),
            Direction::S => is_south_pipe(next_pipe),
            Direction::W => is_west_pipe(next_pipe),
        },
        _ => false,
    }
}

fn get_direction(
    (current_row, current_col): (i32, i32),
    (next_row, next_col): (i32, i32),
) -> Option<Direction> {
    if current_row - 1 == next_row && current_col == next_col {
        return Some(Direction::N);
    } else if current_row == next_row && current_col + 1 == next_col {
        return Some(Direction::E);
    } else if current_row + 1 == next_row && current_col == next_col {
        return Some(Direction::S);
    } else if current_row == next_row && current_col - 1 == next_col {
        return Some(Direction::W);
    }
    None
}

fn get_enclosed_by_loop(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let mut start_pos: Option<(usize, usize)> = None;
    for ((row, col), pipe) in input.tiles.indexed_iter() {
        if *pipe == Pipe::StartPos {
            start_pos = Some((row, col));
        }
    }

    if let Some((row, col)) = start_pos {
        let mut arena: Arena<Entry> = Arena::new();
        let root_node = arena.new_node(Entry {
            pipe: Pipe::StartPos,
            pos: (row, col),
            direction: None,
        });

        build_tree(&input.tiles, &mut arena, None, root_node);

        let cleaned_tiles = clean_tiles(&input.tiles, &arena, &root_node);

        // Was stumped by the squeezing between pipes (my initial raycasting approach using even-odd failed)
        // Inspired/plagiarised from https://nickymeuleman.netlify.app/garden/aoc2023-day10#final-code
        // TODO: replace this with shoelace or something similar
        let mut inside = false;
        cleaned_tiles
            .flatten()
            .iter()
            .filter(|tile| match tile {
                Pipe::Ground => inside,
                Pipe::VerticalNS | Pipe::NW90DegJSym | Pipe::NE90DegLSym => {
                    inside = !inside;
                    false
                }
                _ => false,
            })
            .count()
    } else {
        panic!("Invalid start node");
    }
}

fn print_tiles(tiles: &Grid<Pipe>) {
    for tile_row in tiles.iter_rows() {
        for tile in tile_row {
            print!("{:#}", *tile as u8 as char);
        }
        println!();
    }
}

fn clean_tiles(tiles: &Grid<Pipe>, arena: &Arena<Entry>, root_node: &NodeId) -> Grid<Pipe> {
    let mut cleaned_tiles: Grid<Pipe> = Grid::new(tiles.rows(), tiles.cols());
    for (pos, pipe) in tiles.indexed_iter() {
        if *pipe == Pipe::StartPos {
            cleaned_tiles[pos] = clean_start_tile(arena, root_node);
        } else {
            cleaned_tiles[pos] = clean_tile(arena, root_node, pos);
        }
    }
    cleaned_tiles
}

fn clean_start_tile(arena: &Arena<Entry>, root_node: &NodeId) -> Pipe {
    let start_edge_tiles: Vec<NodeId> = root_node.children(arena).collect_vec();
    let mut start_edge_nodes: Vec<Entry> = vec![];
    for edge_tile in start_edge_tiles {
        start_edge_nodes.push(*arena[edge_tile].get());
    }
    let count_north = start_edge_nodes
        .iter()
        .filter(|&t| t.direction.unwrap() == Direction::N)
        .count();
    let count_south = start_edge_nodes
        .iter()
        .filter(|&t| t.direction.unwrap() == Direction::S)
        .count();
    let count_east = start_edge_nodes
        .iter()
        .filter(|&t| t.direction.unwrap() == Direction::E)
        .count();
    let count_west = start_edge_nodes
        .iter()
        .filter(|&t| t.direction.unwrap() == Direction::W)
        .count();

    if count_north == 1 && count_west == 1 {
        return Pipe::NW90DegJSym;
    } else if count_north == 1 && count_east == 1 {
        return Pipe::NE90DegLSym;
    } else if count_south == 1 && count_west == 1 {
        return Pipe::SW90Deg7Sym;
    } else if count_south == 1 && count_east == 1 {
        return Pipe::SE90DegFSym;
    }
    Pipe::Ground
}

fn clean_tile(arena: &Arena<Entry>, root_node: &NodeId, pos: (usize, usize)) -> Pipe {
    let mut maybe_next = Some(NodeEdge::Start(*root_node));
    while let Some(current) = maybe_next {
        maybe_next = current.next_traverse(arena);
        let current = match current {
            NodeEdge::Start(id) => id,
            NodeEdge::End(_) => break,
        };

        let tile = arena[current].get();
        if pos == tile.pos {
            return tile.pipe;
        }
    }
    Pipe::Ground
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_farthest_steps_test01() {
        assert_eq!(4, get_farthest_steps("input/day10_test01.txt"));
    }

    #[test]
    fn test_get_farthest_steps_test02() {
        assert_eq!(8, get_farthest_steps("input/day10_test02.txt"));
    }

    #[test]
    fn test_get_farthest_steps() {
        assert_eq!(6823, get_farthest_steps("input/day10.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test01() {
        assert_eq!(1, get_enclosed_by_loop("input/2023/day10_test01.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test02() {
        assert_eq!(1, get_enclosed_by_loop("input/2023/day10_test02.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test03() {
        assert_eq!(4, get_enclosed_by_loop("input/2023/day10_test03.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test04() {
        assert_eq!(4, get_enclosed_by_loop("input/2023/day10_test04.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test05() {
        assert_eq!(8, get_enclosed_by_loop("input/2023/day10_test05.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test06() {
        assert_eq!(10, get_enclosed_by_loop("input/2023/day10_test06.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test07() {
        assert_eq!(4, get_enclosed_by_loop("input/2023/day10_test07.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test08() {
        assert_eq!(4, get_enclosed_by_loop("input/2023/day10_test08.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test09() {
        assert_eq!(4, get_enclosed_by_loop("input/2023/day10_test09.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test10() {
        assert_eq!(4, get_enclosed_by_loop("input/2023/day10_test10.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test11() {
        assert_eq!(4, get_enclosed_by_loop("input/2023/day10_test11.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_test12() {
        assert_eq!(4, get_enclosed_by_loop("input/2023/day10_test12.txt"));
    }

    #[test]
    fn test_get_enclosed_by_loop_steps() {
        assert_eq!(415, get_enclosed_by_loop("input/2023/day10.txt"));
    }
}
