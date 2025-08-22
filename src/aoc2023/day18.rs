// https://adventofcode.com/2023/day/16

use std::cmp;

use colorsys::Rgb;
use grid::Grid;

use crate::utils::get_lines;

#[derive(Debug)]
struct Input {
    dig_plan: Vec<DigStep>,
}

#[derive(Debug)]
struct DigStep {
    dir: Direction,
    steps: usize,
    color: Rgb,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)]
enum Direction {
    Up = b'U',
    Down = b'D',
    Left = b'L',
    Right = b'R',
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Direction::Up as u8 => Ok(Direction::Up),
            x if x == Direction::Down as u8 => Ok(Direction::Down),
            x if x == Direction::Left as u8 => Ok(Direction::Left),
            x if x == Direction::Right as u8 => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Tile {
    dig_level: usize,
    color: Option<Rgb>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        dig_plan: parse_dig_plan(iter.next().unwrap().to_owned()),
    }
}

fn parse_dig_plan(dig_plan_lines: Vec<String>) -> Vec<DigStep> {
    let mut dig_plan: Vec<DigStep> = vec![];
    for dig_plan_line in dig_plan_lines.into_iter() {
        let dig_plan_parts: Vec<&str> = dig_plan_line.split(" ").collect();
        dig_plan.push(DigStep {
            dir: match Direction::try_from(dig_plan_parts[0].chars().next().unwrap() as u8) {
                Ok(dir) => dir,
                Err(_) => panic!("Invalid dir"),
            },
            steps: dig_plan_parts[1].trim().parse().unwrap(),
            color: Rgb::from_hex_str(
                dig_plan_parts[2]
                    .trim()
                    .replace(&['(', ')'][..], "")
                    .as_str(),
            )
            .unwrap(),
        });
    }
    dig_plan
}

fn print_trench(trench: &Grid<Tile>) {
    println!("TRENCH:");
    for tile_row in trench.iter_rows() {
        for tile in tile_row {
            if tile.dig_level == 0 {
                print!(".");
            } else if tile.dig_level == 1 {
                print!("#");
            } else {
                print!("{:#}", tile.dig_level);
            }
        }
        println!();
    }
}

fn get_cubic_meters_lava(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let (mut row, mut col): (i64, i64) = (0, 0);
    let (mut max_row, mut max_col): (i64, i64) = (0, 0);

    for dig_step in &input.dig_plan {
        for _step in 0..dig_step.steps {
            match dig_step.dir {
                Direction::Up => {
                    row -= 1;
                    if row < 0 {
                        max_row += 1;
                    }
                }
                Direction::Down => {
                    row += 1;
                    if row > max_row {
                        max_row += 1;
                    }
                }
                Direction::Left => {
                    col -= 1;
                    if col < 0 {
                        max_col += 1;
                    }
                }
                Direction::Right => {
                    col += 1;
                    if col > max_col {
                        max_col += 1;
                    }
                }
            }
        }
    }

    let mut trench: Grid<Tile> = Grid::new(max_row as usize + 1, max_col as usize + 1);

    let (mut row, mut col): (i64, i64) = (0, 0);

    for dig_step in input.dig_plan {
        for _step in 0..dig_step.steps {
            match dig_step.dir {
                Direction::Up => {
                    row -= 1;
                }
                Direction::Down => {
                    row += 1;
                }
                Direction::Left => {
                    col -= 1;
                }
                Direction::Right => {
                    col += 1;
                }
            }

            if let Some(ground) = trench.get_mut(row, col) {
                ground.dig_level = 1;
                ground.color = Some(dig_step.color.clone());
            }
        }
    }

    let mut cubit_meters_lava = 0;

    for ((row, col), tile) in trench.indexed_iter() {
        if tile.dig_level == 0 {
            let mut trench_copy = trench.clone();
            let is_inside = flood_fill((row as i64, col as i64), &mut trench_copy);
            if is_inside {
                cubit_meters_lava = trench_copy.iter().filter(|&x| x.dig_level == 1).count();
                break;
            }
        }
    }

    print_trench(&trench);

    let cubit_meters_all_dug = trench.iter().filter(|&x| x.dig_level == 1).count();

    cmp::max(cubit_meters_lava, cubit_meters_all_dug)
}

fn flood_fill((row, col): (i64, i64), trench: &mut Grid<Tile>) -> bool {
    if let Some(ground) = trench.get_mut(row, col) {
        if ground.dig_level != 1 {
            ground.dig_level = 1;
        } else {
            return true;
        }
    } else {
        return false;
    }
    flood_fill((row + 1, col), trench);
    flood_fill((row - 1, col), trench);
    flood_fill((row, col - 1), trench);
    flood_fill((row, col + 1), trench);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cubic_meters_lava_test01() {
        assert_eq!(62, get_cubic_meters_lava("input/2023/day18_test01.txt"));
    }

    #[test]
    fn test_get_cubic_meters_lava_test02() {
        assert_eq!(4, get_cubic_meters_lava("input/2023/day18_test02.txt"));
    }

    #[test]
    fn test_get_cubic_meters_lava_test03() {
        assert_eq!(9, get_cubic_meters_lava("input/2023/day18_test03.txt"));
    }

    #[test]
    fn test_get_cubic_meters_lava_test04() {
        assert_eq!(16, get_cubic_meters_lava("input/2023/day18_test04.txt"));
    }

    #[test]
    fn test_get_cubic_meters_lava_test05() {
        assert_eq!(108, get_cubic_meters_lava("input/2023/day18_test05.txt"));
    }

    #[test]
    fn test_get_cubic_meters_lava() {
        assert_eq!(9716, get_cubic_meters_lava("input/2023/day18.txt"));
    }
}
