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

fn eval_override(monkeys: &HashMap<String, JobType>, monkey: &str, human_override: i64) -> f64 {
    if monkey == "humn" {
        return human_override as f64;
    }
    return match &monkeys[monkey] {
        Number(n) => *n as f64,
        ResultOf(op, m1, m2) => match op {
            '-' => {
                eval_override(monkeys, m1.as_str(), human_override)
                    - eval_override(monkeys, m2.as_str(), human_override)
            }
            '+' => {
                eval_override(monkeys, m1.as_str(), human_override)
                    + eval_override(monkeys, m2.as_str(), human_override)
            }
            '/' => {
                eval_override(monkeys, m1.as_str(), human_override)
                    / eval_override(monkeys, m2.as_str(), human_override)
            }
            '*' => {
                eval_override(monkeys, m1.as_str(), human_override)
                    * eval_override(monkeys, m2.as_str(), human_override)
            }
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

fn find_x_binary_search(monkeys: &HashMap<String, JobType>) -> i64 {
    if let ResultOf(_, m1, m2) = &monkeys["root"] {
        let human_left = find_human(monkeys, m1.as_str());
        let human_right = find_human(monkeys, m2.as_str());

        let target;
        let branch_with_human;
        if human_left {
            target = eval(&monkeys, m2);
            branch_with_human = m1;
        } else if human_right {
            target = eval(&monkeys, m1);
            branch_with_human = m2;
        } else {
            todo!();
        }

        let target = target as f64;
        let (mut lo, mut hi) = (i64::MIN / 100, i64::MAX / 100);
        let v1 = eval_override(monkeys, &branch_with_human, lo);
        let v2 = eval_override(monkeys, &branch_with_human, hi);
        let ascending = v1 < v2;

        while lo < hi {
            let x = (lo + hi) / 2;
            let guess = eval_override(&monkeys, &branch_with_human, x);
            if guess == target {
                return x;
            }
            if (guess < target && ascending) || (guess > target && !ascending) {
                lo = x + 1;
            } else if (guess > target && ascending) || (guess < target && !ascending) {
                hi = x;
            }
        }
        return hi;
    }
    todo!();
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
    let result = find_x_binary_search(&monkey_map);
    println!("{result}");
}

fn main() {
    part_two();
}
