// https://adventofcode.com/2024/day/23

use crate::utils::get_lines;

struct Input {
    conn_pairs: Vec<(String, String)>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut conn_pairs: Vec<(String, String)> = vec![];

    for line in lines {
        let parts: Vec<&str> = line.split("-").collect();
        conn_pairs.push((parts[0].to_string(), parts[1].to_string()));
    }

    Input { conn_pairs }
}

pub fn get_num_conn_start_t(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    for conn_pair in input.conn_pairs {
        println!("{:?}", conn_pair);
    }
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_conn_start_t_test01() {
        assert_eq!(0, get_num_conn_start_t("input/2024/day23_test01.txt"));
    }

    #[test]
    fn test_get_num_conn_start_t() {
        assert_eq!(0, get_num_conn_start_t("input/2024/day23.txt"));
    }
}
