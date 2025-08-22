// https://adventofcode.com/2022/day/8

use crate::utils::get_lines;

pub fn parse_input(input_file: &str) -> Vec<u32> {
    let lines = get_lines(input_file);

    let rows = lines.len();
    let cols = lines.first().unwrap().len();

    let grid: Vec<u32> = vec![0; rows * cols];

    lines
        .into_iter()
        .enumerate()
        .fold(grid, |mut x, (i, line)| {
            let start_idx = i * cols;
            let end_idx = start_idx + cols;
            x[start_idx..end_idx].copy_from_slice(
                &line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>(),
            );
            x
        })
}

pub fn get_visible_trees(input_file: &str) -> u32 {
    let grid: Vec<u32> = parse_input(input_file);

    let rows = (grid.len() as f64).sqrt() as usize;
    let cols = rows;
    let outer_visible_trees: u32 = (cols + (rows - 1) + (cols - 1) + (rows - 2))
        .try_into()
        .unwrap();

    let mut inner_visible_trees: u32 = 0;

    let rows = (grid.len() as f64).sqrt() as usize;
    let cols = rows;
    for i in 0..rows {
        for j in 0..cols {
            if i > 0 && i < rows - 1 && j > 0 && j < cols - 1 {
                let val = grid.get(i * cols + j);
                let row_view = &grid[i * cols..(i + 1) * cols];
                let col_view: Vec<u32> = (0..rows).map(|row| grid[row * cols + j]).collect();

                let slice_left = &row_view[0..j];
                let slice_right = &row_view[j + 1..];
                let slice_up = &col_view[0..i];
                let slice_down = &col_view[i + 1..];

                if val > slice_left.iter().max() {
                    inner_visible_trees += 1;
                    continue;
                }
                if val > slice_right.iter().max() {
                    inner_visible_trees += 1;
                    continue;
                }
                if val > slice_up.iter().max() {
                    inner_visible_trees += 1;
                    continue;
                }
                if val > slice_down.iter().max() {
                    inner_visible_trees += 1;
                    continue;
                }
            }
        }
    }

    outer_visible_trees + inner_visible_trees
}

pub fn get_viewing_distance(val: u32, slice: Vec<u32>) -> u32 {
    let mut count = 0;
    for x in slice {
        count += 1;
        if x >= val {
            break;
        }
    }
    count
}

pub fn get_highest_scenic_score(input_file: &str) -> u32 {
    let grid: Vec<u32> = parse_input(input_file);

    let mut scenic_scores: Vec<u32> = vec![];

    let rows = (grid.len() as f64).sqrt() as usize;
    for i in 0..rows {
        let cols = (grid.len() as f64).sqrt() as usize;
        for j in 0..cols {
            let val = grid.get(i * cols + j);
            let row_view = &grid[i * rows..(i + 1) * rows];
            let col_view: Vec<u32> = (0..rows).map(|row| grid[row * rows + j]).collect();

            let slice_left = &row_view[0..j];
            let slice_right = &row_view[j + 1..];
            let slice_up = &col_view[0..i];
            let slice_down = &col_view[i + 1..];

            let viewing_distance_left =
                get_viewing_distance(*val.unwrap(), slice_left.iter().copied().rev().collect());
            let viewing_distance_right = get_viewing_distance(*val.unwrap(), slice_right.to_vec());
            let viewing_distance_up = get_viewing_distance(
                *val.unwrap(),
                slice_up.to_vec().to_vec().into_iter().rev().collect(),
            );
            let viewing_distance_down = get_viewing_distance(*val.unwrap(), slice_down.to_vec());

            let scenic_score = viewing_distance_left
                * viewing_distance_right
                * viewing_distance_up
                * viewing_distance_down;

            scenic_scores.push(scenic_score);
        }
    }

    scenic_scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_visible_trees01() {
        assert_eq!(21, get_visible_trees("input/2022/day08_test01.txt"));
    }

    #[test]
    fn test_get_visible_trees() {
        assert_eq!(1715, get_visible_trees("input/2022/day08.txt"));
    }

    #[test]
    fn test_get_viewing_distance_01_left() {
        assert_eq!(
            1,
            get_viewing_distance(5, vec![2, 5].into_iter().rev().collect())
        );
    }

    #[test]
    fn test_get_viewing_distance_01_right() {
        assert_eq!(2, get_viewing_distance(5, vec![1, 2]));
    }

    #[test]
    fn test_get_viewing_distance_01_up() {
        assert_eq!(
            1,
            get_viewing_distance(5, vec![3].into_iter().rev().collect())
        );
    }

    #[test]
    fn test_get_viewing_distance_01_down() {
        assert_eq!(2, get_viewing_distance(5, vec![3, 5, 5]));
    }

    #[test]
    fn test_get_highest_scenic_score01() {
        assert_eq!(8, get_highest_scenic_score("input/2022/day08_test01.txt"));
    }

    #[test]
    fn test_get_highest_scenic_score() {
        assert_eq!(374400, get_highest_scenic_score("input/2022/day08.txt"));
    }
}
