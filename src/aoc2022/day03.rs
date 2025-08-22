// https://adventofcode.com/2022/day/3/input

use std::collections::{HashMap, HashSet};

use crate::utils::get_lines;

pub fn get_priority_for_char(c: char) -> i32 {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let pos: i32 = alphabet
        .iter()
        .position(|&r| r == c)
        .unwrap()
        .try_into()
        .unwrap();
    pos + 1
}

pub fn get_sum_priorities_of_item_types(input_file: &str) -> i32 {
    let lines = get_lines(input_file);

    let mut sum_priorities = 0;

    for line in lines {
        let (first_comp, second_comp) = line.split_at(line.len() / 2);

        // Count the chars in the second comp string
        let second_comp_counts: HashMap<char, i32> =
            second_comp.chars().fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });

        // Iterate through the first string, checking for a match for counted chars in the second
        for c in first_comp.chars() {
            if second_comp_counts.contains_key(&c) {
                sum_priorities += get_priority_for_char(c);
                break;
            }
        }
    }

    sum_priorities
}

pub fn get_sum_of_priorities_of_item_types_part2(input_file: &str) -> i32 {
    let lines = get_lines(input_file);

    let mut sum_priorities = 0;

    for group in lines.chunks(3) {
        // Create a group of hash sets
        let group_sets = group
            .into_iter()
            .map(|line| HashSet::from_iter(line.chars()))
            .collect::<Vec<HashSet<char>>>();

        // Get the intersection of all sets
        let intersect = group_sets
            .into_iter()
            .reduce(|a, b| a.intersection(&b).cloned().collect())
            .unwrap();

        sum_priorities += get_priority_for_char(intersect.iter().next().unwrap().clone());
    }

    sum_priorities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_priority_for_char_16() {
        assert_eq!(16, get_priority_for_char('p'));
    }

    #[test]
    fn get_priority_for_char_38() {
        assert_eq!(38, get_priority_for_char('L'));
    }

    #[test]
    fn get_priority_for_char_42() {
        assert_eq!(42, get_priority_for_char('P'));
    }

    #[test]
    fn get_priority_for_char_22() {
        assert_eq!(22, get_priority_for_char('v'));
    }

    #[test]
    fn get_priority_for_char_20() {
        assert_eq!(20, get_priority_for_char('t'));
    }

    #[test]
    fn get_priority_for_char_19() {
        assert_eq!(19, get_priority_for_char('s'));
    }

    #[test]
    fn test_get_sum_priorities_of_item_types() {
        assert_eq!(7568, get_sum_priorities_of_item_types("input/2022/day03.txt"));
    }

    #[test]
    fn test_get_sum_of_priorities_of_item_types_part2() {
        assert_eq!(
            2780,
            get_sum_of_priorities_of_item_types_part2("input/2022/day03.txt")
        );
    }
}
