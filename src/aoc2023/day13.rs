// https://adventofcode.com/2023/day/13

use std::{
    cmp,
    collections::{HashMap, HashSet},
    iter::zip,
};

use itertools::Itertools;

use crate::utils::get_lines;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Ground {
    #[default]
    Ash,
    Rocks,
}

pub fn get_column(matrix: &[Vec<Ground>], col_index: usize) -> Vec<Ground> {
    matrix.iter().map(|row| row[col_index]).collect()
}

#[derive(Debug)]
pub struct Input {
    pub mirrors: Vec<Vec<Vec<Ground>>>,
}

pub fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let iter = lines.split(|e| e.is_empty());

    let mut mirrors: Vec<Vec<Vec<Ground>>> = vec![];
    for ground_strs in iter {
        let mirror = parse_ground(ground_strs.iter().collect_vec());
        mirrors.push(mirror);
    }
    Input { mirrors }
}

pub fn parse_ground(ground_lines: Vec<&String>) -> Vec<Vec<Ground>> {
    let mut ground = Vec::new();
    for ground_line in ground_lines.into_iter() {
        let mut ground_entries: Vec<Ground> = Vec::new();
        for ground_entry in ground_line.chars() {
            match ground_entry {
                '.' => ground_entries.push(Ground::Ash),
                '#' => ground_entries.push(Ground::Rocks),
                _ => panic!("invalid ground"),
            }
        }
        ground.push(ground_entries)
    }
    ground
}

pub fn get_reflections(
    row_cache: &mut HashSet<(usize, usize)>,
    col_cache: &mut HashSet<(usize, usize)>,
    mirror: &[Vec<Ground>],
    find_different: bool,
) -> u32 {
    let mut row_reflection_lens: HashMap<usize, usize> = HashMap::new();
    let mut col_reflection_lens: HashMap<usize, usize> = HashMap::new();

    let mut reflections = 0;

    let mut row_reflections: Vec<usize> = vec![];
    for (row_index, (row_a, row_b)) in mirror.iter().tuple_windows().enumerate() {
        if find_different
            && off_by_one_or_equal(row_a.iter().collect_vec(), row_b.iter().collect_vec())
        {
            row_reflections.push(row_index);
        }

        if !find_different && row_a.iter().collect_vec() == row_b.iter().collect_vec() {
            row_reflections.push(row_index);
        }
    }
    println!("row_reflections: {:?}", row_reflections);

    let mut col_reflections: Vec<usize> = vec![];
    for col_index in 0..mirror[0].len() {
        let col_a = get_column(mirror, col_index);
        let col_b = if col_index + 1 < mirror[0].len() {
            get_column(mirror, col_index + 1)
        } else {
            continue;
        };
        if find_different
            && off_by_one_or_equal(col_a.iter().collect_vec(), col_b.iter().collect_vec())
        {
            col_reflections.push(col_index);
        }

        if !find_different && col_a == col_b {
            col_reflections.push(col_index);
        }
    }
    println!("col_reflections: {:?}", col_reflections);

    for row_reflection_index in row_reflections {
        let mut row_reflection_len = 0;
        let up_range = (0..row_reflection_index + 1).rev();
        let down_range = row_reflection_index + 1..mirror.len();
        for (row_a, row_b) in zip(up_range.clone(), down_range.clone()) {
            if find_different
                && off_by_one_or_equal(
                    mirror[row_a].iter().collect_vec(),
                    mirror[row_b].iter().collect_vec(),
                )
            {
                row_reflection_len += 1;
            }

            if !find_different
                && mirror[row_a].iter().collect_vec() == mirror[row_b].iter().collect_vec()
            {
                row_reflection_len += 1;
            }
        }
        if row_reflection_len == cmp::min(up_range.len(), down_range.len())
            && !row_cache.contains(&(row_reflection_len, row_reflection_index))
        {
            row_reflection_lens.insert(row_reflection_len, row_reflection_index);
        }
    }

    for col_reflection_index in col_reflections {
        let mut col_reflection_len = 0;
        let left_range = (0..col_reflection_index + 1).rev();
        let right_range = col_reflection_index + 1..mirror[0].len();
        for (col_a, col_b) in zip(left_range.clone(), right_range.clone()) {
            if find_different
                && off_by_one_or_equal(
                    get_column(mirror, col_a).iter().collect_vec(),
                    get_column(mirror, col_b).iter().collect_vec(),
                )
            {
                col_reflection_len += 1;
            }

            if !find_different && get_column(mirror, col_a) == get_column(mirror, col_b) {
                col_reflection_len += 1;
            }
        }
        if col_reflection_len == cmp::min(left_range.len(), right_range.len())
            && !col_cache.contains(&(col_reflection_len, col_reflection_index))
        {
            col_reflection_lens.insert(col_reflection_len, col_reflection_index);
        }
    }

    println!("row_reflection_lens: {:?}", row_reflection_lens);
    println!("col_reflection_lens: {:?}", col_reflection_lens);

    let max_row_reflection_len = *(row_reflection_lens.keys().clone().max().unwrap_or(&0));
    let max_col_reflection_len = *(col_reflection_lens.keys().clone().max().unwrap_or(&0));

    if max_col_reflection_len > max_row_reflection_len && !col_reflection_lens.is_empty() {
        if let Some(num_left_cols) = col_reflection_lens.get(&max_col_reflection_len) {
            println!("num_left_cols: {:?}", num_left_cols + 1);
            reflections = (num_left_cols + 1) as u32;
            col_cache.insert((max_col_reflection_len, *num_left_cols));
        }
    } else if !row_reflection_lens.is_empty()
        && let Some(num_above_rows) = row_reflection_lens.get(&max_row_reflection_len)
    {
        println!("num_above_rows: {:?}", num_above_rows + 1);
        reflections = 100 * (num_above_rows + 1) as u32;
        row_cache.insert((max_row_reflection_len, *num_above_rows));
    }

    reflections
}

