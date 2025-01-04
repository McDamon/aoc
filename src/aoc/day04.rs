// https://adventofcode.com/2024/day/4

use super::utils::get_lines;
use super::utils::Compass;

struct Input {
    search_grid: Vec<Vec<char>>,
    row_size: usize,
    col_size: usize,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut search_grid: Vec<Vec<char>> = Vec::new();

    let mut row_size: usize = 0;

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        row_size = row.len();
        search_grid.push(row);
    }

    let col_size = search_grid.len();

    Input {
        search_grid,
        row_size,
        col_size,
    }
}

fn print_grid(search_grid: &Vec<Vec<char>>) {
    for row in search_grid {
        let _: String = row.iter().collect();
    }
}

fn min_row_index(row_index: usize) -> usize {
    if row_index as i32 - 1 < 0 {
        0
    } else {
        row_index - 1
    }
}

fn min_col_index(col_index: usize) -> usize {
    if col_index as i32 - 1 < 0 {
        0
    } else {
        col_index - 1
    }
}

fn max_row_index(row_index: usize, row_size: usize) -> usize {
    if row_index as i32 + 1 > row_size as i32 - 1 {
        row_size - 1
    } else {
        row_index + 1
    }
}

fn max_col_index(col_index: usize, col_size: usize) -> usize {
    if col_index as i32 + 1 > col_size as i32 - 1 {
        col_size - 1
    } else {
        col_index + 1
    }
}

fn check_next_in_xmas_seq(
    exp_letter: char,
    dir: Compass,
    row_index: usize,
    col_index: usize,
    input: &Input,
) -> Option<(usize, usize)> {
    let min_row_index = min_row_index(row_index);
    let min_col_index = min_col_index(col_index);
    let max_row_index = max_row_index(row_index, input.row_size);
    let max_col_index = max_col_index(col_index, input.col_size);

    //   0 1 2 3 4 5 6 7 8 9
    // 0 M M M S X X M A S M
    // 1 M S A M X M S M S A
    // 2 A M X S X M A A M M
    // 3 M S A M A S M S M X
    // 4 X M A S A M X A M M
    // 5 X X A M M X X A M A
    // 6 S M S M S A S X S S
    // 7 S A X A M A S A A A
    // 8 M A M M M X M M M M
    // 9 M X M X A X M A S X

    match dir {
        Compass::North => {
            if exp_letter == input.search_grid[min_row_index][col_index]
                && row_index != min_row_index
            {
                return Some((min_row_index, col_index));
            }
        }
        Compass::NorthEast => {
            if exp_letter == input.search_grid[min_row_index][max_col_index]
                && row_index != min_row_index
                && col_index != max_col_index
            {
                return Some((min_row_index, max_col_index));
            }
        }
        Compass::East => {
            if exp_letter == input.search_grid[row_index][max_col_index]
                && col_index != max_col_index
            {
                return Some((row_index, max_col_index));
            }
        }
        Compass::SouthEast => {
            if exp_letter == input.search_grid[max_row_index][max_col_index]
                && row_index != max_row_index
                && col_index != max_col_index
            {
                return Some((max_row_index, max_col_index));
            }
        }
        Compass::South => {
            if exp_letter == input.search_grid[max_row_index][col_index]
                && row_index != max_row_index
            {
                return Some((max_row_index, col_index));
            }
        }
        Compass::SouthWest => {
            if exp_letter == input.search_grid[max_row_index][min_col_index]
                && row_index != max_row_index
                && col_index != min_col_index
            {
                return Some((max_row_index, min_col_index));
            }
        }
        Compass::West => {
            if exp_letter == input.search_grid[row_index][min_col_index]
                && col_index != min_col_index
            {
                return Some((row_index, min_col_index));
            }
        }
        Compass::NorthWest => {
            if exp_letter == input.search_grid[min_row_index][min_col_index]
                && row_index != min_row_index
                && col_index != min_col_index
            {
                return Some((min_row_index, min_col_index));
            }
        }
    }
    None
}

fn trace_xmas(
    last_letter: char,
    dir: Compass,
    row_index: usize,
    col_index: usize,
    input: &Input,
) -> u32 {
    match last_letter {
        'X' => match check_next_in_xmas_seq('M', dir, row_index, col_index, input) {
            Some((new_row_index, new_col_index)) => {
                trace_xmas('M', dir, new_row_index, new_col_index, input)
            }
            None => 0,
        },
        'M' => match check_next_in_xmas_seq('A', dir, row_index, col_index, input) {
            Some((new_row_index, new_col_index)) => {
                trace_xmas('A', dir, new_row_index, new_col_index, input)
            }
            None => 0,
        },
        'A' => match check_next_in_xmas_seq('S', dir, row_index, col_index, input) {
            Some((_, _)) => 1,
            None => 0,
        },
        _ => 0,
    }
}

