#!/usr/bin/env run-cargo-script
use self::Element::{Int, List};
use std::cmp::{self, Ordering};
use std::io::stdin;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Element {
    Int(i32),
    List(Vec<Element>),
}

fn parse_value(val: &str) -> Element {
    let int_result = val.parse::<i32>();
    if int_result.is_ok() {
        return Int(int_result.unwrap());
    }
    let elements_str = &val[1..val.len() - 1];
    let mut elements = Vec::new();
    let mut brace_balance = 0;
    let mut curr_val = String::new();
    for c in elements_str.chars() {
        if c == ',' && brace_balance == 0 {
            elements.push(parse_value(&curr_val));
            curr_val = String::new();
            continue;
        }

        if c == '[' {
            brace_balance += 1;
        } else if c == ']' {
            brace_balance -= 1;
        }
        curr_val.push(c);
    }
    if !curr_val.is_empty() {
        elements.push(parse_value(&curr_val));
    }
    return List(elements);
}

fn cmp_list(l1: &Vec<Element>, l2: &Vec<Element>) -> Ordering {
    let size = cmp::min(l1.len(), l2.len());
    for i in 0..size {
        let cmp_val = cmp(&l1[i], &l2[i]);
        match cmp_val {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => continue,
        }
    }
    return l1.len().cmp(&l2.len());
}

fn cmp(e1: &Element, e2: &Element) -> Ordering {
    match e1 {
        Int(n1) => match e2 {
            Int(n2) => return n1.cmp(n2),
            List(els) => {
                return cmp_list(&vec![Int(*n1)], els);
            }
        },
        List(els1) => match e2 {
            Int(n2) => return cmp_list(els1, &vec![Int(*n2)]),
            List(els2) => return cmp_list(els1, els2),
        },
    }
}

fn part_one() {
    let mut idx = 0;
    let mut result = 0;

    let mut element1 = String::new();
    let mut element2;
    for line in stdin().lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            element1 = String::new();
            continue;
        }

        if element1.is_empty() {
            element1 = line;
            idx += 1;
        } else {
            element2 = line;

            let parsed_1 = parse_value(&element1);
            let parsed_2 = parse_value(&element2);

            if cmp(&parsed_1, &parsed_2) == Ordering::Less {
                result += idx;
            }
        }
    }
    println!("{result}");
}

fn part_two() {
    let mut elements = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            continue;
        }

        let parsed = parse_value(&line);
        elements.push(parsed);
    }
    let v1 = parse_value("[[2]]");
    let v2 = parse_value("[[6]]");
    elements.push(v1.clone());
    elements.push(v2.clone());

    elements.sort_by(|e1, e2| cmp(e1, e2));
    let idx1 = elements.iter().position(|e| *e == v1).unwrap() + 1;
    let idx2 = elements.iter().position(|e| *e == v2).unwrap() + 1;

    let result = idx1 * idx2;
    println!("{result}");
}

fn main() {
    part_two();
}
