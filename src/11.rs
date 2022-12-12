#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! scanf = "1.2.1"
//! ```
extern crate scanf;
use scanf::sscanf;
use std::io::{stdin, Read};

type ItemType = i64;
struct Monkey {
    items: Vec<ItemType>,
    op: Box<dyn Fn(ItemType) -> ItemType>,
    test_divisor: i32,
    pass_if_true: usize,
    pass_if_false: usize,
}

fn read_line_trimmed() -> String {
    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    return result.trim().to_string();
}

fn make_fn(op: &str, other: &str) -> Box<dyn Fn(ItemType) -> ItemType> {
    let operator: fn(ItemType, ItemType) -> ItemType = match op {
        "*" => |x, y| x * y,
        "+" => |x, y| x + y,
        "-" => |x, y| x - y,
        "/" => |x, y| x / y,
        _ => todo!(),
    };

    if other == "old" {
        return Box::new(move |x| operator(x, x));
    } else {
        let y: ItemType = other.parse().unwrap();
        return Box::new(move |x| operator(x, y));
    }
}

fn read_monkeys() -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    loop {
        let mut line = String::new();
        let read = stdin().read_line(&mut line).unwrap();
        if read == 0 {
            break;
        }
        if line.is_empty() {
            continue;
        }

        let mut monkey_idx = 0;
        if sscanf!(&line, "Monkey {}:", monkey_idx).is_ok() {
            let mut items_str = String::new();
            sscanf!(&read_line_trimmed(), "Starting items: {}", items_str)
                .expect("should have items");

            let items: Vec<ItemType> = items_str
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect();

            let mut op = String::new();
            let mut other = String::new();
            sscanf!(
                &read_line_trimmed(),
                "Operation: new = old {} {}",
                op,
                other
            )
            .expect("should have operation");

            let operation_fn = make_fn(&op, &other);

            let mut divisor: i32 = 0;
            sscanf!(&read_line_trimmed(), "Test: divisible by {}", divisor)
                .expect("should have test");

            let mut dest_monkey_1: usize = 0;
            sscanf!(
                &read_line_trimmed(),
                "If true: throw to monkey {}",
                dest_monkey_1
            )
            .expect("should have if-clause");

            let mut dest_monkey_2: usize = 0;
            sscanf!(
                &read_line_trimmed(),
                "If false: throw to monkey {}",
                dest_monkey_2
            )
            .expect("should have else-clause");

            monkeys.push(Monkey {
                items: items,
                op: operation_fn,
                test_divisor: divisor,
                pass_if_true: dest_monkey_1,
                pass_if_false: dest_monkey_2,
            });
        }
    }
    return monkeys;
}

fn part_one() {
    let monkeys = read_monkeys();

    let rounds = 20;
    let mut items: Vec<Vec<ItemType>> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspection_counts = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in items[i].clone() {
                let new_item = (monkey.op)(item) / 3;
                if new_item % (monkey.test_divisor as ItemType) == 0 {
                    items[monkey.pass_if_true as usize].push(new_item);
                } else {
                    items[monkey.pass_if_false as usize].push(new_item);
                }
                inspection_counts[i] += 1;
            }
            items[i].clear();
        }
    }
    inspection_counts.sort_unstable_by(|a, b| b.cmp(a));

    let result: i32 = inspection_counts[..2].iter().product();
    println!("{result}");
}

fn part_two() {
    let monkeys = read_monkeys();

    let rounds = 10000;
    let mut items: Vec<Vec<ItemType>> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspection_counts = vec![0; monkeys.len()];
    let modulo: i64 = monkeys.iter().map(|m| m.test_divisor as i64).product();

    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in items[i].clone() {
                let new_item = (monkey.op)(item) % modulo;
                if new_item % (monkey.test_divisor as ItemType) == 0 {
                    items[monkey.pass_if_true as usize].push(new_item);
                } else {
                    items[monkey.pass_if_false as usize].push(new_item);
                }
                inspection_counts[i] += 1;
            }
            items[i].clear();
        }
    }
    inspection_counts.sort_unstable_by(|a, b| b.cmp(a));

    let result: i64 = inspection_counts[..2].iter().product();
    println!("{result}");
}

fn main() {
    part_two();
}
