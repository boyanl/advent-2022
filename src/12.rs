#!/usr/bin/env run-cargo-script
use std::{
    collections::{HashSet, VecDeque},
    io::{self, stdin},
};

type Vec2 = (i32, i32);

fn find_start_dest(maze: &Vec<String>) -> (Vec2, Vec2) {
    let mut start: Vec2 = (-1, -1);
    let mut end: Vec2 = (-1, -1);
    for (i, line) in maze.iter().enumerate() {
        match line.find("S") {
            Some(n) => start = (i as i32, n as i32),
            _ => (),
        }
        match line.find("E") {
            Some(n) => end = (i as i32, n as i32),
            _ => (),
        }
    }

    return (start, end);
}

fn height_ok(c1: char, c2: char) -> bool {
    let h1 = c1 as i32;
    let h2 = c2 as i32;
    return h1 >= h2 || h2 == h1 + 1;
}

fn part_one() {
    let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
    let (start, end) = find_start_dest(&lines);

    let n = lines.len();
    let m = lines[0].len();

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let mut result = -1;

    q.push_back((start, 0));
    while !q.is_empty() {
        let ((i, j), steps) = q.pop_front().unwrap();

        visited.insert((i, j));

        if (i, j) == end {
            result = steps;
            break;
        }

        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (ni, nj) = (i + di, j + dj);

            if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 && !visited.contains(&(ni, nj))
            {
                let origin_height = if (i, j) == start {
                    'a'
                } else {
                    lines[i as usize].chars().nth(j as usize).unwrap()
                };
                let dest_height = if (ni, nj) == end {
                    'z'
                } else {
                    lines[ni as usize].chars().nth(nj as usize).unwrap()
                };
                if height_ok(origin_height, dest_height) {
                    q.push_back(((ni, nj), steps + 1));
                    visited.insert((ni, nj));
                }
            }
        }
    }

    println!("{result}");
}

fn find_other_starting_positions(maze: &Vec<String>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for (i, line) in maze.iter().enumerate() {
        for (j, _) in line.match_indices("a") {
            result.push((i as i32, j as i32));
        }
    }
    return result;
}

fn part_two() {
    let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
    let (start, end) = find_start_dest(&lines);

    let n = lines.len();
    let m = lines[0].len();

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let mut result = -1;

    let other_starting = find_other_starting_positions(&lines);

    q.push_back((start, 0));
    for other in other_starting {
        q.push_back((other, 0));
    }
    while !q.is_empty() {
        let ((i, j), steps) = q.pop_front().unwrap();

        if (i, j) == end {
            result = steps;
            break;
        }

        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (ni, nj) = (i + di, j + dj);

            if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 && !visited.contains(&(ni, nj))
            {
                let origin_height = if (i, j) == start {
                    'a'
                } else {
                    lines[i as usize].chars().nth(j as usize).unwrap()
                };
                let dest_height = if (ni, nj) == end {
                    'z'
                } else {
                    lines[ni as usize].chars().nth(nj as usize).unwrap()
                };
                if height_ok(origin_height, dest_height) {
                    q.push_back(((ni, nj), steps + 1));
                    visited.insert((ni, nj));
                }
            }
        }
    }

    println!("{result}");
}

fn main() {
    part_two();
}