fn get_sum_xmas(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut sum_xmas = 0;

    for i in 0..input.row_size {
        for j in 0..input.col_size {
            if input.search_grid[i][j] == 'X' {
                sum_xmas += trace_xmas('X', Compass::North, i, j, &input);
                sum_xmas += trace_xmas('X', Compass::NorthEast, i, j, &input);
                sum_xmas += trace_xmas('X', Compass::East, i, j, &input);
                sum_xmas += trace_xmas('X', Compass::SouthEast, i, j, &input);
                sum_xmas += trace_xmas('X', Compass::South, i, j, &input);
                sum_xmas += trace_xmas('X', Compass::SouthWest, i, j, &input);
                sum_xmas += trace_xmas('X', Compass::West, i, j, &input);
                sum_xmas += trace_xmas('X', Compass::NorthWest, i, j, &input);
            }
        }
    }

    sum_xmas
}

fn check_next_in_x_mas_seq(
    dir: Compass,
    row_index: usize,
    col_index: usize,
    input: &Input,
) -> Option<char> {
    let min_row_index = min_row_index(row_index);
    let min_col_index = min_col_index(col_index);
    let max_row_index = max_row_index(row_index, input.row_size);
    let max_col_index = max_col_index(col_index, input.col_size);

    match dir {
        Compass::NorthEast => {
            if (input.search_grid[min_row_index][max_col_index] == 'S'
                || input.search_grid[min_row_index][max_col_index] == 'M')
                && row_index != min_row_index
                && col_index != max_col_index
            {
                return Some(input.search_grid[min_row_index][max_col_index]);
            }
        }
        Compass::SouthEast => {
            if (input.search_grid[max_row_index][max_col_index] == 'S'
                || input.search_grid[max_row_index][max_col_index] == 'M')
                && row_index != max_row_index
                && col_index != max_col_index
            {
                return Some(input.search_grid[max_row_index][max_col_index]);
            }
        }
        Compass::SouthWest => {
            if (input.search_grid[max_row_index][min_col_index] == 'S'
                || input.search_grid[max_row_index][min_col_index] == 'M')
                && row_index != max_row_index
                && col_index != min_col_index
            {
                return Some(input.search_grid[max_row_index][min_col_index]);
            }
        }
        Compass::NorthWest => {
            if (input.search_grid[min_row_index][min_col_index] == 'S'
                || input.search_grid[min_row_index][min_col_index] == 'M')
                && row_index != min_row_index
                && col_index != min_col_index
            {
                return Some(input.search_grid[min_row_index][min_col_index]);
            }
        }
        _ => (),
    }
    None
}

fn get_sum_x_mas(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut sum_x_mas = 0;

    for i in 0..input.row_size {
        for j in 0..input.col_size {
            if input.search_grid[i][j] == 'A' {
                let nw = check_next_in_x_mas_seq(Compass::NorthWest, i, j, &input);
                let ne = check_next_in_x_mas_seq(Compass::NorthEast, i, j, &input);
                let sw = check_next_in_x_mas_seq(Compass::SouthWest, i, j, &input);
                let se = check_next_in_x_mas_seq(Compass::SouthEast, i, j, &input);

                let mut fwd_str: Vec<char> = vec![nw.unwrap_or('.'), 'A', se.unwrap_or('.')];
                fwd_str.sort();

                let mut back_str: Vec<char> = vec![sw.unwrap_or('.'), 'A', ne.unwrap_or('.')];
                back_str.sort();

                if String::from_iter(fwd_str) == "AMS" && String::from_iter(back_str) == "AMS" {
                    sum_x_mas += 1;
                }
            }
        }
    }

    sum_x_mas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_xmas_test01() {
        assert_eq!(18, get_sum_xmas("input/day04_test01.txt"));
    }

    #[test]
    fn test_get_sum_xmas() {
        assert_eq!(2336, get_sum_xmas("input/day04.txt"));
    }

    #[test]
    fn test_get_sum_x_mas_test01() {
        assert_eq!(9, get_sum_x_mas("input/day04_test01.txt"));
    }

    #[test]
    fn test_get_sum_x_mas() {
        assert_eq!(1831, get_sum_x_mas("input/day04.txt"));
    }
}
