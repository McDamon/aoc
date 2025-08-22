// https://adventofcode.com/2023/day/11

use std::collections::VecDeque;

use grid::Grid;
use itertools::Itertools;

use crate::utils::get_lines;

#[derive(Debug, Default, Clone, Copy)]
struct GridEntry {
    space: usize,
    expansion: usize,
}

#[derive(Debug)]
struct Input {
    image: Grid<GridEntry>,
    galaxy_pairs: Vec<(usize, usize)>,
}

fn parse_input(input_file: &str, expansion: usize) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    let input = parse_image(iter.next().unwrap().to_owned(), expansion);
    Input {
        image: input.0,
        galaxy_pairs: input.1,
    }
}

fn parse_image(
    image_lines: Vec<String>,
    expansion: usize,
) -> (Grid<GridEntry>, Vec<(usize, usize)>) {
    let mut image: Grid<GridEntry> = Grid::new(0, 0);

    let mut galaxy_count: usize = 0;
    for image_line in image_lines {
        if image_line.contains('#') {
            let mut row: Vec<GridEntry> = vec![];
            for image_pixel in image_line.chars() {
                match image_pixel {
                    '#' => {
                        galaxy_count += 1;
                        row.push(GridEntry {
                            space: galaxy_count,
                            expansion: 1,
                        });
                    }
                    _ => row.push(GridEntry {
                        space: 0,
                        expansion: 1,
                    }),
                }
            }
            image.push_row(row);
        } else {
            image.push_row(vec![
                GridEntry {
                    space: 0,
                    expansion
                };
                image_line.len()
            ]);
        }
    }

    let mut expanded_image: Grid<GridEntry> = Grid::new(0, 0);

    for image_col in image.iter_cols() {
        let image_col_vec: Vec<GridEntry> = image_col.cloned().collect();
        if image_col_vec.iter().all(|&x| x.space == 0) {
            expanded_image.push_col(vec![
                GridEntry {
                    space: 0,
                    expansion
                };
                image_col_vec.len()
            ]);
        } else {
            expanded_image.push_col(image_col_vec);
        }
    }

    let galaxy_pairs: Vec<(usize, usize)> =
        (1..galaxy_count + 1).tuple_combinations().collect_vec();

    (expanded_image, galaxy_pairs)
}

fn print_image(image: &Grid<GridEntry>) {
    for image_row in image.iter_rows() {
        for image_pixel in image_row {
            if image_pixel.space != 0 {
                print!("#");
            } else if image_pixel.expansion > 1 {
                print!("@");
            } else {
                print!("{:#}", image_pixel.space);
            }
        }
        println!();
    }
}

fn get_sum_shortest_paths(input_file: &str, expansion: usize) -> usize {
    let mut sum_shortest_paths = 0;
    let input = parse_input(input_file, expansion);

    //print_image(&input.image);

    for (source, target) in input.galaxy_pairs {
        if let Some(shortest_path) = get_shortest_path(&input.image, source, target) {
            /*println!(
                "shortest path for {}, {} is: {}",
                source, target, shortest_path
            );*/
            sum_shortest_paths += shortest_path;
        }
    }

    sum_shortest_paths
}

#[derive(Debug, Copy, Clone)]
struct PixelData {
    pos: (usize, usize),
    dist: usize,
}

fn get_shortest_path(image: &Grid<GridEntry>, source: usize, target: usize) -> Option<usize> {
    let mut source_pixel_data: PixelData = PixelData {
        pos: (0, 0),
        dist: 0,
    };
    let mut visited: Grid<bool> = Grid::new(image.rows(), image.cols());

    for ((row, col), pixel) in image.indexed_iter() {
        if pixel.space == source {
            source_pixel_data.pos = (row, col);
        }
    }

    let mut queue: VecDeque<PixelData> = VecDeque::new();
    queue.push_back(source_pixel_data);
    visited[(source_pixel_data.pos.0, source_pixel_data.pos.1)] = true;

    while !queue.is_empty() {
        if let Some(pixel_data) = queue.pop_front() {
            let (row, col) = pixel_data.pos;

            if let Some(pixel) = image.get(row, col) {
                if pixel.space == target {
                    return Some(pixel_data.dist);
                }

                // Moving up
                if row as i32 > 0 && !visited[(row - 1, col)] {
                    queue.push_back(PixelData {
                        pos: (row - 1, col),
                        dist: pixel_data.dist + pixel.expansion,
                    });
                    visited[(row - 1, col)] = true;
                }

                // Moving down
                if row as i32 + 1 < image.rows() as i32 && !visited[(row + 1, col)] {
                    queue.push_back(PixelData {
                        pos: (row + 1, col),
                        dist: pixel_data.dist + pixel.expansion,
                    });
                    visited[(row + 1, col)] = true;
                }

                // Moving left
                if col as i32 > 0 && !visited[(row, col - 1)] {
                    queue.push_back(PixelData {
                        pos: (row, col - 1),
                        dist: pixel_data.dist + pixel.expansion,
                    });
                    visited[(row, col - 1)] = true;
                }

                // Moving right
                if col as i32 + 1 < image.cols() as i32 && !visited[(row, col + 1)] {
                    queue.push_back(PixelData {
                        pos: (row, col + 1),
                        dist: pixel_data.dist + pixel.expansion,
                    });
                    visited[(row, col + 1)] = true;
                }
            }
        }
    }

    None
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
