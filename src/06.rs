use std::collections::HashMap;
use std::io;

fn frequencies(s: &str) -> HashMap<char, u32> {
    let mut result = HashMap::new();
    for c in s.chars() {
        result.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    return result;
}

fn ok(freqs: &HashMap<char, u32>) -> bool {
    for (_, count) in freqs {
        if *count >= 2 {
            return false;
        }
    }
    return true;
}

fn find_first_distinct(line: &str, n: usize) -> usize {
    let mut initial = frequencies(&line[0..n]);
    if ok(&initial) {
        return n;
    }
    for i in n..line.len() {
        let to_remove = line.chars().nth(i - n).unwrap();
        let to_add = line.chars().nth(i).unwrap();

        initial.entry(to_remove).and_modify(|v| *v -= 1);
        initial.entry(to_add).and_modify(|v| *v += 1).or_insert(1);

        if ok(&initial) {
            return i + 1;
        }
    }
    return 0;
}

fn part_one() {
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            continue;
        }

        let result = find_first_distinct(&line, 4);
        println!("{result}")
    }
}

fn part_two() {
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            continue;
        }

        let result = find_first_distinct(&line, 14);
        println!("{result}")
    }
}

fn main() {
    part_two();
}
