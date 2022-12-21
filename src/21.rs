#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! scanf = "1.2.1"
//! ```
extern crate scanf;
use self::JobType::{Number, ResultOf};
use scanf::sscanf;
use std::{collections::HashMap, io::stdin};

#[derive(Debug, Clone)]
enum JobType {
    Number(i64),
    ResultOf(char, String, String),
}

fn eval(monkeys: &HashMap<String, JobType>, monkey: &str) -> i64 {
    return match &monkeys[monkey] {
        Number(n) => *n as i64,
        ResultOf(op, m1, m2) => match op {
            '-' => eval(monkeys, m1.as_str()) - eval(monkeys, m2.as_str()),
            '+' => eval(monkeys, m1.as_str()) + eval(monkeys, m2.as_str()),
            '/' => eval(monkeys, m1.as_str()) / eval(monkeys, m2.as_str()),
            '*' => eval(monkeys, m1.as_str()) * eval(monkeys, m2.as_str()),
            _ => todo!(),
        },
    };
}

fn find_human(monkeys: &HashMap<String, JobType>, from_monkey: &str) -> bool {
    if from_monkey == "humn" {
        return true;
    }
    return match &monkeys[from_monkey] {
        Number(_) => false,
        ResultOf(_, m1, m2) => find_human(monkeys, m1.as_str()) || find_human(monkeys, m2.as_str()),
    };
}

fn find_value_for_equality(monkeys: &HashMap<String, JobType>) -> i64 {
    if let ResultOf(_, m1, m2) = &monkeys["root"] {
        let human_left = find_human(monkeys, m1.as_str());
        let target_monkey = if human_left { m2 } else { m1 };
        let other_monkey = if human_left { m1 } else { m2 };

        let target = eval(&monkeys, &target_monkey);
        return find_value_for_equality_internal(monkeys, other_monkey, target);
    }
    return 0;
}

fn find_value_for_equality_internal(
    monkeys: &HashMap<String, JobType>,
    root: &str,
    target: i64,
) -> i64 {
    if root == "humn" {
        return target;
    }
    match &monkeys[root] {
        Number(n) => {
            if *n == target {
                return target;
            } else {
                todo!();
            }
        }
        ResultOf(op, m1, m2) => {
            let human_left = find_human(&monkeys, m1.as_str());
            let other_val = if human_left {
                eval(monkeys, m2)
            } else {
                eval(monkeys, m1)
            };
            let next_target = match *op {
                '+' => target - other_val,
                '-' => {
                    if human_left {
                        target + other_val
                    } else {
                        other_val - target
                    }
                }
                '*' => target / other_val,
                '/' => {
                    if human_left {
                        other_val * target
                    } else {
                        other_val / target
                    }
                }
                _ => todo!(),
            };

            return find_value_for_equality_internal(
                monkeys,
                if human_left { m1 } else { m2 },
                next_target,
            );
        }
    }
}

fn parse_monkeys() -> HashMap<String, JobType> {
    let mut monkey_map = HashMap::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut monkey_name = String::new();
        let mut yells = String::new();
        if sscanf!(&line, "{}: {}", monkey_name, yells).is_ok() {
            let number_res = yells.parse::<i64>();
            if number_res.is_ok() {
                monkey_map.insert(monkey_name.clone(), Number(number_res.unwrap()));
            } else {
                let (mut monkey_1, mut monkey_2) = (String::new(), String::new());
                let mut op: char = ' ';
                if sscanf!(&yells, "{} {} {}", monkey_1, op, monkey_2).is_ok() {
                    monkey_map.insert(
                        monkey_name,
                        ResultOf(op, monkey_1.clone(), monkey_2.clone()),
                    );
                }
            }
        }
    }
    return monkey_map;
}

fn part_one() {
    let monkey_map = parse_monkeys();
    let result = eval(&monkey_map, "root");
    println!("{result}");
}

fn part_two() {
    let monkey_map = parse_monkeys();
    let result = find_value_for_equality(&monkey_map);
    println!("{result}");
}

fn main() {
    part_two();
}
