use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use rustc_hash::{FxHashMap, FxHashSet};

mod util;
type Point = util::vec2::Point2<i32>;

const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
];

fn dirs_to_check(d: Point) -> [Point; 3] {
    if d.x != 0 {
        return [
            Point { x: d.x, y: -1 },
            Point { x: d.x, y: 0 },
            Point { x: d.x, y: 1 },
        ];
    }
    return [
        Point { x: -1, y: d.y },
        Point { x: 0, y: d.y },
        Point { x: 1, y: d.y },
    ];
}

fn all_adjacent(d: Point) -> Vec<Point> {
    let mut result = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx != 0 || dy != 0 {
                result.push(d + Point { x: dx, y: dy });
            }
        }
    }
    return result;
}

type State = FxHashSet<Point>;

fn next_state(state: &State, round: usize) -> State {
    let mut result = FxHashSet::default();

    let mut want = FxHashMap::default();
    let mut elves_to_move = Vec::new();
    let mut elves_remaining = Vec::new();
    for &elf in state {
        let mut have_adjacent = false;

        'outer: for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx != 0 || dy != 0)
                    && state.contains(&Point {
                        x: elf.x + dx,
                        y: elf.y + dy,
                    })
                {
                    have_adjacent = true;
                    break 'outer;
                }
            }
        }

        if !have_adjacent {
            elves_remaining.push(elf);
            continue;
        }

        let mut moved = false;
        for dir_idx in round..round + DIRECTIONS.len() {
            let dir = DIRECTIONS[dir_idx % DIRECTIONS.len()];

            let dest = elf + dir;
            let mut ok = true;

            for to_check in dirs_to_check(dir) {
                if state.contains(&(elf + to_check)) {
                    ok = false;
                }
            }

            if ok {
                moved = true;
                *want.entry(dest).or_insert(0) += 1;
                elves_to_move.push((elf, dir));
                break;
            }
        }
        if !moved {
            elves_remaining.push(elf);
        }
    }

    for (elf, dir) in elves_to_move {
        let dest = elf + dir;
        if want[&dest] == 1 {
            result.insert(dest);
        } else {
            result.insert(elf);
        }
    }
    for elf in elves_remaining {
        result.insert(elf);
    }

    return result;
}

fn empty_tiles(state: &State) -> i32 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for pt in state {
        min_x = min_x.min(pt.x);
        max_x = max_x.max(pt.x);
        min_y = min_y.min(pt.y);
        max_y = max_y.max(pt.y);
    }

    return (max_x - min_x + 1) * (max_y - min_y + 1) - (state.len() as i32);
}

fn read_elf_positions() -> State {
    let mut elves = FxHashSet::default();
    for (i, line) in stdin().lines().map(|l| l.unwrap()).enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Point {
                    x: j as i32,
                    y: i as i32,
                });
            }
        }
    }
    return elves;
}

fn part_one() {
    let mut elves = read_elf_positions();
    let rounds = 10;
    for i in 0..rounds {
        elves = next_state(&elves, i);
    }

    let result = empty_tiles(&elves);
    println!("{}", result);
}

fn part_two() {
    let mut elves = read_elf_positions();

    let mut result = 0;
    for i in 0.. {
        let new_state = next_state(&elves, i);
        if new_state == elves {
            result = i + 1;
            break;
        }
        elves = new_state;
    }

    println!("{}", result);
}

fn main() {
    part_two();
}