pub fn get_sum_reflections(input_file: &str, find_different: bool) -> u32 {
    let mut sum_reflections: u32 = 0;

    let input = parse_input(input_file);
    for mirror in input.mirrors {
        println!("***** checking mirror *****");

        if find_different {
            let mut row_cache: HashSet<(usize, usize)> = HashSet::new();
            let mut col_cache: HashSet<(usize, usize)> = HashSet::new();
            get_reflections(&mut row_cache, &mut col_cache, &mirror, false);
            sum_reflections += get_reflections(&mut row_cache, &mut col_cache, &mirror, true);
        } else {
            sum_reflections +=
                get_reflections(&mut HashSet::new(), &mut HashSet::new(), &mirror, false);
        }
    }
    sum_reflections
}

fn off_by_one_or_equal(vec_a: Vec<&Ground>, vec_b: Vec<&Ground>) -> bool {
    let mut diff_count = 0;
    for (a, b) in zip(vec_a, vec_b) {
        if a != b {
            diff_count += 1;
        }
    }
    diff_count == 1 || diff_count == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_reflections_test01() {
        assert_eq!(5, get_sum_reflections("input/2023/day13_test01.txt", false));
    }

    #[test]
    fn test_get_sum_reflections_test02() {
        assert_eq!(
            400,
            get_sum_reflections("input/2023/day13_test02.txt", false)
        );
    }

    #[test]
    fn test_get_sum_reflections_test03() {
        assert_eq!(
            405,
            get_sum_reflections("input/2023/day13_test03.txt", false)
        );
    }

    #[test]
    fn test_get_sum_reflections_test04() {
        assert_eq!(3, get_sum_reflections("input/2023/day13_test04.txt", false));
    }

    #[test]
    fn test_get_sum_reflections_test05() {
        assert_eq!(
            100,
            get_sum_reflections("input/2023/day13_test05.txt", false)
        );
    }

    #[test]
    fn test_get_sum_reflections_test06() {
        assert_eq!(1, get_sum_reflections("input/2023/day13_test06.txt", false));
    }

    #[test]
    fn test_get_sum_reflections_part01() {
        assert_eq!(35210, get_sum_reflections("input/2023/day13.txt", false));
    }

    #[test]
    fn test_get_sum_reflections_part02_test01() {
        assert_eq!(
            300,
            get_sum_reflections("input/2023/day13_test01.txt", true)
        );
    }

    #[test]
    fn test_get_sum_reflections_part02_test02() {
        assert_eq!(
            100,
            get_sum_reflections("input/2023/day13_test02.txt", true)
        );
    }

    #[test]
    fn test_get_sum_reflections_part02_test03() {
        assert_eq!(
            400,
            get_sum_reflections("input/2023/day13_test03.txt", true)
        );
    }

    #[test]
    fn test_get_sum_reflections_part02_test04() {
        assert_eq!(14, get_sum_reflections("input/2023/day13_test04.txt", true));
    }

    #[test]
    fn test_get_sum_reflections_part02_test05() {
        assert_eq!(12, get_sum_reflections("input/2023/day13_test05.txt", true));
    }

    #[test]
    fn test_get_sum_reflections_part02_test06() {
        assert_eq!(
            1400,
            get_sum_reflections("input/2023/day13_test06.txt", true)
        );
    }

    #[test]
    fn test_get_sum_reflections_part02() {
        assert_eq!(31974, get_sum_reflections("input/2023/day13.txt", true));
    }
}
