#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! scanf = "1.2.1"
//! ```
extern crate scanf;
use scanf::sscanf;
use std::collections::HashMap;
use std::io::stdin;
use std::{cmp, vec};

fn set_bit(x: i64, n: usize) -> i64 {
    return x | (1 << n);
}

fn clear_bit(x: i64, n: usize) -> i64 {
    return x & !(1 << n);
}

fn has_bit_set(x: i64, n: usize) -> bool {
    return x & (1 << n) != 0;
}

fn get_set_positions(x: i64) -> Vec<usize> {
    let mut result = Vec::new();
    let mut curr = x;
    for i in 0..64 {
        if curr % 2 == 1 {
            result.push(i);
        }
        curr /= 2;
        if curr == 0 {
            break;
        }
    }
    return result;
}

type Valve = usize;
fn most_released(
    t: i32,
    current: Valve,
    remaining_valves: i64,
    flow_rates: &HashMap<Valve, i32>,
    distances: &Vec<Vec<i32>>,
    cache: &mut HashMap<(i32, Valve, i64), i32>,
) -> i32 {
    let key = (t, current, remaining_valves);
    if t <= 1 {
        return 0;
    }
    let cached = cache.get(&key);
    if cached.is_some() {
        return *cached.unwrap();
    }

    let mut max = 0;

    for next in get_set_positions(remaining_valves) {
        if distances[current][next] < t {
            let remaining_t = t - distances[current][next] - 1;
            let open_next = remaining_t * flow_rates[&next]
                + most_released(
                    remaining_t,
                    next,
                    clear_bit(remaining_valves, next),
                    flow_rates,
                    distances,
                    cache,
                );
            max = cmp::max(max, open_next);
        }
    }
    cache.entry(key).or_insert(max);
    return max;
}

fn most_released_with_elephant(
    t: i32,
    current: usize,
    remaining_valves: i64,
    flow_rates: &HashMap<usize, i32>,
    distances: &Vec<Vec<i32>>,
    cache1: &mut HashMap<(i32, usize, i64), i32>,
    cache2: &mut HashMap<(i32, usize, i64), i32>,
) -> i32 {
    let key = (t, current, remaining_valves);
    if t <= 1 {
        return 0;
    }
    let cached = cache1.get(&key);
    if cached.is_some() {
        return *cached.unwrap();
    }

    let mut max = most_released(26, 0, remaining_valves, flow_rates, distances, cache2);

    for next in get_set_positions(remaining_valves) {
        if distances[current][next] < t {
            let remaining_t = t - distances[current][next] - 1;
            let open_next = remaining_t * flow_rates[&next]
                + most_released_with_elephant(
                    remaining_t,
                    next,
                    clear_bit(remaining_valves, next),
                    flow_rates,
                    distances,
                    cache1,
                    cache2,
                );
            max = cmp::max(max, open_next);
        }
    }

    cache1.entry(key).or_insert(max);
    return max;
}

struct Input {
    valve_numbers: HashMap<String, usize>,
    neighbours: HashMap<usize, i64>,
    flow_rates: HashMap<usize, i32>,
}

fn read_input() -> Input {
    let mut valve_numbers: HashMap<String, usize> = HashMap::new();
    let mut valve_cnt = 0usize;
    let mut next_valve = || {
        let old = valve_cnt;
        valve_cnt += 1;
        return old;
    };

    let mut neighbours = HashMap::new();
    let mut flow_rates = HashMap::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut valve = String::new();
        let mut flow_rate = 0;
        let mut other_valves_str = String::new();

        if sscanf!(
            &line,
            "Valve {} has flow rate={i32}; tunnels lead to valves {}",
            valve,
            flow_rate,
            other_valves_str
        )
        .is_ok()
        {
            let valve_num = *valve_numbers.entry(valve).or_insert_with(|| next_valve());

            flow_rates.entry(valve_num).or_insert(flow_rate);
            for other in other_valves_str.split(", ") {
                let other_valve_num = *valve_numbers
                    .entry(other.to_string())
                    .or_insert_with(|| next_valve());
                let ns = neighbours.entry(valve_num).or_insert(0);
                *ns = set_bit(*ns, other_valve_num);
            }
        } else if sscanf!(
            &line,
            "Valve {} has flow rate={i32}; tunnel leads to valve {}",
            valve,
            flow_rate,
            other_valves_str
        )
        .is_ok()
        {
            let valve_num = *valve_numbers.entry(valve).or_insert_with(|| next_valve());

            flow_rates.entry(valve_num).or_insert(flow_rate);
            let other_valve_num = *valve_numbers
                .entry(other_valves_str.to_string())
                .or_insert_with(|| next_valve());

            let ns = neighbours.entry(valve_num).or_insert(0);
            *ns = set_bit(*ns, other_valve_num);
        }
    }

    return Input {
        valve_numbers: valve_numbers,
        neighbours: neighbours,
        flow_rates: flow_rates,
    };
}

fn part_one() {
    let inp = read_input();
    let mut cache = HashMap::new();
    let nonzero_valves = (0..inp.neighbours.len())
        .filter(|v| inp.flow_rates[v] > 0)
        .fold(0, |accum, item| set_bit(accum, item));
    let distances = compute_distances(&inp.neighbours);

    let result = most_released(
        30,
        inp.valve_numbers["AA"],
        nonzero_valves,
        &inp.flow_rates,
        &distances,
        &mut cache,
    );
    println!("{result}");
}

fn compute_distances(neighbours: &HashMap<usize, i64>) -> Vec<Vec<i32>> {
    let n = neighbours.len();
    let mut dist = vec![vec![std::i32::MAX / 3; n]; n];

    for (&v1, neighb) in neighbours {
        for v2 in get_set_positions(*neighb) {
            dist[v1][v2] = 1;
        }
        dist[v1 as usize][v1 as usize] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    return dist;
}

fn part_two() {
    let inp = read_input();
    let start = inp.valve_numbers["AA"];
    let distances = compute_distances(&inp.neighbours);
    let nonzero_valves = (0..inp.neighbours.len())
        .filter(|v| inp.flow_rates[v] > 0)
        .fold(0, |accum, item| set_bit(accum, item));

    let result = most_released_with_elephant(
        26,
        start,
        nonzero_valves,
        &inp.flow_rates,
        &distances,
        &mut HashMap::new(),
        &mut HashMap::new(),
    );
    println!("{result}");
}

fn main() {
    part_two();
}
