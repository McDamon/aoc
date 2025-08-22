// https://adventofcode.com/2022/day/1

use crate::utils::get_lines;

pub fn get_highest_elf_calories(input_file: &str) -> i32 {
    let lines = get_lines(input_file);

    let mut calorie_counter = Vec::new();

    let mut max_calories = 0;

    for line in lines {
        if line.is_empty() {
            let calories = calorie_counter.iter().sum();
            if calories > max_calories {
                max_calories = calories;
            }
            calorie_counter.clear();
        } else {
            calorie_counter.push(line.parse::<i32>().unwrap_or_default());
        }
    }

    max_calories
}

pub fn get_sum_top_three_elf_calories(input_file: &str) -> i32 {
    let lines = get_lines(input_file);

    let mut calorie_counter = Vec::new();

    let mut calories_per_elf = Vec::new();

    for line in lines {
        if line.is_empty() {
            let calories: i32 = calorie_counter.iter().sum();
            calories_per_elf.push(calories);
            calorie_counter.clear();
        } else {
            calorie_counter.push(line.parse::<i32>().unwrap_or_default());
        }
    }

    // Flip the comparison operator here
    calories_per_elf.sort_by(|a, b| b.cmp(a));

    // Return the sum of the top 3
    calories_per_elf[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_highest_elf_calories() {
        assert_eq!(69206, get_highest_elf_calories("input/2022/day01.txt"));
    }

    #[test]
    fn test_get_sum_top_three_elf_calories() {
        assert_eq!(
            197400,
            get_sum_top_three_elf_calories("input/2022/day01.txt")
        );
    }
}
