#!/usr/bin/env run-cargo-script
use std::io::{self, stdin};

fn mix(ns: &Vec<i64>, rounds: u32) -> Vec<i64> {
    let mut result = ns
        .iter()
        .enumerate()
        .map(|(i, &x)| (i, x))
        .collect::<Vec<_>>();
    for _ in 0..rounds {
        for (i, &x) in ns.iter().enumerate() {
            let element_pos = result.iter().position(|&el| el == (i, x)).unwrap();
            let el = result.remove(element_pos);
            let amount = x.rem_euclid(result.len() as i64) as usize;

            result.insert((element_pos + amount) % result.len(), el);
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
