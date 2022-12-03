use std::io::{self, BufRead};
use std::cmp::{max};

fn part_one() {
    let mut current_sum = 0;
    let mut max_sum = 0;
    for line_res in io::stdin().lock().lines() {
        let line = line_res.expect("failed to read line");
        if line.is_empty() {
            max_sum = max(max_sum, current_sum);
            current_sum = 0; 
            continue;
        }
        let calories:u32 = line.trim().parse().expect("need a number");
        current_sum += calories;
    }
    max_sum = max(max_sum, current_sum);
    println!("{max_sum}");
}


fn part_two() {
    let mut vec = Vec::new();
    let mut current_sum = 0;
    for line_res in io::stdin().lock().lines() {
        let line = line_res.expect("failed to read line");
        if line.is_empty() {
            vec.push(current_sum);
            current_sum = 0; 
            continue;
        }
        let calories:u32 = line.trim().parse().expect("need a number");
        current_sum += calories;
    }
    vec.push(current_sum);

    vec.sort_by(|a, b| b.cmp(a));

    let top3_sum:u32 = vec[0..3].iter().sum();
    println!("{top3_sum}")
}

fn main() {
    part_two();
}