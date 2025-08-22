// https://adventofcode.com/2023/day/1

use crate::utils::get_lines;

pub fn get_sum_calibration_values(input_file: &str, digits_can_be_text: bool) -> u32 {
    let lines = get_lines(input_file);

    let mut sum_calibration_values = 0;

    for line in lines {
        if !line.is_empty() {
            sum_calibration_values += get_calibration_value(&line, digits_can_be_text);
        }
    }

    sum_calibration_values
}

pub fn get_digit(line: &str, digits: &mut Vec<u32>, digits_can_be_text: bool) {
    if digits_can_be_text {
        if let Some(rest) = line.strip_prefix("one") {
            digits.push(1);
            get_digit(rest, digits, digits_can_be_text);
        } else if let Some(rest) = line.strip_prefix("two") {
            digits.push(2);
            get_digit(rest, digits, digits_can_be_text);
        } else if let Some(rest) = line.strip_prefix("three") {
            digits.push(3);
            get_digit(rest, digits, digits_can_be_text);
        } else if let Some(rest) = line.strip_prefix("four") {
            digits.push(4);
            get_digit(rest, digits, digits_can_be_text);
        } else if let Some(rest) = line.strip_prefix("five") {
            digits.push(5);
            get_digit(rest, digits, digits_can_be_text);
        } else if let Some(rest) = line.strip_prefix("six") {
            digits.push(6);
            get_digit(rest, digits, digits_can_be_text);
        } else if let Some(rest) = line.strip_prefix("seven") {
            digits.push(7);
            get_digit(rest, digits, digits_can_be_text);
        } else if let Some(rest) = line.strip_prefix("eight") {
            digits.push(8);
            get_digit(rest, digits, digits_can_be_text);
        } else if let Some(rest) = line.strip_prefix("nine") {
            digits.push(9);
            get_digit(rest, digits, digits_can_be_text);
        }
    }

    match line.chars().next() {
        Some(num) if num.is_ascii_digit() => {
            digits.push(num.to_digit(10).unwrap());
            get_digit(&line[1..], digits, digits_can_be_text);
        }
        Some(_) => {
            get_digit(&line[1..], digits, digits_can_be_text);
        }
        None => {}
    };
}

fn get_digits(line: &str, digits_can_be_text: bool) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    get_digit(line, &mut digits, digits_can_be_text);

    digits
}

fn get_calibration_value(line: &str, digits_can_be_text: bool) -> u32 {
    let digits = get_digits(line, digits_can_be_text);

    let first_digit = digits.first();
    let last_digit = digits.last();

    let mut calibration_value: String = String::new();

    calibration_value += &first_digit.ok_or("").unwrap().to_string();
    calibration_value += &last_digit.ok_or("").unwrap().to_string();

    calibration_value.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_calibration_values_test01() {
        assert_eq!(
            142,
            get_sum_calibration_values("input/2023/day01_test01.txt", false)
        );
    }

    #[test]
    fn test_sum_calibration_values_test02() {
        assert_eq!(
            281,
            get_sum_calibration_values("input/2023/day01_test02.txt", true)
        );
    }

    #[test]
    fn test_sum_calibration_values_part01() {
        assert_eq!(54390, get_sum_calibration_values("input/2023/day01.txt", false));
    }

    #[test]
    fn test_sum_calibration_values_part02() {
        assert_eq!(54277, get_sum_calibration_values("input/2023/day01.txt", true));
    }
}
