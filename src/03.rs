use std::hash::{BuildHasher, Hash};
use std::io;
use std::collections::HashSet;
use std::collections::hash_set::Intersection;


fn item_types(comparement: &str) -> HashSet<char> {
    let mut res  : HashSet<char> = HashSet::new();
    for c in comparement.chars() {
        res.insert(c);
    } 
    return res;
}

fn priority(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        return (c as u32) - ('a' as u32) + 1;
    }
    return (c as u32) - ('A' as u32) + 27;
}

fn part_one() {
    let mut priority_sum = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            continue;
        }
        let (compartment1, compartment2) = (&line[..line.len()/2], &line[line.len()/2..]);
        let items1 = item_types(compartment1);
        let items2 = item_types(compartment2);

        for common_item in items1.intersection(&items2) {
            priority_sum += priority(*common_item);
        }
    }
    
    println!("{priority_sum}")
}

fn into_hash_set<'a, T: Eq + Hash + Copy, S: BuildHasher>(isect: Intersection<'a, T, S>) -> HashSet<T> {
    let mut result = HashSet::new();
    for el in isect {
        result.insert(*el);
    }
    return result;
}

fn common_elements(strs: &[String]) -> HashSet<char> {
    return strs.iter().map(|l| item_types(l)).reduce(|acc, item| into_hash_set(acc.intersection(&item))).unwrap();
}


fn part_two() {
    let mut lines = Vec::new();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            continue;
        }
        lines.push(line);
    }
    
    let mut result = 0;
    for i in 0..lines.len()/3 {
       let group = &lines[i*3..i*3+3];
       result += common_elements(group).iter().map(|c| priority(*c)).sum::<u32>();
    }
    println!("{result}")
}

fn main() {
    part_two();
}
