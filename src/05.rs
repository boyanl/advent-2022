#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! scanf = "1.2.1"
//! ```
extern crate scanf;
use scanf::sscanf;
use std::io;

#[derive(Debug, Clone, Copy)]
struct Move {
    from: u32,
    to: u32,
    amount: usize,
}

type Stack = Vec<u8>;
type Stacks = [Stack; 9];

fn empty_stack() -> Stack {
    let mut result = Vec::new();
    result.reserve(1024 * 1024);
    return result;
}

fn read_input() -> (Stacks, usize, Vec<Move>) {
    let mut read_stacks = false;
    let mut moves = Vec::new();

    let mut stacks: [Stack; 9] = [
        empty_stack(),
        empty_stack(),
        empty_stack(),
        empty_stack(),
        empty_stack(),
        empty_stack(),
        empty_stack(),
        empty_stack(),
        empty_stack(),
    ];
    let mut stacks_cnt = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        if !read_stacks {
            for i in 0..(line.len() + 1) / 4 {
                let stack = line[i * 4..i * 4 + 3].trim();
                let mut c: char = ' ';
                if sscanf!(stack, "[{}]", c).is_ok() {
                    if stacks_cnt <= i {
                        stacks_cnt = i + 1;
                    }
                    stacks[i].push(c as u8);
                } else if stack.len() > 0 && stack.chars().nth(0).unwrap() == '1' {
                    read_stacks = true;
                    break;
                }
            }
        } else {
            let (mut from, mut to, mut amount) = (0, 0, 0);
            if sscanf!(&line, "move {} from {} to {}", amount, from, to).is_ok() {
                let m = Move {
                    from: from,
                    to: to,
                    amount: amount,
                };
                moves.push(m);
            }
        }
    }

    for i in 0..stacks_cnt {
        let len = stacks[i].len();
        for j in 0..len / 2 {
            let tmp = stacks[i][j];
            stacks[i][j] = stacks[i][len - j - 1];
            stacks[i][len - j - 1] = tmp;
        }
    }

    return (stacks, stacks_cnt, moves);
}

fn part_one() {
    let (mut stacks, stacks_cnt, moves) = read_input();
    for m in moves {
        for _ in 0..m.amount {
            let (from, to) = ((m.from - 1) as usize, (m.to - 1) as usize);
            let element = stacks[from].pop().unwrap();
            stacks[to].push(element);
        }
    }

    let mut result = String::new();
    for i in 0..stacks_cnt {
        if stacks[i].len() > 0 {
            result.push(stacks[i][stacks[i].len() - 1] as char);
        }
    }
    println!("{result}")
}

fn part_two() {
    let (mut stacks, stacks_cnt, moves) = read_input();

    for m in moves {
        let (from, to) = ((m.from - 1) as usize, (m.to - 1) as usize);

        let new_src_len = stacks[from].len() - m.amount;
        for i in new_src_len..stacks[from].len() {
            let el = stacks[from][i];
            stacks[to].push(el);
        }
        stacks[from].truncate(new_src_len);
    }

    let mut result = String::new();
    for i in 0..stacks_cnt {
        if stacks[i].len() > 0 {
            result.push(stacks[i][stacks[i].len() - 1] as char);
        }
    }
    println!("{result}")
}

fn main() {
    part_two();
}
