// https://adventofcode.com/2019/day/10

use crate::utils::get_lines;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SpaceLocation {
    Asteroid = b'#',
    Space = b'.',
}

impl TryFrom<u8> for SpaceLocation {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == SpaceLocation::Asteroid as u8 => Ok(SpaceLocation::Asteroid),
            x if x == SpaceLocation::Space as u8 => Ok(SpaceLocation::Space),
            _ => Err(()),
        }
    }
}

struct Input {
    space: Vec<Vec<SpaceLocation>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut space: Vec<Vec<SpaceLocation>> = vec![];

    for line in lines {
        let row: Vec<SpaceLocation> = line
            .chars()
            .map(|c| SpaceLocation::try_from(c as u8).unwrap())
            .collect();
        space.push(row);
    }

    Input { space }
}

/// Generates points along a line using Bresenham's line algorithm
pub fn bresenham_line(origin: (usize, usize), dest: (usize, usize)) -> Vec<(usize, usize)> {
    let (mut x0, mut y0) = (origin.0 as isize, origin.1 as isize);
    let (x1, y1) = (dest.0 as isize, dest.1 as isize);

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;

    let mut points = Vec::new();

    loop {
        points.push((x0 as usize, y0 as usize));

        let e2 = 2 * error;
        if e2 >= dy {
            if x0 == x1 {
                break;
            }
            error += dy;
            x0 += sx;
        }
        if e2 <= dx {
            if y0 == y1 {
                break;
            }
            error += dx;
            y0 += sy;
        }
    }

    points
}

/// Calculate the gradient between two points
pub fn calculate_gradient(origin: (usize, usize), point: (usize, usize)) -> f32 {
    let y_change = point.1 as isize - origin.1 as isize;
    let x_change = point.0 as isize - origin.0 as isize;

    if x_change == 0 {
        f32::INFINITY
    } else {
        y_change as f32 / x_change as f32
    }
}

/// Check if an asteroid is visible from the origin based on gradient analysis
pub fn check_visibility(gradients: &[f32]) -> bool {
    if gradients.len() <= 1 {
        return true;
    }

    if let Some(last_grad) = gradients.last() {
        let count = gradients
            .iter()
            .filter(|&&grad| {
                if grad.is_infinite() && last_grad.is_infinite() {
                    grad.is_sign_positive() == last_grad.is_sign_positive()
                } else {
                    (grad - last_grad).abs() < f32::EPSILON
                }
            })
            .count();

        count <= 1
    } else {
        true
    }
}

pub fn is_asteroid_detectable(
    space: &[Vec<SpaceLocation>],
    origin: (usize, usize),
    dest: (usize, usize),
) -> bool {
    //println!("Origin: {:?}, Dest: {:?}", origin, dest);

    let points = bresenham_line(origin, dest);
    let mut grads: Vec<f32> = vec![];

    for &(x, y) in &points {
        match space[y][x] {
            SpaceLocation::Asteroid => {
                if (x, y) != origin {
                    let grad = calculate_gradient(origin, (x, y));
                    grads.push(grad);
                    /*println!(
                        "Asteroid at (x, y) = ({}, {}), grad = {}",
                        x, y, grad
                    );*/
                }
            }
            SpaceLocation::Space => {
                /*if (x, y) != origin {
                    println!("Space at (x, y) = ({}, {})", x, y);
                }*/
            }
        }
    }

    /*if is_visible {
        println!("Asteroid at (x, y) = ({}, {}) is visible", dest.0, dest.1);
    } else {
        println!("Asteroid at (x, y) = ({}, {}) is invisible", dest.0, dest.1);
    }*/

    check_visibility(&grads)
}

pub fn get_detected_asteroids_for_entry(
    space: &[Vec<SpaceLocation>],
    origin: (usize, usize),
) -> u32 {
    let mut detected_asteroids = 0;

    for (i, space_col) in space.iter().enumerate() {
        for (j, space_entry) in space_col.iter().enumerate() {
            let dest = (j, i);
            if space_entry == &SpaceLocation::Asteroid
                && origin != dest
                && is_asteroid_detectable(space, origin, dest)
            {
                detected_asteroids += 1;
            }
        }
    }

    detected_asteroids
}

pub fn get_detected_asteroids(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut detected_asteroids: Vec<u32> = vec![];

    for (i, space_col) in input.space.iter().enumerate() {
        for (j, space_entry) in space_col.iter().enumerate() {
            if space_entry == &SpaceLocation::Asteroid {
                detected_asteroids.push(get_detected_asteroids_for_entry(&input.space, (j, i)));
            }
        }
    }

    *detected_asteroids.iter().max().unwrap_or(&0)
}

