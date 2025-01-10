// https://adventofcode.com/2024/day/17

use crate::utils::get_lines;

fn get_joined_vals(input_file: &str) -> &str {
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_joined_vals_test01() {
        assert_eq!("", get_joined_vals("input/2024/day17_test01.txt"));
    }

    #[test]
    fn test_get_joined_vals() {
        assert_eq!("", get_joined_vals("input/2024/day17.txt"));
    }
}
