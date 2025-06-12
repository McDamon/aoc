// https://adventofcode.com/2019/day/4

use crate::utils::get_lines;
use itertools::Itertools;

struct Input {
    range: (usize, usize),
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);
    let mut parts = lines[0].split('-');
    let start = parts.next().unwrap().parse::<usize>().unwrap();
    let end = parts.next().unwrap().parse::<usize>().unwrap();
    Input {
        range: (start, end),
    }
}

pub fn is_valid_password(password: &[usize]) -> bool {
    if password.len() != 6 {
        return false;
    }

    let mut is_adj_equal = false;
    for (prev, next) in password.iter().tuple_windows() {
        if prev == next {
            is_adj_equal = true;
            break;
        }
    }

    let mut is_any_dec = false;
    for (prev, next) in password.iter().tuple_windows() {
        if next < prev {
            is_any_dec = true;
            break;
        }
    }

    is_adj_equal && !is_any_dec
}

pub fn is_valid_password_range(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let mut num_valid_passwords = 0usize;

    for num in input.range.0..=input.range.1 {
        let password: Vec<usize> = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        if is_valid_password(&password) {
            num_valid_passwords += 1;
        }
    }

    num_valid_passwords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_password_first() {
        assert_eq!(true, is_valid_password(&[1, 1, 1, 1, 1, 1,]));
    }

    #[test]
    fn test_is_valid_password_second() {
        assert_eq!(false, is_valid_password(&[2, 2, 3, 4, 5, 0]));
    }

    #[test]
    fn test_is_valid_password_third() {
        assert_eq!(false, is_valid_password(&[1, 2, 3, 7, 8, 9]));
    }

    #[test]
    fn test_is_valid_password_fourth() {
        assert_eq!(false, is_valid_password(&[5, 8, 4, 7, 0, 0]));
    }

    #[test]
    fn test_is_valid_password_range() {
        assert_eq!(1929, is_valid_password_range("input/2019/day04.txt"));
    }
}
