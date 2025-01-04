// https://adventofcode.com/2024/day/2

use super::utils::get_lines;

struct Input {
    reports: Vec<Vec<i32>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|level| level.parse::<i32>().unwrap())
            .collect();
        reports.push(report);
    }

    Input { reports }
}

fn is_all_increasing(arr: &[i32]) -> bool {
    arr.iter().all(|elem| *elem > 0)
}

fn is_all_decreasing(arr: &[i32]) -> bool {
    arr.iter().all(|elem| *elem < 0)
}

fn is_all_adj_diff_in_limits(arr: &[i32]) -> bool {
    arr.iter().all(|elem| elem.abs() >= 1 && elem.abs() <= 3)
}

fn get_num_safe_reports(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut num_safe_reports = 0;

    for report in input.reports {
        let gradients: Vec<i32> = report
            .windows(2)
            .map(|elems| {
                if let [left, right] = elems {
                    *right - *left
                } else {
                    panic!()
                }
            })
            .collect();
        if (is_all_increasing(&gradients) || is_all_decreasing(&gradients))
            && is_all_adj_diff_in_limits(&gradients)
        {
            num_safe_reports += 1;
        }
    }

    num_safe_reports
}

fn get_num_safe_reports_with_prob_damp(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut num_safe_reports = 0;

    for report in input.reports {
        for n in 0..report.len() {
            let filtered_report: Vec<i32> = report
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != n)
                .map(|(_, n)| n)
                .cloned()
                .collect();
            let gradients: Vec<i32> = filtered_report
                .windows(2)
                .map(|elems| {
                    if let [left, right] = elems {
                        *right - *left
                    } else {
                        panic!()
                    }
                })
                .collect();
            if (is_all_increasing(&gradients) || is_all_decreasing(&gradients))
                && is_all_adj_diff_in_limits(&gradients)
            {
                num_safe_reports += 1;
                break;
            }
        }
    }

    num_safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_safe_reports_test01() {
        assert_eq!(2, get_num_safe_reports("input/day02_test01.txt"));
    }

    #[test]
    fn test_get_num_safe_reports() {
        assert_eq!(686, get_num_safe_reports("input/day02.txt"));
    }

    #[test]
    fn test_get_num_safe_reports_with_prob_damp_test01() {
        assert_eq!(
            4,
            get_num_safe_reports_with_prob_damp("input/day02_test01.txt")
        );
    }

    #[test]
    fn test_get_num_safe_reports_with_prob_damp() {
        assert_eq!(717, get_num_safe_reports_with_prob_damp("input/day02.txt"));
    }
}
