// https://adventofcode.com/2023/day/4

use std::collections::HashMap;

use crate::utils::get_lines;

use regex::Regex;

#[derive(Debug, Default)]
pub struct Game {
    pub winning_nums: Vec<u32>,
    pub nums: Vec<u32>,
}

#[derive(Debug)]
pub struct Input {
    pub games: HashMap<u32, Game>,
}

pub fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    Input {
        games: parse_games(lines),
    }
}

pub fn parse_games(game_lines: Vec<String>) -> HashMap<u32, Game> {
    lazy_static! {
        static ref RE_GAME: Regex = Regex::new(r"\d+").unwrap();
    }

    let mut games: HashMap<u32, Game> = HashMap::new();

    for game_line in game_lines {
        let game_line_parts: Vec<&str> = game_line.split([':', '|']).collect();
        assert!(game_line_parts.len() == 3);

        let card_num: Vec<u32> = RE_GAME
            .find_iter(game_line_parts[0])
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();

        let winning_nums: Vec<u32> = RE_GAME
            .find_iter(game_line_parts[1])
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();

        let nums: Vec<u32> = RE_GAME
            .find_iter(game_line_parts[2])
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();

        let game = Game { winning_nums, nums };

        games.insert(*card_num.first().unwrap(), game);
    }

    games
}

pub fn get_scratchcard_score(input_file: &str) -> u32 {
    let mut sum_scratchcard_points: u32 = 0;

    let input = parse_input(input_file);

    for (_, game) in input.games.into_iter() {
        let mut game_score: u32 = 0;
        for num in game.nums {
            if game.winning_nums.contains(&num) {
                if game_score == 0 {
                    game_score = 1;
                } else {
                    game_score *= 2;
                }
            }
        }
        sum_scratchcard_points += game_score;
    }

    sum_scratchcard_points
}

pub fn get_total_scratchcards(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut total_scratchcards: u32 = input.games.len() as u32;

    total_scratchcards +=
        get_total_scratchcards_rec(input.games.keys().cloned().collect(), &input.games);

    total_scratchcards
}

fn get_total_scratchcards_rec(game_ids: Vec<u32>, all_games: &HashMap<u32, Game>) -> u32 {
    let mut sorted_game_ids = game_ids.clone();
    sorted_game_ids.sort();
    //println!("get_total_scratchcards_rec");
    let mut total_scratchcards: u32 = 0;
    for &game_id in &sorted_game_ids {
        //println!("game_id: {game_id}");
        if let Some(game) = all_games.get(&game_id) {
            let winning_nums = num_winning_nums(game);

            //println!("winning_nums: {winning_nums}");
            let won_game_ids: Vec<u32> = ((game_id + 1)..(game_id + 1 + winning_nums)).collect();
            if !won_game_ids.is_empty() {
                //println!("won_game_ids: {:?}", won_game_ids);
                total_scratchcards += won_game_ids.len() as u32;
                total_scratchcards += get_total_scratchcards_rec(won_game_ids, all_games);
            }
        }
    }
    total_scratchcards
}

fn num_winning_nums(game: &Game) -> u32 {
    let mut winning_nums: u32 = 0;
    for num in &game.nums {
        if game.winning_nums.contains(num) {
            winning_nums += 1;
        }
    }
    winning_nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_scratchcard_score_test01() {
        assert_eq!(13, get_scratchcard_score("input/2023/day04_test01.txt"));
    }

    #[test]
    fn test_get_scratchcard_score() {
        assert_eq!(20667, get_scratchcard_score("input/2023/day04.txt"));
    }

    #[test]
    fn test_get_total_scratchcards_test01() {
        assert_eq!(30, get_total_scratchcards("input/2023/day04_test01.txt"));
    }

    #[test]
    fn test_get_total_scratchcards() {
        assert_eq!(5833065, get_total_scratchcards("input/2023/day04.txt"));
    }
}
