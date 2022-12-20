use std::collections::HashMap;
use std::io::{self, stdin};

fn mix(ns: &Vec<i64>, rounds: u32) -> Vec<i64> {
    let mut result = Vec::new();
    for (i, &x) in ns.iter().enumerate() {
        result.push((i, x));
    }
    let mut pos = HashMap::new();

    for _ in 0..rounds {
        for (i, &x) in ns.iter().enumerate() {
            let len = ns.len();
            let mod_l = (ns.len() - 1) as i64;
            let amount = ((x % mod_l + mod_l) % mod_l) as usize;

            let s = *pos.get(&(i, x)).unwrap_or(&i);
            for j in 1..=amount {
                let old_idx = (s + j) % len;
                let new_idx = (s + j - 1) % len;
                result[new_idx] = result[old_idx];
                pos.insert(result[old_idx], new_idx);
            }
            result[(s + amount) % len] = (i, x);
            pos.insert((i, x), (s + amount) % len);
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
