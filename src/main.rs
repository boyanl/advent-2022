use scanf::sscanf;
use std::io;

#[derive(Debug, Clone, Copy)]
struct Move {
    from: u32,
    to: u32,
    amount: usize,
}

type Stack = Vec<char>;

fn read_input() -> (Vec<Stack>, Vec<Move>) {
    let mut read_stacks = false;
    let mut moves = Vec::new();
    let mut stacks = Vec::new();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        if !read_stacks {
            for i in 0..(line.len() + 1) / 4 {
                let stack = line[i * 4..i * 4 + 3].trim();
                let mut c: char = ' ';
                if sscanf!(stack, "[{}]", c).is_ok() {
                    if stacks.len() <= i {
                        stacks.resize(i + 1, Vec::new());
                    }
                    stacks[i].insert(0, c);
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

    return (stacks, moves);
}

fn part_one() {
    let (mut stacks, moves) = read_input();
    for m in moves {
        for _ in 0..m.amount {
            let (from, to) = ((m.from - 1) as usize, (m.to - 1) as usize);
            let element = stacks[from].pop().unwrap();
            stacks[to].push(element);
        }
    }

    let mut result = String::new();
    for stack in &stacks {
        if stack.len() > 0 {
            result.push(*stack.last().unwrap())
        }
    }
    println!("{result}")
}

fn part_two() {
    let (mut stacks, moves) = read_input();

    for m in moves {
        let (from, to) = ((m.from - 1) as usize, (m.to - 1) as usize);

        let new_src_len = stacks[from].len() - m.amount;
        let new_dst_len = stacks[to].len() + m.amount;
        let dst = &mut stacks[to];
        dst.reserve(new_dst_len);
        for i in new_src_len..stacks[from].len() {
            let el = stacks[from][i];
            stacks[to].push(el);
        }
        stacks[from].truncate(new_src_len);
    }

    let mut result = String::new();
    for stack in &stacks {
        if stack.len() > 0 {
            result.push(*stack.last().unwrap())
        }
    }
    println!("{result}")
}

fn main() {
    part_two();
}
