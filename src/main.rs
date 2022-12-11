use scanf::sscanf;
use std::io::{stdin, Read};

struct Monkey {
    items: Vec<i32>,
    op: Box<dyn Fn(i32) -> i32>,
    test_divisor: i32,
    pass_if_true: i32,
    pass_if_false: i32,
}

fn read_line_trimmed() -> String {
    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    return result.trim().to_string();
}

fn make_fn(op: &str, other: &str) -> Box<dyn Fn(i32) -> i32> {
    let operator: fn(i32, i32) -> i32 = match op {
        "*" => |x: i32, y: i32| x * y,
        "+" => |x: i32, y: i32| x + y,
        "-" => |x: i32, y: i32| x - y,
        "/" => |x: i32, y: i32| x / y,
        _ => todo!(),
    };

    if other == "old" {
        return Box::new(move |x| operator(x, x));
    } else {
        let y: i32 = other.parse().unwrap();
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

            let items: Vec<i32> = items_str
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

            let mut dest_monkey_1: i32 = 0;
            sscanf!(
                &read_line_trimmed(),
                "If true: throw to monkey {i32}",
                dest_monkey_1
            )
            .expect("should have if-clause");

            let mut dest_monkey_2: i32 = 0;
            sscanf!(
                &read_line_trimmed(),
                "If false: throw to monkey {i32}",
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
    let mut items: Vec<Vec<i32>> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspection_counts = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in items[i].clone() {
                let new_item = (monkey.op)(item) / 3;
                if new_item % monkey.test_divisor == 0 {
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

// Each monkey keeps track of the values for items each other monkey has, modulo its test_divisor
type Items = Vec<Vec<i32>>;

fn part_two() {
    let monkeys = read_monkeys();

    let rounds = 10000;
    let mut items: Vec<Items> = Vec::new();
    for monkey in &monkeys {
        let items_as_viewed_by_monkey = monkeys
            .iter()
            .map(|other_monkey| {
                other_monkey
                    .items
                    .iter()
                    .map(|&x| x % monkey.test_divisor)
                    .collect()
            })
            .collect();
        items.push(items_as_viewed_by_monkey);
    }

    let pass_item =
        |items: &mut Vec<Items>, from_monkey: usize, to_monkey: usize, item_idx: usize| {
            let monkey = &monkeys[from_monkey];
            for (j, monkey_items_view) in items.iter_mut().enumerate() {
                let new_item =
                    (monkey.op)(monkey_items_view[from_monkey][item_idx]) % monkeys[j].test_divisor;
                monkey_items_view[to_monkey].push(new_item);
            }
        };
    let mut inspection_counts = vec![0i64; monkeys.len()];

    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            for (item_idx, &item) in items[i][i].clone().iter().enumerate() {
                let new_item = (monkey.op)(item) % monkey.test_divisor;
                if new_item % monkey.test_divisor == 0 {
                    pass_item(&mut items, i, monkey.pass_if_true as usize, item_idx);
                } else {
                    pass_item(&mut items, i, monkey.pass_if_false as usize, item_idx);
                }
                inspection_counts[i] += 1;
            }
            for monkey_items_view in items.iter_mut() {
                monkey_items_view[i].clear();
            }
        }
    }

    inspection_counts.sort_unstable_by(|a, b| b.cmp(a));
    let result: i64 = inspection_counts[..2].iter().product();
    println!("{result}");
}

fn main() {
    part_two();
}
