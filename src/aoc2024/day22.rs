// https://adventofcode.com/2024/day/22

use crate::utils::get_lines;

struct Input {
    init_sec_nums: Vec<usize>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut init_sec_nums = Vec::new();

    for line in lines {
        init_sec_nums.push(line.parse::<usize>().unwrap());
    }

    Input { init_sec_nums }
}

fn get_next_sec_num(sec_num: usize) -> usize {
    // First step, sec_num * 64 and XOR (mix) with sec_num, then prune
    let mut next_sec_num = (sec_num * 64) ^ sec_num;
    next_sec_num = next_sec_num % 16777216;
    // Second step, sec_num / 32 and XOR (mix) with sec_num, then prune
    next_sec_num = (next_sec_num / 32) ^ next_sec_num;
    next_sec_num = next_sec_num % 16777216;
    // Finally, sec_num * 2014 and XOR (mix) with sec_num, then prune
    next_sec_num = (next_sec_num * 2048) ^ next_sec_num;
    next_sec_num % 16777216
}

pub fn get_sum_sec_nums(input_file: &str, level: usize) -> usize {
    let input = parse_input(input_file);

    let mut sum_sec_num = 0usize;

    for sec_num in input.init_sec_nums {
        let mut curr_sec_num = sec_num;
        for _i in 0..level {
            curr_sec_num = get_next_sec_num(curr_sec_num);
        }
        sum_sec_num += curr_sec_num;
    }

    sum_sec_num
}

fn get_all_sec_nums(sec_num: usize, level: usize) -> Vec<usize> {
    let mut sec_nums = Vec::new();
    let mut curr_sec_num = sec_num;
    for _i in 0..level {
        sec_nums.push(curr_sec_num);
        curr_sec_num = get_next_sec_num(curr_sec_num);
    }
    sec_nums
}

fn get_all_sec_nums_ones(sec_num: usize, level: usize) -> Vec<usize> {
    let mut sec_nums = Vec::new();
    let mut curr_sec_num = sec_num;
    for _i in 0..level {
        sec_nums.push(
            curr_sec_num
                .to_string()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize,
        );
        curr_sec_num = get_next_sec_num(curr_sec_num);
    }
    sec_nums
}

fn get_sum_bananas(input_file: &str, level: usize) -> usize {
    let input = parse_input(input_file);

    let mut sum_bananas = 0usize;

    for sec_num in input.init_sec_nums {
        let sec_nums = get_all_sec_nums(sec_num, level);
        for sec_num in sec_nums {
            sum_bananas += sec_num;
        }
    }

    sum_bananas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_sec_nums_test01() {
        assert_eq!(
            37327623,
            get_sum_sec_nums("input/2024/day22_test01.txt", 2000)
        );
    }

    #[test]
    fn test_get_sum_sec_nums_test02() {
        assert_eq!(5908254, get_sum_sec_nums("input/2024/day22_test02.txt", 10));
    }

    #[test]
    fn test_get_sum_sec_nums() {
        assert_eq!(15608699004, get_sum_sec_nums("input/2024/day22.txt", 2000));
    }

    #[test]
    fn test_get_all_sec_nums() {
        assert_eq!(vec![123, 15887950], get_all_sec_nums(123, 2));
    }

    #[test]
    fn test_get_all_sec_nums_ones() {
        assert_eq!(vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2], get_all_sec_nums_ones(123, 10));
    }

    #[test]
    fn test_get_sum_bananas_test01() {
        assert_eq!(6, get_sum_bananas("input/2024/day22_test02.txt", 10));
    }

    #[test]
    fn test_get_sum_bananas_test02() {
        assert_eq!(23, get_sum_bananas("input/2024/day22_test03.txt", 2000));
    }

    #[test]
    fn test_get_sum_bananas() {
        assert_eq!(0, get_sum_bananas("input/2024/day22.txt", 2000));
    }
}
