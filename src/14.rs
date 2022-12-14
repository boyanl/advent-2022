use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::{self, stdin};

fn sign(x: i32) -> i32 {
    if x > 0 {
        return 1;
    } else if x == 0 {
        return 0;
    }
    return -1;
}

fn part_one() {
    let mut taken = HashSet::new();

    let mut rock_max_y = -1;

    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut prev = (-1, -1);
        for part in line.split(" -> ") {
            let coords_vec: Vec<i32> = part.split(",").map(|p| p.parse::<i32>().unwrap()).collect();
            let (x, y) = (coords_vec[0], coords_vec[1]);
            if rock_max_y == -1 || rock_max_y < y {
                rock_max_y = y;
            }

            if prev.0 == -1 {
                prev = (x, y);
                continue;
            }
            let (dx, dy) = (sign(x - prev.0), sign(y - prev.1));

            if dx != 0 {
                for nx in min(prev.0, x)..=max(prev.0, x) {
                    taken.insert((nx, y));
                }
            } else if dy != 0 {
                for ny in min(prev.1, y)..=max(prev.1, y) {
                    taken.insert((x, ny));
                }
            }
            prev = (x, y);
        }
    }

    let src = (500, 0);
    let mut accumulated_sand = 0;
    let mut flowing_out = false;
    loop {
        let mut sand_coords = src.clone();
        loop {
            if sand_coords.1 >= rock_max_y {
                println!("Flowing out! {:?}", sand_coords);
                flowing_out = true;
                break;
            }
            let down = (sand_coords.0, sand_coords.1 + 1);
            if !taken.contains(&down) {
                sand_coords = down;
                continue;
            }
            let down_left = (sand_coords.0 - 1, sand_coords.1 + 1);
            if !taken.contains(&down_left) {
                sand_coords = down_left;
                continue;
            }
            let down_right = (sand_coords.0 + 1, sand_coords.1 + 1);
            if !taken.contains(&down_right) {
                sand_coords = down_right;
                continue;
            }
            // all taken, sand remains in place
            taken.insert(sand_coords);
            accumulated_sand += 1;
            break;
        }
        if flowing_out {
            break;
        }
    }

    println!("{accumulated_sand}");
}

fn part_two() {
    let mut taken = HashSet::new();

    let mut rock_max_y = -1;
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut prev = (-1, -1);
        for part in line.split(" -> ") {
            let coords_vec: Vec<i32> = part.split(",").map(|p| p.parse::<i32>().unwrap()).collect();
            let (x, y) = (coords_vec[0], coords_vec[1]);
            if rock_max_y == -1 || rock_max_y < y {
                rock_max_y = y;
            }

            if prev.0 == -1 {
                prev = (x, y);
                continue;
            }
            let (dx, dy) = (sign(x - prev.0), sign(y - prev.1));

            if dx != 0 {
                for nx in min(prev.0, x)..=max(prev.0, x) {
                    taken.insert((nx, y));
                }
            } else if dy != 0 {
                for ny in min(prev.1, y)..=max(prev.1, y) {
                    taken.insert((x, ny));
                }
            }
            prev = (x, y);
        }
    }

    let floor_y = rock_max_y + 2;
    let is_floor = |pos: (i32, i32)| pos.1 == floor_y;

    let src = (500, 0);

    let mut accumulated_sand = 0;
    let mut src_blocked = false;
    loop {
        let mut sand_coords = src.clone();
        loop {
            let down = (sand_coords.0, sand_coords.1 + 1);
            if !is_floor(down) && !taken.contains(&down) {
                sand_coords = down;
                continue;
            }
            let down_left = (sand_coords.0 - 1, sand_coords.1 + 1);
            if !is_floor(down_left) && !taken.contains(&down_left) {
                sand_coords = down_left;
                continue;
            }
            let down_right = (sand_coords.0 + 1, sand_coords.1 + 1);
            if !is_floor(down_right) && !taken.contains(&down_right) {
                sand_coords = down_right;
                continue;
            }

            // all taken, sand remains in place
            taken.insert(sand_coords);
            accumulated_sand += 1;
            if sand_coords == src {
                src_blocked = true;
            }
            break;
        }
        if src_blocked {
            break;
        }
    }

    println!("{accumulated_sand}");
}

fn main() {
    part_two();
}