pub fn get_vaporised_asteroids(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut detected_asteroids: Vec<(u32, (usize, usize))> = vec![];

    for (i, space_col) in input.space.iter().enumerate() {
        for (j, space_entry) in space_col.iter().enumerate() {
            if space_entry == &SpaceLocation::Asteroid {
                detected_asteroids.push((
                    get_detected_asteroids_for_entry(&input.space, (j, i)),
                    (j, i),
                ));
            }
        }
    }

    if let Some((_max_visible_asteroids, max_visible_asteroids_pos)) =
        detected_asteroids.iter().max()
    {
        println!("Max visible asteroids pos {:?}", *max_visible_asteroids_pos);
        return vaporise_asteroids(&mut input.space.clone(), *max_visible_asteroids_pos);
    }

    0
}

fn vaporise_asteroids(space: &mut [Vec<SpaceLocation>], station_point: (usize, usize)) -> u32 {
    let width = space.first().map_or(0, |row| row.len());
    let height = space.len();

    println!("width = {}, height = {}", width, height);

    let mut vaporised_asteroids = 0;

    // Construct the list of points we need to visit
    let mut visit_points: Vec<(usize, usize)> = vec![];

    // Loop top row, north from station, to east
    for x in station_point.0..width {
        visit_points.push((x, 0));
    }

    // Loop east most column, skipping the top entry
    for y in 1..height {
        visit_points.push((width - 1, y));
    }

    // Loop bottom row, from east to west, skipping rightmost entry
    for x in (0..(width - 1)).rev() {
        visit_points.push((x, height - 1));
    }

    // Loop west most column, skipping the bottom entry
    for y in (0..(height - 1)).rev() {
        visit_points.push((0, y));
    }

    // Loop top row from west to east, skipping the first column, up until the start point
    for x in 1..station_point.0 {
        visit_points.push((x, 0));
    }

    loop {
        if let Some(visit_point) = visit_points.first() {
            if vaporise_asteroid(space, station_point, *visit_point) {
                vaporised_asteroids += 1;
            }
        }

        visit_points.rotate_left(1);

        break;
    }

    vaporised_asteroids
}

fn vaporise_asteroid(
    space: &mut [Vec<SpaceLocation>],
    origin: (usize, usize),
    dest: (usize, usize),
) -> bool {
    //println!("Origin: {:?}, Dest: {:?}", origin, dest);

    let points = bresenham_line(origin, dest);

    for &(x, y) in &points {
        match space[y][x] {
            SpaceLocation::Asteroid => {
                if (x, y) != origin {
                    space[y][x] = SpaceLocation::Space;
                    return true;
                }
            }
            SpaceLocation::Space => {
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_detected_asteroids_test01() {
        assert_eq!(8, get_detected_asteroids("input/2019/day10_test01.txt"));
    }

    #[test]
    fn test_get_detected_asteroids_test02() {
        assert_eq!(33, get_detected_asteroids("input/2019/day10_test02.txt"))
    }

    #[test]
    fn test_get_detected_asteroids_test03() {
        assert_eq!(35, get_detected_asteroids("input/2019/day10_test03.txt"));
    }

    #[test]
    fn test_get_detected_asteroids_test04() {
        assert_eq!(41, get_detected_asteroids("input/2019/day10_test04.txt"));
    }

    #[test]
    fn test_get_detected_asteroids_test05() {
        assert_eq!(210, get_detected_asteroids("input/2019/day10_test05.txt"));
    }

    #[test]
    fn test_get_detected_asteroids_test06() {
        assert_eq!(3, get_detected_asteroids("input/2019/day10_test06.txt"));
    }

    #[test]
    fn test_get_detected_asteroids() {
        assert_eq!(284, get_detected_asteroids("input/2019/day10.txt"));
    }

    #[test]
    fn test_get_vaporised_asteroids_test01() {
        assert_eq!(0, get_vaporised_asteroids("input/2019/day10_test07.txt"));
    }

    #[test]
    fn test_get_vaporised_asteroids_test02() {
        assert_eq!(802, get_vaporised_asteroids("input/2019/day10_test06.txt"));
    }

    #[test]
    fn test_get_vaporised_asteroids() {
        assert_eq!(0, get_vaporised_asteroids("input/2019/day10.txt"));
    }
}
