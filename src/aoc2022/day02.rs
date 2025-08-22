// https://adventofcode.com/2022/day/2

use std::collections::HashMap;

use crate::utils::get_lines;

lazy_static! {
    static ref MY_PLAY_SCORE_TABLE: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("X", 1);
        m.insert("Y", 2);
        m.insert("Z", 3);
        m
    };

    static ref OPP_PLAY_SCORE_TABLE: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("AX", 3); // opp plays Rock, my play is Rock
        m.insert("AY", 6); // opp plays Rock, my play is Paper
        m.insert("AZ", 0); // opp plays Rock, my play is Scissors
        m.insert("BX", 0); // opp plays Paper, my play is Rock
        m.insert("BY", 3); // opp plays Paper, my play is Paper
        m.insert("BZ", 6); // opp plays Paper, my play is Scissors
        m.insert("CX", 6); // opp plays Scissors, my play is Rock
        m.insert("CY", 0); // opp plays Scissors, my play is Paper
        m.insert("CZ", 3); // opp plays Scissors, my play is Scissors
        m
    };
}

pub fn get_total_rps_score(input_file: &str) -> i32 {
    let lines = get_lines(input_file);

    let mut total_score = 0;

    for line in lines {
        let mut rps_strategy = line.split_whitespace();

        let opp_play = rps_strategy.next().unwrap();

        let my_play = rps_strategy.next().unwrap();

        let my_play_score = MY_PLAY_SCORE_TABLE.get(my_play).unwrap();

        let game_check = format!("{}{}", opp_play, my_play);

        let opp_play_score = OPP_PLAY_SCORE_TABLE.get(game_check.as_str()).unwrap();

        total_score += my_play_score + opp_play_score;
    }

    total_score
}

pub fn get_total_rps_score_part_two(input_file: &str) -> i32 {
    enum RpsResult {
        Lose,
        Draw,
        Win,
    }

    let my_play_result_table = HashMap::from([
        ("X", RpsResult::Lose),
        ("Y", RpsResult::Draw),
        ("Z", RpsResult::Win),
    ]);

    let lines = get_lines(input_file);

    let mut total_score = 0;

    for line in lines {
        let mut rps_strategy = line.split_whitespace();

        let opp_play = rps_strategy.next().unwrap();

        let my_play_result = my_play_result_table
            .get(rps_strategy.next().unwrap())
            .unwrap();

        let my_play = match (opp_play, my_play_result) {
            ("A", RpsResult::Lose) => "Z", // Opp plays Rock, my play should be Scissors
            ("A", RpsResult::Draw) => "X", // Opp plays Rock, my play should be Rock
            ("A", RpsResult::Win) => "Y",  // Opp plays Rock, my play should be Paper
            ("B", RpsResult::Lose) => "X", // Opp plays Paper, my play should be Rock
            ("B", RpsResult::Draw) => "Y", // Opp plays Paper, my play should be Paper
            ("B", RpsResult::Win) => "Z",  // Opp plays Paper, my play should be Scissors
            ("C", RpsResult::Lose) => "Y", // Opp plays Scissors, my play should be Paper
            ("C", RpsResult::Draw) => "Z", // Opp plays Scissors, my play should be Scissors
            ("C", RpsResult::Win) => "X",  // Opp plays Scissors, my play should be Rock
            (&_, _) => "",
        };

        let my_play_score = MY_PLAY_SCORE_TABLE.get(my_play).unwrap();

        let game_check = format!("{}{}", opp_play, my_play);

        let opp_play_score = OPP_PLAY_SCORE_TABLE.get(game_check.as_str()).unwrap();

        total_score += my_play_score + opp_play_score;
    }

    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_rps_score() {
        assert_eq!(15337, get_total_rps_score("input/2022/day02.txt"));
    }

    #[test]
    fn test_get_total_rps_score_part_two() {
        assert_eq!(11696, get_total_rps_score_part_two("input/2022/day02.txt"));
    }
}
