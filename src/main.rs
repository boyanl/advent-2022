use std::collections::HashMap;
use std::io::{self, stdin};

fn mix(ns: &Vec<i64>, rounds: u32) -> Vec<i64> {
    let mut result: Vec<(usize, i64)> = ns.iter().enumerate().map(|(i, &x)| (i, x)).collect();
    let mod_len = (ns.len() - 1) as i64;
    for _ in 0..rounds {
        for (i, &x) in ns.iter().enumerate() {
            let len = ns.len();
            let amount = x.rem_euclid(mod_len) as usize;
            let s = result.iter().position(|&el| el == (i, x)).unwrap();

            for j in 1..=amount {
                result[(s + j - 1) % len] = result[(s + j) % len];
            }
            result[(s + amount) % len] = (i, x);
        }
    }

    return result.iter().map(|(_, x)| *x).collect();
}

fn part_one() {
    let numbers: Vec<i64> = stdin()
        .lines()
        .map(|l| l.unwrap().parse().expect("need a number"))
        .collect();

    let mixed = mix(&numbers, 1);
    let zero_idx = mixed.iter().position(|&x| x == 0).unwrap();
    let len = mixed.len();

    let result: i64 = [1000, 2000, 3000]
        .iter()
        .map(|n| mixed[(zero_idx + n) % len] as i64)
        .sum();

    println!("{result}");
}

fn part_two() {
    let numbers: Vec<i64> = stdin()
        .lines()
        .map(|l| l.unwrap().parse().expect("need a number"))
        .collect();

    let key: i64 = 811589153;
    let encrypted = numbers.iter().map(|&x| x * key).collect();
    let mixed = mix(&encrypted, 10);

    let zero_idx = mixed.iter().position(|&x| x == 0).unwrap();
    let len = mixed.len();

    let result: i64 = [1000, 2000, 3000]
        .iter()
        .map(|n| mixed[(zero_idx + n) % len] as i64)
        .sum();

    println!("{result}");
}

fn main() {
    part_two();
}
