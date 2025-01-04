// https://adventofcode.com/2024/day/12

use std::collections::{HashMap, HashSet};

use super::utils::get_lines;

struct Input {
    plants: Vec<Vec<char>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut plants: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let report: Vec<char> = line.chars().collect();
        plants.push(report);
    }

    Input { plants }
}

fn print_plants(plants: &Vec<Vec<char>>) {
    for row in plants {
        for level in row {
            print!("{}", level);
        }
        println!();
    }
}

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    corners: usize,
}

impl Region {
    fn new(area: usize, perimeter: usize, corners: usize) -> Region {
        Region {
            area,
            perimeter,
            corners,
        }
    }
}

fn flood_fill(
    plant: char,
    plant_pos: (usize, usize),
    plants: &[Vec<char>],
    regions: &mut HashMap<char, Vec<Region>>,
    visited_plants: &mut HashSet<(usize, usize)>,
) {
    let mut stack: Vec<(usize, usize)> = Vec::new();

    // Add current node to the stack
    stack.push(plant_pos);

    let mut area = 0;
    let mut perimeter = 0;
    let mut corners = 0;

    while let Some((row, col)) = stack.pop() {
        // This makes sure we don't double count positions
        if visited_plants.contains(&(row, col)) {
            continue;
        }

        // Is this 'Inside'?
        if plants[row][col] == plant {
            // If so, increment the area, and mark as visited
            area += 1;
            visited_plants.insert((row, col));

            let n_dir = if row as i32 - 1 < 0 || plants[row - 1][col] != plant {
                perimeter += 1;
                None
            } else {
                stack.push((row - 1, col));
                Some((row - 1, col))
            };
            let nw_dir =
                if row as i32 - 1 < 0 || col as i32 - 1 < 0 || plants[row - 1][col - 1] != plant {
                    None
                } else {
                    Some((row - 1, col - 1))
                };
            let w_dir = if col as i32 - 1 < 0 || plants[row][col - 1] != plant {
                perimeter += 1;
                None
            } else {
                stack.push((row, col - 1));
                Some((row, col - 1))
            };
            let sw_dir = if row as i32 + 1 >= plants.len() as i32
                || col as i32 - 1 < 0
                || plants[row + 1][col - 1] != plant
            {
                None
            } else {
                Some((row + 1, col - 1))
            };
            let s_dir = if row as i32 + 1 >= plants.len() as i32 || plants[row + 1][col] != plant {
                perimeter += 1;
                None
            } else {
                stack.push((row + 1, col));
                Some((row + 1, col))
            };
            let se_dir = if row as i32 + 1 >= plants.len() as i32
                || col as i32 + 1 >= plants[0].len() as i32
                || plants[row + 1][col + 1] != plant
            {
                None
            } else {
                Some((row + 1, col + 1))
            };
            let e_dir = if col as i32 + 1 >= plants[0].len() as i32 || plants[row][col + 1] != plant
            {
                perimeter += 1;
                None
            } else {
                stack.push((row, col + 1));
                Some((row, col + 1))
            };
            let ne_dir = if row as i32 - 1 < 0
                || col as i32 + 1 >= plants[0].len() as i32
                || plants[row - 1][col + 1] != plant
            {
                None
            } else {
                Some((row - 1, col + 1))
            };

            // We want to check concave and convex corners

            // Convex corners
            // ...
            // ###<- This is a convex corner
            // ###

            if n_dir.is_none() && e_dir.is_none() {
                corners += 1;
            }
            if s_dir.is_none() && e_dir.is_none() {
                corners += 1;
            }
            if s_dir.is_none() && w_dir.is_none() {
                corners += 1;
            }
            if n_dir.is_none() && w_dir.is_none() {
                corners += 1;
            }

            // Concave corners
            //                           #..
            //                           ##.
            // This is a concave corner->###

            if n_dir.is_some() && e_dir.is_some() && ne_dir.is_none() {
                corners += 1;
            }
            if s_dir.is_some() && e_dir.is_some() && se_dir.is_none() {
                corners += 1;
            }
            if s_dir.is_some() && w_dir.is_some() && sw_dir.is_none() {
                corners += 1;
            }
            if n_dir.is_some() && w_dir.is_some() && nw_dir.is_none() {
                corners += 1;
            }
        }
    }

    if area > 0 && perimeter > 0 {
        /*println!(
            "Plant: {}, Area: {}, Perimeter: {}, Corners: {}",
            plant, area, perimeter, corners
        );*/
        // Once we are done, we can add the new region
        regions
            .entry(plant)
            .or_default()
            .push(Region::new(area, perimeter, corners));
    }
}

fn get_price_fencing_all_regions(input_file: &str, apply_discount: bool) -> usize {
    let input = parse_input(input_file);

    //print_plants(&input.plants);

    let plant_pos_list: Vec<(char, (usize, usize))> = input
        .plants
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_index, plant)| (*plant, (row_index, col_index)))
        })
        .collect::<Vec<(char, (usize, usize))>>();

    let mut regions: HashMap<char, Vec<Region>> = HashMap::new();
    let mut visited_plants: HashSet<(usize, usize)> = HashSet::new();

    for (plant, (row, col)) in plant_pos_list {
        flood_fill(
            plant,
            (row, col),
            &input.plants,
            &mut regions,
            &mut visited_plants,
        );
    }

    //println!("Regions: {:?}", regions);

    if apply_discount {
        regions
            .into_values()
            .map(|region| {
                region
                    .into_iter()
                    .map(|r| r.area * r.corners)
                    .sum::<usize>()
            })
            .sum()
    } else {
        regions
            .into_values()
            .map(|region| {
                region
                    .into_iter()
                    .map(|r| r.area * r.perimeter)
                    .sum::<usize>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_price_fencing_all_regions_test01() {
        assert_eq!(
            140,
            get_price_fencing_all_regions("input/day12_test01.txt", false)
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions_test02() {
        assert_eq!(
            772,
            get_price_fencing_all_regions("input/day12_test02.txt", false)
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions_test03() {
        assert_eq!(
            1930,
            get_price_fencing_all_regions("input/day12_test03.txt", false)
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions_test04() {
        assert_eq!(
            692,
            get_price_fencing_all_regions("input/day12_test04.txt", false)
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions_test05() {
        assert_eq!(
            1184,
            get_price_fencing_all_regions("input/day12_test05.txt", false)
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions() {
        assert_eq!(
            1363682,
            get_price_fencing_all_regions("input/day12.txt", false)
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions_bulk_discount_test01() {
        assert_eq!(
            80,
            get_price_fencing_all_regions("input/day12_test01.txt", true)
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions_bulk_discount_test02() {
        assert_eq!(
            436,
            get_price_fencing_all_regions("input/day12_test02.txt", true)
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions_bulk_discount_test03() {
        assert_eq!(
            1206,
            get_price_fencing_all_regions("input/day12_test03.txt", true)
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions_bulk_discount_test04() {
        assert_eq!(
            236,
            get_price_fencing_all_regions("input/day12_test04.txt", true)
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions_bulk_discount_test05() {
        assert_eq!(
            368,
            get_price_fencing_all_regions("input/day12_test05.txt", true)
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions_bulk_discount_() {
        assert_eq!(
            787680,
            get_price_fencing_all_regions("input/day12.txt", true)
        );
    }
}
