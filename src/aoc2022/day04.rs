// https://adventofcode.com/2022/day/4

use std::ops::Range;

use crate::utils::get_lines;

pub fn create_range(range_str: &str) -> Range<i32> {
    let split: Vec<&str> = range_str.splitn(2, "-").collect();
    let lower: i32 = split[0].parse().unwrap();
    let upper: i32 = split[1].parse().unwrap();
    let num = upper + 1;
    lower..num
}

pub fn count_contained_assignment_pairs(input_file: &str) -> i32 {
    let lines = get_lines(input_file);

    let mut count = 0;

    for line in lines {
        let split: Vec<&str> = line.splitn(2, ",").collect();
        let first_range = create_range(split[0]);
        let second_range = create_range(split[1]);

        if (first_range.start >= second_range.start && first_range.end <= second_range.end)
            || (second_range.start >= first_range.start && second_range.end <= first_range.end)
        {
            count += 1;
        }
    }

    count
}

pub fn count_overlap_assignment_pairs(input_file: &str) -> i32 {
    let lines = get_lines(input_file);

    let mut count = 0;

    for line in lines {
        let split: Vec<&str> = line.splitn(2, ",").collect();
        let first_range = create_range(split[0]);
        let second_range = create_range(split[1]);

        if (first_range.start > second_range.start && first_range.start < second_range.end)
            || (first_range.end > second_range.start && first_range.end <= second_range.end)
            || (second_range.start > first_range.start && second_range.start < first_range.end)
            || (second_range.end > first_range.start && second_range.end < first_range.end)
        {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_range() {
        assert_eq!(2..9, create_range("2-8"));
    }

    #[test]
    fn test_count_contained_assignment_pairs() {
        assert_eq!(
            453,
            count_contained_assignment_pairs("input/2022/day04.txt")
        );
    }

    #[test]
    fn test_count_overlap_assignment_pairs() {
        assert_eq!(919, count_overlap_assignment_pairs("input/2022/day04.txt"));
    }
}
