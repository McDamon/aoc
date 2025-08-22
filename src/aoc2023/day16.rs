// https://adventofcode.com/2023/day/16

use std::collections::HashSet;

use crate::utils::{Direction, get_lines};

#[derive(Debug)]
pub struct Input {
    pub tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, PartialEq, Eq, Default, Copy, Clone)]
#[repr(u8)]
pub enum Tile {
    #[default]
    Empty = b'.',
    MirrorForward = b'/',
    MirrorBack = b'\\',
    SplitterVert = b'|',
    SplitterHoriz = b'-',
    Energized = b'#',
}

impl TryFrom<u8> for Tile {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Tile::Empty as u8 => Ok(Tile::Empty),
            x if x == Tile::MirrorForward as u8 => Ok(Tile::MirrorForward),
            x if x == Tile::MirrorBack as u8 => Ok(Tile::MirrorBack),
            x if x == Tile::SplitterVert as u8 => Ok(Tile::SplitterVert),
            x if x == Tile::SplitterHoriz as u8 => Ok(Tile::SplitterHoriz),
            _ => Err(()),
        }
    }
}

pub fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        tiles: parse_tiles(iter.next().unwrap().to_owned()),
    }
}

pub fn parse_tiles(tiles_lines: Vec<String>) -> Vec<Vec<Tile>> {
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    for tiles_line in tiles_lines.into_iter() {
        let mut tiles_entries: Vec<Tile> = Vec::new();
        for tiles_entry in tiles_line.chars() {
            match Tile::try_from(tiles_entry as u8) {
                Ok(tile) => tiles_entries.push(tile),
                Err(_) => panic!("Invalid tile"),
            }
        }
        tiles.push(tiles_entries)
    }
    tiles
}

pub struct Raytracer {
    tiles: Vec<Vec<Tile>>,
}

impl Raytracer {
    pub fn raytrace(
        &mut self,
        index: &mut usize,
        (mut row, mut col): (usize, usize),
        mut dir: Direction,
        visited_tiles: &mut HashSet<(usize, usize, Direction)>,
    ) {
        *index += 1;
        loop {
            if !visited_tiles.contains(&(row, col, dir)) {
                visited_tiles.insert((row, col, dir));
            } else {
                break;
            }

            if let Some(tile) = self.tiles.get(row).and_then(|r| r.get(col)) {
                match tile {
                    Tile::MirrorForward => match dir {
                        Direction::N => dir = Direction::E,
                        Direction::S => dir = Direction::W,
                        Direction::E => dir = Direction::N,
                        Direction::W => dir = Direction::S,
                        Direction::Stop => (),
                    },
                    Tile::MirrorBack => match dir {
                        Direction::N => dir = Direction::W,
                        Direction::S => dir = Direction::E,
                        Direction::E => dir = Direction::S,
                        Direction::W => dir = Direction::N,
                        Direction::Stop => (),
                    },
                    Tile::SplitterVert => match dir {
                        Direction::N | Direction::S => (),
                        Direction::E | Direction::W => {
                            let mut rt = Raytracer {
                                tiles: self.tiles.clone(),
                            };

                            dir = Direction::Stop;

                            rt.raytrace(index, (row, col), Direction::N, visited_tiles);
                            rt.raytrace(index, (row, col), Direction::S, visited_tiles);
                        }
                        Direction::Stop => (),
                    },
                    Tile::SplitterHoriz => match dir {
                        Direction::N | Direction::S => {
                            let mut rt = Raytracer {
                                tiles: self.tiles.clone(),
                            };

                            dir = Direction::Stop;

                            rt.raytrace(index, (row, col), Direction::E, visited_tiles);
                            rt.raytrace(index, (row, col), Direction::W, visited_tiles);
                        }
                        Direction::E | Direction::W => (),
                        Direction::Stop => (),
                    },
                    _ => (),
                }
            }

            match dir {
                Direction::N => {
                    if row > 0 {
                        row -= 1;
                    } else {
                        break;
                    }
                }
                Direction::S => {
                    if row < self.tiles.len() - 1 {
                        row += 1;
                    } else {
                        break;
                    }
                }
                Direction::E => {
                    if col < self.tiles[row].len() - 1 {
                        col += 1;
                    } else {
                        break;
                    }
                }
                Direction::W => {
                    if col > 0 {
                        col -= 1;
                    } else {
                        break;
                    }
                }
                Direction::Stop => (),
            }
        }
    }
}

