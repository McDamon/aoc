// https://adventofcode.com/2024/day/1

use super::utils::get_lines;

use std::iter::zip;

struct Input {
    left_list: Vec<u32>,
    right_list: Vec<u32>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in lines {
        let line_parts: Vec<&str> = line.split_whitespace().take(2).collect();
        if let [left, right] = &line_parts[..] {
            left_list.push(left.parse::<u32>().unwrap());
            right_list.push(right.parse::<u32>().unwrap());
        }
    }

    left_list.sort();
    right_list.sort();

    Input {
        left_list,
        right_list,
    }
}

fn get_total_distance(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut total_distance = 0;

    let iter = zip(input.left_list, input.right_list);

    for (left, right) in iter {
        total_distance += right.abs_diff(left)
    }

    total_distance
}

fn get_similarity_score(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut similarity_score = 0;

    for left in input.left_list {
        let right_count = input.right_list.iter().filter(|&i| *i == left).count();
        similarity_score += left * right_count as u32;
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_distance_test01() {
        assert_eq!(11, get_total_distance("input/day01_test01.txt"));
    }

    #[test]
    fn test_get_total_distance() {
        assert_eq!(1151792, get_total_distance("input/day01.txt"));
    }

    #[test]
    fn test_get_similarity_score_test01() {
        assert_eq!(31, get_similarity_score("input/day01_test01.txt"));
    }

    #[test]
    fn test_similarity_score() {
        assert_eq!(21790168, get_similarity_score("input/day01.txt"));
    }
}
