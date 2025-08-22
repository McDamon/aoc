// https://adventofcode.com/2023/day/10

use crate::utils::{get_lines, Direction};

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum Pipe {
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
pub struct Input {
    pub tiles: Vec<Vec<Pipe>>,
}

pub fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        tiles: parse_tiles(iter.next().unwrap().to_owned()),
    }
}

pub fn parse_tiles(tiles_lines: Vec<String>) -> Vec<Vec<Pipe>> {
    let mut tiles = Vec::new();
    for tiles_line in tiles_lines.into_iter() {
        let mut tiles_entries: Vec<Pipe> = Vec::new();
        for tiles_entry in tiles_line.chars() {
            match Pipe::try_from(tiles_entry as u8) {
                Ok(pipe) => tiles_entries.push(pipe),
                Err(_) => panic!("Invalid pipe"),
            }
        }
        tiles.push(tiles_entries)
    }
    tiles
}

pub fn get_farthest_steps(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let mut start_pos: Option<(usize, usize)> = None;
    for (row, tile_row) in input.tiles.iter().enumerate() {
        for (col, pipe) in tile_row.iter().enumerate() {
            if *pipe == Pipe::StartPos {
                start_pos = Some((row, col));
                break;
            }
        }
        if start_pos.is_some() {
            break;
        }
    }

    if let Some(start) = start_pos {
        let loop_path = find_main_loop(&input.tiles, start);
        loop_path.len() / 2
    } else {
        panic!("Invalid start node");
    }
}

// Find the main loop by following connected pipes
fn find_main_loop(tiles: &Vec<Vec<Pipe>>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = vec![start];
    let mut current = start;
    let mut came_from: Option<(usize, usize)> = None;
    
    loop {
        let mut found_next = false;
        
        // Check all 4 directions
        for direction in [Direction::N, Direction::S, Direction::E, Direction::W] {
            if let Some(next_pos) = get_next_position(current, direction, tiles.len(), tiles[0].len()) {
                // Don't go back where we came from
                if Some(next_pos) == came_from {
                    continue;
                }
                
                let current_pipe = tiles[current.0][current.1];
                let next_pipe = tiles[next_pos.0][next_pos.1];
                
                if is_pipe_connected(current_pipe, next_pipe, direction) {
                    // If we've reached the start again, we've completed the loop
                    if next_pos == start && path.len() > 2 {
                        return path;
                    }
                    
                    // If we haven't visited this position yet, continue
                    if !path.contains(&next_pos) {
                        came_from = Some(current);
                        current = next_pos;
                        path.push(current);
                        found_next = true;
                        break;
                    }
                }
            }
        }
        
        if !found_next {
            break;
        }
    }
    
    path
}

fn get_next_position(pos: (usize, usize), direction: Direction, max_row: usize, max_col: usize) -> Option<(usize, usize)> {
    let (row, col) = pos;
    match direction {
        Direction::N => {
            if row > 0 {
                Some((row - 1, col))
            } else {
                None
            }
        }
        Direction::S => {
            if row + 1 < max_row {
                Some((row + 1, col))
            } else {
                None
            }
        }
        Direction::E => {
            if col + 1 < max_col {
                Some((row, col + 1))
            } else {
                None
            }
        }
        Direction::W => {
            if col > 0 {
                Some((row, col - 1))
            } else {
                None
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

pub fn get_enclosed_by_loop(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let mut start_pos: Option<(usize, usize)> = None;
    for (row, tile_row) in input.tiles.iter().enumerate() {
        for (col, pipe) in tile_row.iter().enumerate() {
            if *pipe == Pipe::StartPos {
                start_pos = Some((row, col));
                break;
            }
        }
        if start_pos.is_some() {
            break;
        }
    }

    if let Some(start) = start_pos {
        let loop_path = find_main_loop(&input.tiles, start);
        let loop_set: std::collections::HashSet<(usize, usize)> = loop_path.into_iter().collect();
        
        let cleaned_tiles = clean_tiles_simple(&input.tiles, &loop_set, start);

        // Use the ray casting approach to count inside tiles
        let mut inside = false;
        cleaned_tiles
            .iter()
            .flatten()
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

fn clean_tiles_simple(tiles: &Vec<Vec<Pipe>>, loop_set: &std::collections::HashSet<(usize, usize)>, start: (usize, usize)) -> Vec<Vec<Pipe>> {
    let rows = tiles.len();
    let cols = if rows > 0 { tiles[0].len() } else { 0 };
    let mut cleaned_tiles: Vec<Vec<Pipe>> = vec![vec![Pipe::Ground; cols]; rows];
    
    for (row, tile_row) in tiles.iter().enumerate() {
        for (col, pipe) in tile_row.iter().enumerate() {
            let pos = (row, col);
            if loop_set.contains(&pos) {
                if pos == start {
                    cleaned_tiles[row][col] = infer_start_pipe(tiles, start);
                } else {
                    cleaned_tiles[row][col] = *pipe;
                }
            }
            // Everything else stays as Ground
        }
    }
    cleaned_tiles
}

fn infer_start_pipe(tiles: &Vec<Vec<Pipe>>, start: (usize, usize)) -> Pipe {
    let mut connections = Vec::new();
    
    // Check all four directions to see what connects to the start
    for direction in [Direction::N, Direction::S, Direction::E, Direction::W] {
        if let Some(next_pos) = get_next_position(start, direction, tiles.len(), tiles[0].len()) {
            let next_pipe = tiles[next_pos.0][next_pos.1];
            if is_pipe_connected(Pipe::StartPos, next_pipe, direction) {
                connections.push(direction);
            }
        }
    }
    
    // Determine the pipe type based on connections
    match connections.as_slice() {
        [Direction::N, Direction::S] | [Direction::S, Direction::N] => Pipe::VerticalNS,
        [Direction::E, Direction::W] | [Direction::W, Direction::E] => Pipe::HorizontalEW,
        [Direction::N, Direction::E] | [Direction::E, Direction::N] => Pipe::NE90DegLSym,
        [Direction::N, Direction::W] | [Direction::W, Direction::N] => Pipe::NW90DegJSym,
        [Direction::S, Direction::W] | [Direction::W, Direction::S] => Pipe::SW90Deg7Sym,
        [Direction::S, Direction::E] | [Direction::E, Direction::S] => Pipe::SE90DegFSym,
        _ => Pipe::Ground,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_farthest_steps_test01() {
        assert_eq!(4, get_farthest_steps("input/2023/day10_test01.txt"));
    }

    #[test]
    fn test_get_farthest_steps_test02() {
        assert_eq!(8, get_farthest_steps("input/2023/day10_test02.txt"));
    }

    #[test]
    fn test_get_farthest_steps() {
        assert_eq!(6823, get_farthest_steps("input/2023/day10.txt"));
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
