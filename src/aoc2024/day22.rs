// https://adventofcode.com/2024/day/22

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

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
        let sec_nums = get_all_sec_nums(sec_num, level);
        sum_sec_num += sec_nums.last().unwrap();
    }

    sum_sec_num
}

pub fn get_all_sec_nums(sec_num: usize, level: usize) -> Vec<usize> {
    let mut sec_nums = vec![];
    sec_nums.push(sec_num);
    let mut curr_sec_num = sec_num;
    for _i in 0..level {
        curr_sec_num = get_next_sec_num(curr_sec_num);
        sec_nums.push(curr_sec_num);
    }
    sec_nums
}

fn get_all_sec_nums_ones(sec_num: usize, level: usize) -> Vec<usize> {
    let sec_nums = get_all_sec_nums(sec_num, level);
    let mut sec_num_ones = vec![];
    for sec_num in sec_nums {
        sec_num_ones.push(
            sec_num
                .to_string()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize,
        );
    }
    sec_num_ones
}

fn get_banana_price_deltas(banana_prices: &[usize]) -> Vec<isize> {
    let mut banana_price_deltas = vec![];

    if banana_prices.len() < 2 {
        return banana_price_deltas;
    }

    for window in banana_prices.windows(2) {
        let curr_banana_price = window[0] as isize;
        let next_banana_price = window[1] as isize;
        banana_price_deltas.push(next_banana_price - curr_banana_price);
    }

    banana_price_deltas
}

fn get_banana_price_seqs_with_prices(
    banana_prices: &[usize],
    banana_price_deltas: &[isize],
) -> HashMap<(isize, isize, isize, isize), usize> {
    let mut banana_price_seqs_with_prices = HashMap::new();

    let dedup_banana_prices: Vec<usize> = banana_prices
        .iter()
        .cloned()
        .unique()
        .collect();

    for dedup_banana_price in dedup_banana_prices {
        for (i, banana_price) in banana_prices.iter().enumerate() {
            if *banana_price == dedup_banana_price && i >= 4 {
                banana_price_seqs_with_prices.insert(
                    (
                        banana_price_deltas[i - 4],
                        banana_price_deltas[i - 3],
                        banana_price_deltas[i - 2],
                        banana_price_deltas[i - 1],
                    ),
                    *banana_price,
                );
            }
        }
    }

    banana_price_seqs_with_prices
}

pub fn get_max_bananas(input_file: &str, level: usize) -> usize {
    let input = parse_input(input_file);

    let mut all_banana_price_seqs_by_sec_num = HashMap::new();
    let mut all_banana_price_seqs = HashSet::new();
    let mut all_banana_price_seqs_with_prices = HashMap::new();

    for sec_num in &input.init_sec_nums {
        let banana_prices = get_all_sec_nums_ones(*sec_num, level);
        let banana_price_deltas = get_banana_price_deltas(&banana_prices);
        let banana_price_seqs_with_prices =
            get_banana_price_seqs_with_prices(&banana_prices, &banana_price_deltas);
        let banana_price_seqs: Vec<(isize, isize, isize, isize)> =
            banana_price_seqs_with_prices.keys().cloned().collect();
        all_banana_price_seqs_by_sec_num.insert(*sec_num, banana_price_seqs.clone());
        all_banana_price_seqs.extend(banana_price_seqs);
        all_banana_price_seqs_with_prices.insert(
            *sec_num,
            banana_price_seqs_with_prices.clone(),
        );
    }

    let mut all_banana_price_seqs_with_max_bananas = HashMap::new();

    for banana_price_seq in &all_banana_price_seqs {
        let mut max_bananas = 0;
        for sec_num in &input.init_sec_nums {
            if all_banana_price_seqs_by_sec_num[&sec_num].contains(&banana_price_seq) {
                if let Some(all_banana_prices_seqs_for_sec_num) = all_banana_price_seqs_with_prices.get(&sec_num) {
                    if let Some(banana_price_for_seq_num) = all_banana_prices_seqs_for_sec_num.get(banana_price_seq) {
                        max_bananas += banana_price_for_seq_num;
                    }
                }
            }
        }
        all_banana_price_seqs_with_max_bananas.insert(banana_price_seq, max_bananas);
    }

    let max_bananas = all_banana_price_seqs_with_max_bananas
        .values()
        .max()
        .unwrap()
        .clone();

    max_bananas
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
    fn test_get_sum_sec_nums_test03() {
        assert_eq!(18183557, get_sum_sec_nums("input/2024/day22_test04.txt", 2000));
    }

    #[test]
    fn test_get_sum_sec_nums_test04() {
        assert_eq!(8876699, get_sum_sec_nums("input/2024/day22_test05.txt", 2000));
    }

    #[test]
    fn test_get_sum_sec_nums() {
        assert_eq!(15608699004, get_sum_sec_nums("input/2024/day22.txt", 2000));
    }

    #[test]
    fn test_get_all_sec_nums_test01() {
        assert_eq!(vec![123, 15887950, 16495136], get_all_sec_nums(123, 2));
    }

    #[test]
    fn test_get_all_sec_nums_test02() {
        assert_eq!(
            vec![
                123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
                7753432, 5908254
            ],
            get_all_sec_nums(123, 10)
        );
    }

    #[test]
    fn test_get_all_sec_nums_test03() {
        let all_sec_nums = get_all_sec_nums(1, 2000);
        assert_eq!(8685429, all_sec_nums.last().unwrap().clone());
    }

    #[test]
    fn test_get_all_sec_nums_ones() {
        assert_eq!(
            vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2],
            get_all_sec_nums_ones(123, 9)
        );
    }

    #[test]
    fn test_get_max_bananas_test01() {
        assert_eq!(6, get_max_bananas("input/2024/day22_test02.txt", 9));
    }

    #[test]
    fn test_get_max_bananas_test02() {
        assert_eq!(23, get_max_bananas("input/2024/day22_test03.txt", 2000));
    }

    #[test]
    fn test_get_max_bananas_test03() {
        assert_eq!(27, get_max_bananas("input/2024/day22_test04.txt", 2000));
    }

    #[test]
    fn test_get_max_bananas_test04() {
        assert_eq!(27, get_max_bananas("input/2024/day22_test05.txt", 2000));
    }

    #[test]
    fn test_get_max_bananas_test05() {
        assert_eq!(12, get_max_bananas("input/2024/day22_test06.txt", 9));
    }
    
    #[test]
    fn test_get_max_bananas() {
        assert_eq!(0, get_max_bananas("input/2024/day22.txt", 2000));
    }
}
