#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! scanf = "1.2.1"
//! ```
extern crate scanf;
use scanf::sscanf;
use std::collections::HashMap;
use std::io::{self, stdin};
use std::thread::current;

fn is_command(line: &str) -> bool {
    return line.starts_with("$");
}

#[derive(Clone, Debug)]
enum Entry {
    Directory(String),
    File(String, i32),
}

fn read_directory_structure() -> HashMap<String, i64> {
    let mut dir_stack = Vec::new();
    let mut root = String::new();
    let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
    let mut i = 0;
    let mut contents: HashMap<String, Vec<Entry>> = HashMap::new();
    let mut sizes: HashMap<String, i64> = HashMap::new();

    while i < lines.len() {
        let line = &lines[i];
        if is_command(&line) {
            let command = &line[2..];
            let mut dir_name = String::new();
            if sscanf!(command, "cd {}", dir_name).is_ok() {
                if dir_name != ".." {
                    if dir_stack.is_empty() {
                        root = dir_name.clone();
                    }
                    dir_stack.push(dir_name);
                } else {
                    dir_stack.pop();
                }
            } else if command == "ls" {
                let current_dir = dir_stack.last().unwrap();
                while i + 1 < lines.len() && !is_command(&lines[i + 1]) {
                    let content = &lines[i + 1];
                    let mut file_name = String::new();
                    let d;
                    if sscanf!(content, "dir {}", file_name).is_ok() {
                        d = Entry::Directory(file_name.clone());
                    } else {
                        let mut size: i32 = 0;
                        sscanf!(content, "{} {}", size, file_name).unwrap();
                        d = Entry::File(file_name.clone(), size);

                        for i in 0..dir_stack.len() {
                            let joined = dir_stack[0..=i].join("/");
                            let full_path = if joined.starts_with("//") {
                                &joined[1..]
                            } else {
                                &joined[..]
                            };
                            sizes
                                .entry(String::from(full_path))
                                .and_modify(|v| *v += size as i64)
                                .or_insert(size as i64);
                        }
                    }
                    contents
                        .entry(current_dir.clone())
                        .and_modify(|v| v.push(d.clone()))
                        .or_insert(Vec::from([d]));
                    i += 1;
                }
            }
        }
        i += 1;
    }
    return sizes;
}

fn part_one() {
    let sizes = read_directory_structure();

    let result: i64 = sizes
        .iter()
        .filter(|(_, &v)| v <= 100000)
        .map(|(_, &v)| v)
        .sum();
    println!("{result}");
}

fn part_two() {
    let sizes = read_directory_structure();

    let total_used = sizes["/"];
    let currently_free = 70000000 - total_used;
    let to_free = 30000000 - currently_free;

    let result = sizes
        .iter()
        .filter(|(_, &v)| v >= to_free)
        .map(|(_, &v)| v)
        .min()
        .unwrap();
    println!("{result}");
}

fn main() {
    part_two();
}
