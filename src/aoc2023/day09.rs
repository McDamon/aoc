// https://adventofcode.com/2023/day/9

use crate::utils::get_lines;

#[derive(Debug)]
pub struct Input {
    reports: Vec<Vec<i64>>,
}

pub fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    Input {
        reports: parse_reports(lines),
    }
}

pub fn parse_reports(lines: Vec<String>) -> Vec<Vec<i64>> {
    let mut reports = vec![];
    for line in lines {
        let report: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        reports.push(report)
    }
    reports
}

pub fn get_sum_extrapolated_values(input_file: &str) -> i64 {
    let input = parse_input(input_file);
    let mut extrapolated_values: Vec<i64> = vec![];
    for report in input.reports {
        extrapolated_values.push(extrapolate_next_value(&report));
    }
    extrapolated_values.iter().sum()
}

pub fn extrapolate_next_value(report: &[i64]) -> i64 {
    let next_report: Vec<i64> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    

    if next_report.iter().all(|x| *x == 0) {
        *report.last().unwrap()
    } else {
        let val = extrapolate_next_value(&next_report);
        *report.last().unwrap() + val
    }
}

pub fn get_sum_back_extrapolated_values(input_file: &str) -> i64 {
    let input = parse_input(input_file);
    let mut extrapolated_values: Vec<i64> = vec![];
    for report in input.reports {
        let reversed_report: Vec<i64> = report.into_iter().rev().collect();
        extrapolated_values.push(extrapolate_next_value(&reversed_report));
    }
    extrapolated_values.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_extrapolated_values_test01() {
        assert_eq!(18, get_sum_extrapolated_values("input/2023/day09_test01.txt"));
    }

    #[test]
    fn test_get_sum_extrapolated_values_test02() {
        assert_eq!(28, get_sum_extrapolated_values("input/2023/day09_test02.txt"));
    }

    #[test]
    fn test_get_sum_extrapolated_values_test03() {
        assert_eq!(68, get_sum_extrapolated_values("input/2023/day09_test03.txt"));
    }

    #[test]
    fn test_get_sum_extrapolated_values_test04() {
        assert_eq!(114, get_sum_extrapolated_values("input/2023/day09_test04.txt"));
    }

    #[test]
    fn test_get_sum_extrapolated_values_test05() {
        assert_eq!(-106, get_sum_extrapolated_values("input/2023/day09_test05.txt"));
    }

    #[test]
    fn test_get_sum_extrapolated_values_test06() {
        assert_eq!(0, get_sum_extrapolated_values("input/2023/day09_test06.txt"));
    }

    #[test]
    fn test_get_sum_extrapolated_values() {
        assert_eq!(1708206096, get_sum_extrapolated_values("input/2023/day09.txt"));
    }

    #[test]
    fn test_get_sum_back_extrapolated_values_test01() {
        assert_eq!(
            5,
            get_sum_back_extrapolated_values("input/2023/day09_test03.txt")
        );
    }

    #[test]
    fn test_get_sum_back_extrapolated_values() {
        assert_eq!(1050, get_sum_back_extrapolated_values("input/2023/day09.txt"));
    }
}
