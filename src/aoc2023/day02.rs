// https://adventofcode.com/2023/day/2

use crate::utils::get_lines;

#[derive(Debug, PartialEq)]
pub struct CubeCount {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq)]
pub struct Input {
    games: Vec<(u32, Vec<CubeCount>)>,
}

pub fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        games: parse_games(iter.next().unwrap().to_owned()),
    }
}

pub fn parse_games(games_lines: Vec<String>) -> Vec<(u32, Vec<CubeCount>)> {
    let mut games: Vec<(u32, Vec<CubeCount>)> = Vec::new();

    for game_line in games_lines.into_iter() {
        let game_split: Vec<&str> = game_line.split(':').collect();

        let game_id_part = game_split.first().unwrap();

        let game_id: u32 = game_id_part
            .split_whitespace()
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let game_result_part = game_split.last().unwrap();

        let game_result_sets: Vec<&str> = game_result_part.split(';').collect();

        let mut cube_counts: Vec<CubeCount> = Vec::new();

        for game_result_set in game_result_sets {
            let mut cube_count = CubeCount {
                red: 0,
                green: 0,
                blue: 0,
            };
            let game_result_cubes: Vec<&str> = game_result_set.split(',').collect();
            for game_result_cube in game_result_cubes {
                let game_result_cube_split: Vec<&str> =
                    game_result_cube.split_whitespace().collect();
                let game_result_cube_count: u32 =
                    game_result_cube_split.first().unwrap().parse().unwrap();
                let game_result_cube_color = game_result_cube_split.last().unwrap();
                match *game_result_cube_color {
                    "red" => cube_count.red = game_result_cube_count,
                    "green" => cube_count.green = game_result_cube_count,
                    "blue" => cube_count.blue = game_result_cube_count,
                    _ => panic!("invalid game result"),
                }
            }
            cube_counts.push(cube_count);
        }

        games.push((game_id, cube_counts));
    }

    games
}

pub fn get_sum_ids_of_valid_games(input_file: &str) -> u32 {
    let input = parse_input(input_file);
    println!("{:?}", input);
    let mut sum_ids: u32 = 0;
    for (game_id, cube_counts) in input.games {
        if possible_game(cube_counts) {
            sum_ids += game_id;
        }
    }
    sum_ids
}

pub fn possible_game(cube_counts: Vec<CubeCount>) -> bool {
    let mut result = true;
    for cube_count in cube_counts {
        if cube_count.red > 12 {
            result = false;
        }
        if cube_count.green > 13 {
            result = false;
        }
        if cube_count.blue > 14 {
            result = false;
        }
    }
    result
}

pub fn get_sum_power_sets(input_file: &str) -> u32 {
    let input = parse_input(input_file);
    let mut sum_power_sets: u32 = 0;
    for (_, cube_counts) in input.games {
        let mut reds: Vec<u32> = Vec::new();
        let mut greens: Vec<u32> = Vec::new();
        let mut blues: Vec<u32> = Vec::new();
        for cube_count in cube_counts {
            reds.push(cube_count.red);
            greens.push(cube_count.green);
            blues.push(cube_count.blue);
        }
        let max_red = reds.iter().max().unwrap();
        let max_green = greens.iter().max().unwrap();
        let max_blue = blues.iter().max().unwrap();
        let power = max_red * max_green * max_blue;
        sum_power_sets += power;
    }
    sum_power_sets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_ids_of_valid_games_test01() {
        assert_eq!(8, get_sum_ids_of_valid_games("input/2023/day02_test01.txt"));
    }

    #[test]
    fn test_sum_ids_of_valid_games() {
        assert_eq!(2369, get_sum_ids_of_valid_games("input/2023/day02.txt"));
    }

    #[test]
    fn test_sum_power_sets_test01() {
        assert_eq!(2286, get_sum_power_sets("input/2023/day02_test01.txt"));
    }

    #[test]
    fn test_sum_power_sets_values() {
        assert_eq!(66363, get_sum_power_sets("input/2023/day02.txt"));
    }
}