pub fn get_energized_tiles(input_file: &str) -> usize {
    let input = parse_input(input_file);

    get_energized_tiles_count((0, 0), Direction::E, &input.tiles)
}

pub fn print_tiles(tiles: &[Vec<Tile>]) {
    for tile_row in tiles.iter() {
        for tile in tile_row {
            print!("{:#}", *tile as u8 as char);
        }
        println!();
    }
}

pub fn get_energized_tiles_count(
    pos: (usize, usize),
    dir: Direction,
    tiles: &[Vec<Tile>],
) -> usize {
    let mut visited_tiles: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut rt = Raytracer {
        tiles: tiles.to_vec(),
    };

    let mut index = 0;
    rt.raytrace(&mut index, pos, dir, &mut visited_tiles);

    for (row, col, _) in &visited_tiles {
        if let Some(energized_tile) = rt.tiles.get_mut(*row).and_then(|r| r.get_mut(*col)) {
            *energized_tile = Tile::Energized;
        }
    }

    let mut energized_tile_count = 0;
    for tile in rt.tiles.iter() {
        for t in tile {
            if *t == Tile::Energized {
                energized_tile_count += 1;
            }
        }
    }

    //print_tiles(&rt.tiles);

    energized_tile_count
}

pub fn get_max_energized_tiles(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let mut energized_tiles_vec: Vec<usize> = vec![];

    for left_index in 0..input.tiles.len() {
        energized_tiles_vec.push(get_energized_tiles_count(
            (left_index, 0),
            Direction::E,
            &input.tiles,
        ));
    }

    for right_index in 0..input.tiles.len() {
        energized_tiles_vec.push(get_energized_tiles_count(
            (right_index, input.tiles[0].len() - 1),
            Direction::W,
            &input.tiles,
        ));
    }

    for top_index in 0..input.tiles[0].len() {
        energized_tiles_vec.push(get_energized_tiles_count(
            (0, top_index),
            Direction::S,
            &input.tiles,
        ));
    }

    for bottom_index in 0..input.tiles[0].len() {
        energized_tiles_vec.push(get_energized_tiles_count(
            (input.tiles.len() - 1, bottom_index),
            Direction::N,
            &input.tiles,
        ));
    }

    *energized_tiles_vec.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_energized_tiles_test01() {
        assert_eq!(46, get_energized_tiles("input/2023/day16_test01.txt"));
    }

    #[test]
    fn test_get_energized_tiles_test02() {
        assert_eq!(9, get_energized_tiles("input/2023/day16_test02.txt"));
    }

    #[test]
    fn test_get_energized_tiles_test03() {
        assert_eq!(18, get_energized_tiles("input/2023/day16_test03.txt"));
    }

    #[test]
    fn test_get_energized_tiles_test04() {
        assert_eq!(16, get_energized_tiles("input/2023/day16_test04.txt"));
    }

    #[test]
    fn test_get_energized_tiles_test05() {
        assert_eq!(41, get_energized_tiles("input/2023/day16_test05.txt"));
    }

    #[test]
    fn test_get_sum_steps() {
        assert_eq!(8539, get_energized_tiles("input/2023/day16.txt"));
    }

    #[test]
    fn test_get_max_energized_tiles_test01() {
        assert_eq!(51, get_max_energized_tiles("input/2023/day16_test01.txt"));
    }

    #[test]
    fn test_get_max_energized_tiles() {
        assert_eq!(8674, get_max_energized_tiles("input/2023/day16.txt"));
    }
}
