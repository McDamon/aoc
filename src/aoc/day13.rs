// https://adventofcode.com/2024/day/13

use nalgebra::{Matrix2, Vector2};
use regex::Regex;

use super::utils::get_lines;

#[derive(Clone, Copy, Debug)]
struct GamePosition {
    x_right: usize,
    y_forward: usize,
}

#[derive(Debug)]
struct Game {
    button_a: GamePosition,
    button_b: GamePosition,
    prize: GamePosition,
}

struct Input {
    games: Vec<Game>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut games: Vec<Game> = Vec::new();

    lines.split(|line| line.trim().is_empty()).for_each(|game| {
        if let [button_a, button_b, prize] = game {
            games.push(Game {
                button_a: parse_button_a(button_a),
                button_b: parse_button_b(button_b),
                prize: parse_prize(prize),
            });
        }
    });
    Input { games }
}

fn parse_button_a(button_a: &str) -> GamePosition {
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let caps = re.captures(button_a).unwrap();

    let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

    GamePosition {
        x_right: x as usize,
        y_forward: y as usize,
    }
}

fn parse_button_b(button_b: &str) -> GamePosition {
    let re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let caps = re.captures(button_b).unwrap();

    let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

    GamePosition {
        x_right: x as usize,
        y_forward: y as usize,
    }
}

fn parse_prize(prize: &str) -> GamePosition {
    let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let caps = re.captures(prize).unwrap();

    let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

    GamePosition {
        x_right: x as usize,
        y_forward: y as usize,
    }
}

fn get_fewest_tokens(input_file: &str, pos_inc: usize) -> usize {
    let input = parse_input(input_file);

    input
        .games
        .into_iter()
        .map(|game| {
            // Create a system of linear equations in the form Ax = b
            let a = Matrix2::new(
                game.button_a.x_right as f64,
                game.button_b.x_right as f64,
                game.button_a.y_forward as f64,
                game.button_b.y_forward as f64,
            );
            let b = Vector2::new(
                game.prize.x_right as f64 + pos_inc as f64,
                game.prize.y_forward as f64 + pos_inc as f64,
            );

            let mut prize = 0;
            if let Some(x) = a.lu().solve(&b) {
                // Define a small tolerance value
                let tolerance = 1e-3;
                // Check if the solutions are whole numbers within the tolerance
                if (x[0] - x[0].round()).abs() < tolerance
                    && (x[1] - x[1].round()).abs() < tolerance
                {
                    let x0 = x[0].round() as usize;
                    let x1 = x[1].round() as usize;
                    let prize_calc = x0 * 3 + x1;
                    //println!("Won prize! A: {}, B: {}, Prize: {}", x0, x1, prize_calc);
                    prize = prize_calc;
                }
            }
            prize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fewest_tokens_test01() {
        assert_eq!(480, get_fewest_tokens("input/day13_test01.txt", 0));
    }

    #[test]
    fn test_get_fewest_tokens() {
        assert_eq!(29517, get_fewest_tokens("input/day13.txt", 0));
    }

    #[test]
    fn test_get_fewest_tokens_pos_inc_test01() {
        assert_eq!(
            875318608908,
            get_fewest_tokens("input/day13_test01.txt", 10000000000000)
        );
    }

    #[test]
    fn test_get_fewest_token_pos_inc_() {
        assert_eq!(
            103570327981381,
            get_fewest_tokens("input/day13.txt", 10000000000000)
        );
    }
}
