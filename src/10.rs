use self::Op::{Addx, Noop};
use std::io::{self, stdin};

#[derive(Debug)]
enum Op {
    Noop,
    Addx(i32),
}

fn read_ops() -> Vec<Op> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let args: Vec<&str> = line.split_ascii_whitespace().collect();

        let op = match args[0] {
            "noop" => Noop,
            "addx" => {
                let y: i32 = args[1].parse().unwrap();
                Addx(y)
            }
            _ => todo!("unexpected instruction type"),
        };
        result.push(op);
    }
    return result;
}

fn part_one() {
    let cycles_of_interest = [20, 60, 100, 140, 180, 220];
    let ops = read_ops();

    let mut cycle_number = 1;
    let mut x = 1;
    let mut total_signal_strength = 0;

    for op in ops {
        let old_x = x;
        let cycles_cnt;
        match op {
            Noop => cycles_cnt = 1,
            Addx(y) => {
                cycles_cnt = 2;
                x += y;
            }
        }
        let r = cycle_number..(cycle_number + cycles_cnt);
        for cycle in cycles_of_interest {
            if r.contains(&cycle) {
                total_signal_strength += cycle * old_x;
                break;
            }
        }
        cycle_number += cycles_cnt;
    }

    println!("{total_signal_strength}");
}

fn part_two() {
    let ops = read_ops();

    let mut x = 1;
    let mut curr_pixel = 0;

    let mut output = Vec::new();
    let mut line = String::new();

    for op in ops {
        let cycles_cnt;
        let new_x = match op {
            Noop => {
                cycles_cnt = 1;
                x
            }
            Addx(y) => {
                cycles_cnt = 2;
                x + y
            }
        };

        for _ in 0..cycles_cnt {
            let to_draw = if (x - 1..=x + 1).contains(&curr_pixel) {
                '█'
            } else {
                ' '
            };

            line.push(to_draw);
            curr_pixel += 1;

            if curr_pixel >= 40 {
                output.push(line);

                line = String::new();
                curr_pixel = 0;
            }
        }

        x = new_x;
    }

    for line in output {
        println!("{line}");
    }
}

fn main() {
    part_two();
}
