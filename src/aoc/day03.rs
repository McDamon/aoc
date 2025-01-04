// https://adventofcode.com/2024/day/3

use regex::Regex;
use std::fs;

fn get_sum_mul(input_file: &str) -> u32 {
    lazy_static! {
        static ref RE_MUL: Regex =
            Regex::new(r"(?P<operation>mul)\((?P<left>\d*),(?P<right>\d*)\)").unwrap();
    }
    let line: String = fs::read_to_string(input_file).unwrap();

    let mut sum_mul = 0;

    for cap_mul in RE_MUL.captures_iter(&line) {
        let _ = cap_mul["operation"].to_string();
        let left_str = cap_mul["left"].to_string();
        let right_str = cap_mul["right"].to_string();

        sum_mul += left_str.parse::<u32>().unwrap() * right_str.parse::<u32>().unwrap();
    }

    sum_mul
}

fn get_sum_mul_cond(input_file: &str) -> u32 {
    lazy_static! {
        static ref RE_MUL_COND: Regex =
            Regex::new(r"(?P<operation>mul\((?P<left>\d*),(?P<right>\d*)\))|(?P<enable>do\(\))|(?P<disable>don't\(\))").unwrap();
    }
    let line: String = fs::read_to_string(input_file).unwrap();

    let mut sum_mul = 0;

    // We start out enabled
    let mut is_enabled = true;

    for caps in RE_MUL_COND.captures_iter(&line) {
        if caps.name("enable").is_some() {
            is_enabled = true;
        } else if caps.name("disable").is_some() {
            is_enabled = false;
        } else if is_enabled {
            let _ = caps["operation"].to_string();
            let left_str = caps["left"].to_string();
            let right_str = caps["right"].to_string();

            sum_mul += left_str.parse::<u32>().unwrap() * right_str.parse::<u32>().unwrap();
        }
    }

    sum_mul
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_mul_test01() {
        assert_eq!(161, get_sum_mul("input/day03_test01.txt"));
    }

    #[test]
    fn test_get_sum_mul() {
        assert_eq!(180233229, get_sum_mul("input/day03.txt"));
    }

    #[test]
    fn test_get_sum_mul_cond_test01() {
        assert_eq!(48, get_sum_mul_cond("input/day03_test02.txt"));
    }

    #[test]
    fn test_get_sum_mul_cond() {
        assert_eq!(95411583, get_sum_mul_cond("input/day03.txt"));
    }
}
