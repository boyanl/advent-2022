use scanf::sscanf;
use std::cmp::{max, min};
use std::io;

#[derive(Clone, Copy)]
struct Range {
    low: i32,
    high: i32,
}

fn contains(r1: Range, r2: Range) -> bool {
    return r1.low <= r2.low && r1.high >= r2.high;
}

fn intersects(r1: Range, r2: Range) -> bool {
    let low = max(r1.low, r2.low);
    let high = min(r1.high, r2.high);
    return low <= high;
}

fn part_one() {
    let mut fully_overlapping = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let (mut low1, mut high1, mut low2, mut high2) = (0, 0, 0, 0);
        if sscanf!(&line, "{}-{},{}-{}", low1, high1, low2, high2).is_ok() {
            let range1 = Range {
                low: low1,
                high: high1,
            };
            let range2 = Range {
                low: low2,
                high: high2,
            };

            if contains(range1, range2) || contains(range2, range1) {
                fully_overlapping += 1;
            }
        }
    }

    println!("{fully_overlapping}")
}

fn part_two() {
    let mut overlapping_count = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let (mut low1, mut high1, mut low2, mut high2) = (0, 0, 0, 0);
        if sscanf!(&line, "{}-{},{}-{}", low1, high1, low2, high2).is_ok() {
            let range1 = Range {
                low: low1,
                high: high1,
            };
            let range2 = Range {
                low: low2,
                high: high2,
            };

            if intersects(range1, range2) {
                overlapping_count += 1;
            }
        }
    }

    println!("{overlapping_count}")
}

fn main() {
    part_two();
}
