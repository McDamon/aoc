// https://adventofcode.com/2024/day/5

use super::utils::get_lines;

struct Input {
    page_order_rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut page_order_rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let page_order_rule_parts: Vec<&str> = line.split(['|']).take(2).collect();
        if let [left, right] = &page_order_rule_parts[..] {
            page_order_rules.push((left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()));
        }

        let update_parts: Vec<&str> = line.split([',']).collect();
        if update_parts.len() > 1 {
            updates.push(
                update_parts
                    .iter()
                    .map(|part| part.parse::<u32>().unwrap())
                    .collect(),
            );
        }
    }

    Input {
        page_order_rules,
        updates,
    }
}

fn check_update_order(update: &[u32], page_order_rules: &Vec<(u32, u32)>) -> bool {
    for i in 0..update.len() {
        for j in i..update.len() {
            if i != j {
                let left = update[i];
                let right = update[j];
                for (left_rule, right_rule) in page_order_rules {
                    if left == *right_rule && right == *left_rule {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn get_sum_middle_page_num(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut sum_middle_page_num = 0;

    for update in input.updates {
        if check_update_order(&update, &input.page_order_rules) {
            let middle = update.len() / 2;
            sum_middle_page_num += update[middle];
        }
    }

    sum_middle_page_num
}

fn correct_update(update: &mut [u32], page_order_rules: &Vec<(u32, u32)>) {
    for i in 0..update.len() {
        for j in i..update.len() {
            if i != j {
                let left = update[i];
                let right = update[j];
                for (left_rule, right_rule) in page_order_rules {
                    if left == *right_rule && right == *left_rule {
                        update.swap(j, i);
                    }
                }
            }
        }
    }
}

fn get_sum_corrected_middle_page_num(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut sum_middle_page_num = 0;

    for update in input.updates {
        if !check_update_order(&update, &input.page_order_rules) {
            loop {
                let mut mut_update: Vec<u32> = update.to_vec();
                correct_update(&mut mut_update, &input.page_order_rules);
                if check_update_order(&mut_update, &input.page_order_rules) {
                    let middle = mut_update.len() / 2;
                    sum_middle_page_num += mut_update[middle];
                    break;
                }
            }
        }
    }

    sum_middle_page_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_middle_page_num_test01() {
        assert_eq!(143, get_sum_middle_page_num("input/day05_test01.txt"));
    }

    #[test]
    fn test_get_sum_middle_page_num() {
        assert_eq!(4996, get_sum_middle_page_num("input/day05.txt"));
    }

    #[test]
    fn test_get_sum_corrected_middle_page_num_test01() {
        assert_eq!(
            123,
            get_sum_corrected_middle_page_num("input/day05_test01.txt")
        );
    }

    #[test]
    fn test_get_sum_corrected_middle_page_num() {
        assert_eq!(6311, get_sum_corrected_middle_page_num("input/day05.txt"));
    }
}
