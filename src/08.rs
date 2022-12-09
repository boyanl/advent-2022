use std::io::{self, stdin};

fn in_bounds(pos: (i32, i32), heights: &Vec<Vec<u32>>) -> bool {
    let (i, j) = pos;
    return i >= 0 && i < heights.len() as i32 && j >= 0 && j < heights[i as usize].len() as i32;
}

fn check_visibility(pos: (usize, usize), dir: (i32, i32), heights: &Vec<Vec<u32>>) -> bool {
    let mut i = pos.0 as i32;
    let mut j = pos.1 as i32;

    let height = heights[i as usize][j as usize];

    let mut ok = true;
    loop {
        i += dir.0;
        j += dir.1;

        if !in_bounds((i, j), heights) {
            break;
        }
        ok &= heights[i as usize][j as usize] < height;
    }
    return ok;
}

fn is_visible(pos: (usize, usize), heights: &Vec<Vec<u32>>) -> bool {
    for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        if check_visibility(pos, dir, heights) {
            return true;
        }
    }
    return false;
}

fn visible_trees_in_dir(pos: (usize, usize), dir: (i32, i32), heights: &Vec<Vec<u32>>) -> u32 {
    let mut i = pos.0 as i32;
    let mut j = pos.1 as i32;

    let height = heights[i as usize][j as usize];

    let mut cnt = 0;
    loop {
        i += dir.0;
        j += dir.1;

        if !in_bounds((i, j), heights) {
            break;
        }
        cnt += 1;
        if heights[i as usize][j as usize] >= height {
            break;
        }
    }
    return cnt;
}

fn scenic_score(pos: (usize, usize), heights: &Vec<Vec<u32>>) -> u32 {
    return [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .map(|dir| visible_trees_in_dir(pos, dir, heights))
        .iter()
        .product();
}

fn part_one() {
    let mut heights = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let row: Vec<u32> = line.chars().map(|c| (c as u32) - ('0' as u32)).collect();
        heights.push(row);
    }

    let mut visible_cnt = 0;
    for i in 0..heights.len() {
        for j in 0..heights[i].len() {
            visible_cnt += if is_visible((i, j), &heights) { 1 } else { 0 }
        }
    }

    println!("{visible_cnt}")
}

fn part_two() {
    let mut heights = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let row: Vec<u32> = line.chars().map(|c| (c as u32) - ('0' as u32)).collect();
        heights.push(row);
    }

    let best_score = heights
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| scenic_score((i, j), &heights))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{best_score}");
}

fn main() {
    part_two();
}
