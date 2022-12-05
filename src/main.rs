use scanf::sscanf;
use std::io;

#[derive(Debug, Clone, Copy)]
struct Move {
    from: u32,
    to: u32,
    amount: usize,
}

struct Stack {
    data: Vec<u8>,
    top: usize,
}

fn empty_stack() -> Stack {
    return Stack {
        data: vec![0u8; 1024 * 1024],
        top: 0,
    };
}

fn push(s: &mut Stack, val: u8) {
    s.data[s.top] = val;
    s.top += 1;
}

fn pop(s: &mut Stack) -> u8 {
    let result = s.data[s.top - 1];
    s.top -= 1;
    return result;
}

type Stacks = [Stack; 9];

fn print_stack(s: &Stack) {
    for i in 0..s.top {
        println!("{}", s.data[i] as char);
    }
}

fn read_input() -> (Stacks, usize, Vec<Move>) {
    let mut read_stacks = false;
    let mut moves = Vec::new();

    let mut stacks: Stacks = [
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
                    push(&mut stacks[i], c as u8);
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
        let len = stacks[i].top;
        for j in 0..len / 2 {
            let tmp = stacks[i].data[j];
            stacks[i].data[j] = stacks[i].data[len - j - 1];
            stacks[i].data[len - j - 1] = tmp;
        }
    }

    return (stacks, stacks_cnt, moves);
}

fn part_one() {
    let (mut stacks, stacks_cnt, moves) = read_input();
    for m in moves {
        for _ in 0..m.amount {
            let (from, to) = ((m.from - 1) as usize, (m.to - 1) as usize);
            let element = pop(&mut stacks[from]);
            push(&mut stacks[to], element);
        }
    }

    let mut result = String::new();
    for i in 0..stacks_cnt {
        if stacks[i].top > 0 {
            result.push(stacks[i].data[stacks[i].top - 1] as char);
        }
    }
    println!("{result}")
}

fn part_two() {
    let (mut stacks, stacks_cnt, moves) = read_input();

    for m in moves {
        let (from, to) = ((m.from - 1) as usize, (m.to - 1) as usize);

        let new_src_len = stacks[from].top - m.amount;
        for i in new_src_len..stacks[from].top {
            let el = stacks[from].data[i];
            push(&mut stacks[to], el);
        }
        stacks[from].top = new_src_len;
    }

    let mut result = String::new();
    for i in 0..stacks_cnt {
        if stacks[i].top > 0 {
            result.push(stacks[i].data[stacks[i].top - 1] as char);
        }
    }
    println!("{result}")
}

fn main() {
    part_two();
}
