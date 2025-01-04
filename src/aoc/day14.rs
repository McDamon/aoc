// https://adventofcode.com/2024/day/14

use std::collections::HashMap;

use regex::Regex;

use super::utils::get_lines;

#[derive(Debug, Clone)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

struct Input {
    robots: Vec<Robot>,
}

fn parse_coordinates(input: &str) -> ((i32, i32), (i32, i32)) {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let caps = re.captures(input).unwrap();

    let p1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let p2 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let v1 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let v2 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

    ((p1, p2), (v1, v2))
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut robots: Vec<Robot> = Vec::new();

    for line in lines {
        let (pos, vel) = parse_coordinates(&line);
        robots.push(Robot { pos, vel });
    }

    Input { robots }
}

fn print_robots(width: i32, height: i32, robot_map: &HashMap<(i32, i32), Vec<Robot>>) {
    for row in 0..height {
        for col in 0..width {
            let mut found = false;
            if let Some(robots) = robot_map.get(&(col, row)) {
                print!("{}", robots.len());
                found = true;
            }
            if !found {
                print!(".");
            }
        }
        println!();
    }
}

fn get_safety_factor(input_file: &str, width: i32, height: i32, num_secs: usize) -> usize {
    let input = parse_input(input_file);

    let mut robots = input.robots.clone();

    for _ in 0..num_secs {
        for robot in &mut robots {
            let (old_x, old_y) = robot.pos;
            let (dx, dy) = robot.vel;

            let new_x = if old_x + dx < 0 {
                dx + width + old_x
            } else if old_x + dx >= width {
                dx - width + old_x
            } else {
                old_x + dx
            };
            let new_y = if old_y + dy < 0 {
                dy + height + old_y
            } else if old_y + dy >= height {
                dy - height + old_y
            } else {
                old_y + dy
            };

            robot.pos = (new_x, new_y);

            /*println!(
                "Secs: {}, old_x: {}, old_y: {}, dx: {}, dy: {}, new_x: {}, new_y: {}",
                secs + 1,
                old_x,
                old_y,
                dx,
                dy,
                new_x,
                new_y
            );*/
        }
    }

    let mut robot_map: HashMap<(i32, i32), Vec<Robot>> = HashMap::new();
    for robot in robots {
        robot_map.entry(robot.pos).or_default().push(robot);
    }

    print_robots(width, height, &robot_map);

    let quad_width = width as usize / 2;
    let quad_height = height as usize / 2;

    //println!("quad_width: {}, quad_height: {}", quad_width, quad_height);

    let mut ne_robots = 0;
    let mut nw_robots = 0;
    let mut se_robots = 0;
    let mut sw_robots = 0;

    for (pos, robots) in robot_map {
        for _ in robots {
            let (x, y) = pos;
            let (x, y) = (x as usize, y as usize);
            if x < quad_width && y < quad_height {
                ne_robots += 1;
            } else if x > quad_width && y < quad_height {
                nw_robots += 1;
            } else if x < quad_width && y > quad_height {
                se_robots += 1;
            } else if x > quad_width && y > quad_height {
                sw_robots += 1;
            }
        }
    }

    ne_robots * nw_robots * se_robots * sw_robots
}

fn check_xmas_tree(input_file: &str, width: i32, height: i32, max_secs: usize) -> usize {
    let input = parse_input(input_file);

    let mut robots = input.robots.clone();

    for secs in 0..max_secs {
        for robot in &mut robots {
            let (old_x, old_y) = robot.pos;
            let (dx, dy) = robot.vel;

            let new_x = if old_x + dx < 0 {
                dx + width + old_x
            } else if old_x + dx >= width {
                dx - width + old_x
            } else {
                old_x + dx
            };
            let new_y = if old_y + dy < 0 {
                dy + height + old_y
            } else if old_y + dy >= height {
                dy - height + old_y
            } else {
                old_y + dy
            };

            robot.pos = (new_x, new_y);
        }

        let mut robot_map: HashMap<(i32, i32), Vec<Robot>> = HashMap::new();
        for robot in &robots {
            robot_map.entry(robot.pos).or_default().push(robot.clone());
        }
        let sum_single_entries = robot_map
            .iter()
            .filter(|(_, robots)| robots.len() == 1)
            .count();

        if sum_single_entries == robots.len() {
            print_robots(width, height, &robot_map.clone());
            return secs + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_safety_factor_test01() {
        assert_eq!(0, get_safety_factor("input/day14_test01.txt", 11, 7, 0));
    }

    #[test]
    fn test_get_safety_factor_test02() {
        assert_eq!(0, get_safety_factor("input/day14_test02.txt", 11, 7, 0));
    }

    #[test]
    fn test_get_safety_factor_test03() {
        assert_eq!(0, get_safety_factor("input/day14_test02.txt", 11, 7, 1));
    }

    #[test]
    fn test_get_safety_factor_test04() {
        assert_eq!(0, get_safety_factor("input/day14_test02.txt", 11, 7, 2));
    }

    #[test]
    fn test_get_safety_factor_test05() {
        assert_eq!(0, get_safety_factor("input/day14_test02.txt", 11, 7, 3));
    }

    #[test]
    fn test_get_safety_factor_test06() {
        assert_eq!(0, get_safety_factor("input/day14_test02.txt", 11, 7, 4));
    }

    #[test]
    fn test_get_safety_factor_test07() {
        assert_eq!(0, get_safety_factor("input/day14_test02.txt", 11, 7, 5));
    }

    #[test]
    fn test_get_safety_factor_test08() {
        assert_eq!(0, get_safety_factor("input/day14_test03.txt", 11, 7, 0));
    }

    #[test]
    fn test_get_safety_factor_test09() {
        assert_eq!(0, get_safety_factor("input/day14_test03.txt", 11, 7, 1));
    }

    #[test]
    fn test_get_safety_factor_test10() {
        assert_eq!(0, get_safety_factor("input/day14_test03.txt", 11, 7, 2));
    }

    #[test]
    fn test_get_safety_factor_test11() {
        assert_eq!(0, get_safety_factor("input/day14_test03.txt", 11, 7, 3));
    }

    #[test]
    fn test_get_safety_factor_test12() {
        assert_eq!(0, get_safety_factor("input/day14_test03.txt", 11, 7, 4));
    }

    #[test]
    fn test_get_safety_factor_test13() {
        assert_eq!(0, get_safety_factor("input/day14_test03.txt", 11, 7, 5));
    }

    #[test]
    fn test_get_safety_factor_test14() {
        assert_eq!(12, get_safety_factor("input/day14_test01.txt", 11, 7, 100));
    }

    #[test]
    fn test_get_safety_factor() {
        assert_eq!(
            222901875,
            get_safety_factor("input/day14.txt", 101, 103, 100)
        );
    }

    #[ignore]
    #[test]
    fn test_check_xmas_tree() {
        assert_eq!(6243, check_xmas_tree("input/day14.txt", 101, 103, 10000));
    }
}
