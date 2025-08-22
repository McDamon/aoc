// https://adventofcode.com/2023/day/11

use std::collections::HashSet;

use crate::utils::get_lines;

pub fn get_sum_shortest_paths(input_file: &str, expansion: usize) -> usize {
    let lines = get_lines(input_file);
    let mut iter = lines.split(|e| e.is_empty());
    let image_lines = iter.next().unwrap().to_owned();
    
    // Find all galaxy positions and track which rows/cols have galaxies
    let mut galaxies = Vec::new();
    let mut galaxy_rows = HashSet::new();
    let mut galaxy_cols = HashSet::new();
    
    for (row, line) in image_lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push((row, col));
                galaxy_rows.insert(row);
                galaxy_cols.insert(col);
            }
        }
    }
    
    let mut sum_shortest_paths = 0;
    
    // For each pair of galaxies, calculate the shortest path
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (r1, c1) = galaxies[i];
            let (r2, c2) = galaxies[j];
            
            let min_row = r1.min(r2);
            let max_row = r1.max(r2);
            let min_col = c1.min(c2);
            let max_col = c1.max(c2);
            
            // Count expanded rows between the galaxies
            let expanded_rows = (min_row..max_row).filter(|&r| !galaxy_rows.contains(&r)).count();
            // Count expanded cols between the galaxies
            let expanded_cols = (min_col..max_col).filter(|&c| !galaxy_cols.contains(&c)).count();
            
            // Calculate Manhattan distance with expansion
            let normal_distance = (max_row - min_row) + (max_col - min_col);
            let expansion_distance = expanded_rows * (expansion - 1) + expanded_cols * (expansion - 1);
            
            sum_shortest_paths += normal_distance + expansion_distance;
        }
    }
    
    sum_shortest_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_shortest_paths_test01() {
        assert_eq!(374, get_sum_shortest_paths("input/2023/day11_test01.txt", 2));
    }

    #[test]
    fn test_get_sum_shortest_paths() {
        assert_eq!(10313550, get_sum_shortest_paths("input/2023/day11.txt", 2));
    }

    #[test]
    fn test_get_sum_shortest_paths_part02_test01() {
        assert_eq!(1030, get_sum_shortest_paths("input/2023/day11_test01.txt", 10));
    }

    #[test]
    fn test_get_sum_shortest_paths_part02_test02() {
        assert_eq!(8410, get_sum_shortest_paths("input/2023/day11_test01.txt", 100));
    }

    #[test]
    fn test_get_sum_shortest_paths_part02() {
        assert_eq!(
            611998089572,
            get_sum_shortest_paths("input/2023/day11.txt", 1000000)
        );
    }
}
