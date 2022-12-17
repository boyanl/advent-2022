use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::io::{self, stdin};

mod util;
type Point = util::vec2::Point2<i32>;

type Path = Vec<Point>;

fn read_paths() -> Vec<Path> {
    let mut result = Vec::new();

    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut path = Vec::new();
        for part in line.split(" -> ") {
            let coords_vec: Vec<i32> = part.split(",").map(|p| p.parse::<i32>().unwrap()).collect();
            path.push(Point {
                x: coords_vec[0],
                y: coords_vec[1],
            });
        }
        result.push(path);
    }
    return result;
}

fn initialize_rock_positions(paths: &Vec<Path>) -> HashSet<Point> {
    let mut result = HashSet::new();
    for p in paths {
        for i in 0..p.len() - 1 {
            let (prev, curr) = (p[i], p[i + 1]);

            for x in min(prev.x, curr.x)..=max(prev.x, curr.x) {
                for y in min(prev.y, curr.y)..=max(prev.y, curr.y) {
                    result.insert(Point { x: x, y: y });
                }
            }
        }
    }

    return result;
}

fn part_one() {
    let paths = read_paths();
    let mut taken = initialize_rock_positions(&paths);
    let rock_max_y = taken.iter().map(|pt| pt.y).max().unwrap();

    let src = Point { x: 500, y: 0 };
    let directions = [
        Point { x: 0, y: 1 },
        Point { x: -1, y: 1 },
        Point { x: 1, y: 1 },
    ];

    let mut accumulated_sand = 0;
    let mut flowing_out = false;
    loop {
        let mut sand_coords = src;
        loop {
            if sand_coords.y >= rock_max_y {
                flowing_out = true;
                break;
            }

            let mut moved = false;
            for dir in directions {
                let new_pos = sand_coords + dir;
                if !taken.contains(&new_pos) {
                    sand_coords = new_pos;
                    moved = true;
                    break;
                }
            }

            if !moved {
                taken.insert(sand_coords);
                accumulated_sand += 1;
                break;
            }
        }
        if flowing_out {
            break;
        }
    }

    println!("{accumulated_sand}");
}

fn part_two() {
    let paths = read_paths();
    let mut taken = initialize_rock_positions(&paths);
    let floor_y = taken.iter().map(|pt| pt.y).max().unwrap() + 2;

    let is_floor = |pos: Point| pos.y == floor_y;
    let src = Point { x: 500, y: 0 };

    let mut queue = VecDeque::new();
    let mut accumulated_sand = 0;

    queue.push_back(src);
    let directions = [
        Point { x: 0, y: 1 },
        Point { x: -1, y: 1 },
        Point { x: 1, y: 1 },
    ];
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        if is_floor(curr) {
            break;
        }

        accumulated_sand += 1;

        for dir in directions {
            let new_pos = curr + dir;

            if !taken.contains(&new_pos) {
                taken.insert(new_pos);
                queue.push_back(new_pos);
            }
        }
    }

    println!("{accumulated_sand}");
}

fn main() {
    part_two();
}
