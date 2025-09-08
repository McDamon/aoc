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

pub fn is_asteroid_detectable(
    space: &[Vec<SpaceLocation>],
    origin: (usize, usize),
    dest: (usize, usize),
) -> bool {
    //println!("Origin: {:?}, Dest: {:?}", origin, dest);

    // Bresenham's algo

    let (mut x0, mut y0) = (origin.0 as isize, origin.1 as isize);
    let (x1, y1) = (dest.0 as isize, dest.1 as isize);

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;

    let mut grads: Vec<f32> = vec![];

    loop {
        let (x, y) = (x0 as usize, y0 as usize);

        match space[y][x] {
            SpaceLocation::Asteroid => {
                if (x, y) != origin {
                    let y_change = y0 - (origin.1 as isize);
                    let x_change = x0 - (origin.0 as isize);
                    let grad = if x_change == 0 {
                        f32::INFINITY
                    } else {
                        y_change as f32 / x_change as f32
                    };
                    grads.push(grad);
                    /*println!(
                        "Asteroid at (x, y) = ({}, {}), y_change = {}, x_change = {}, grad = {}",
                        x0, y0, y_change, x_change, grad
                    );*/
                }
            }
            SpaceLocation::Space => {
                /*if (x, y) != origin {
                    println!("Space at (x, y) = ({}, {})", x0, y0);
                }*/
            }
        }

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

    if grads.len() > 1
        && let Some(last_grad) = grads.last()
    {
        let count = grads
            .iter()
            .filter(|&&grad| {
                if grad.is_infinite() && last_grad.is_infinite() {
                    grad.is_sign_positive() == last_grad.is_sign_positive()
                } else {
                    (grad - last_grad).abs() < f32::EPSILON
                }
            })
            .count();
        if count > 1 {
            //println!("Asteroid at (x, y) = ({}, {}) is invisible", dest.0, dest.1);
            return false;
        }
    }

    //println!("Asteroid at (x, y) = ({}, {}) is visible", dest.0, dest.1);
    true
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
}
