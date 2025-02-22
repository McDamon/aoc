// https://adventofcode.com/2024/day/22

use crate::utils::get_lines;

struct Input {
    init_sec_nums: Vec<usize>
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut init_sec_nums = Vec::new();

    for line in lines {
        init_sec_nums.push(line.parse::<usize>().unwrap());
    }

    Input {
        init_sec_nums
    }
}

fn calc_next_sec_num(curr_sec_num: usize) -> usize {
    // Implement the logic for calculating the next sec num
    curr_sec_num + 1 // Example implementation
}

pub fn get_sum_sec_nums(input_file: &str, level: usize) -> usize {
    let input = parse_input(input_file);

    let mut sum_sec_num = 0usize;

    for sec_num in input.init_sec_nums {
        let mut curr_sec_num = sec_num;
        for i in 0..level {
            curr_sec_num = calc_next_sec_num(curr_sec_num);
            println!("Level: {}, Sec Num: {}", i, curr_sec_num);
        }
        sum_sec_num += curr_sec_num;
    }

    sum_sec_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_sec_nums_test01() {
        assert_eq!(0, get_sum_sec_nums("input/2024/day22_test01.txt", 2000));
    }

    #[test]
    fn test_get_sum_sec_nums_test02() {
        assert_eq!(5908254, get_sum_sec_nums("input/2024/day22_test02.txt", 10));
    }

    #[test]
    fn test_get_sum_sec_nums() {
        assert_eq!(0, get_sum_sec_nums("input/2024/day22.txt", 2000));
    }
}
