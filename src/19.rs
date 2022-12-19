#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! scanf = "1.2.1"
//! ```
extern crate scanf;
use scanf::sscanf;
use std::cmp::max;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub};
use std::{
    collections::HashMap,
    io::{self, stdin},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Resources([i32; 4]);

fn zero() -> Resources {
    return Resources([0; 4]);
}

impl Resources {
    fn ore(&self) -> i32 {
        return self.0[0];
    }
    fn clay(&self) -> i32 {
        return self.0[1];
    }
    fn obsidian(&self) -> i32 {
        return self.0[2];
    }
    fn geode(&self) -> i32 {
        return self.0[3];
    }
}

impl Add for Resources {
    type Output = Resources;

    fn add(self, rhs: Resources) -> Self::Output {
        return Resources([
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3],
        ]);
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        return Resources([
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3],
        ]);
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Resources) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
        self[3] += rhs[3];
    }
}

impl Mul<i32> for Resources {
    type Output = Resources;
    fn mul(self, rhs: i32) -> Self::Output {
        return Resources([self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs]);
    }
}

impl Index<usize> for Resources {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl IndexMut<usize> for Resources {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

fn piecewise_max(r1: Resources, r2: Resources) -> Resources {
    return Resources([
        max(r1[0], r2[0]),
        max(r1[1], r2[1]),
        max(r1[2], r2[2]),
        max(r1[3], r2[3]),
    ]);
}

#[derive(Debug)]
struct Blueprint {
    number: i32,
    robot_costs: [Resources; 4],
}

fn parse_cost(cost_str: &str) -> Resources {
    let mut result: Resources = zero();
    for part in cost_str.split(" and ") {
        let mut amount: i32 = 0;
        let mut resource_type = String::new();
        if sscanf!(part, "{} {}", amount, resource_type).is_ok() {
            match resource_type.as_str() {
                "ore" => result[0] = amount,
                "clay" => result[1] = amount,
                "obsidian" => result[2] = amount,
                "geode" => result[3] = amount,
                _ => todo!(),
            }
        } else {
            todo!();
        }
    }
    return result;
}

fn read_blueprint() -> Option<Blueprint> {
    let mut line = String::new();
    let read = stdin().read_line(&mut line);
    if !read.is_ok() {
        return None;
    }

    let mut n = 0;
    let mut ore_cost = String::new();
    let mut clay_cost = String::new();
    let mut obsidian_cost = String::new();
    let mut geode_cost = String::new();

    if sscanf!(&line, "Blueprint {}: Each ore robot costs {}. Each clay robot costs {}. Each obsidian robot costs {}. Each geode robot costs {}.", n, ore_cost, clay_cost, obsidian_cost, geode_cost).is_ok() {
        return Some(Blueprint {
            number: n,
            robot_costs: [parse_cost(&ore_cost), parse_cost(&clay_cost), parse_cost(&obsidian_cost), parse_cost(&geode_cost)]
        });
    } else {
            return None;
    }
}

fn read_blueprints() -> Vec<Blueprint> {
    let mut result = Vec::new();
    loop {
        let blueprint = read_blueprint();
        if blueprint.is_none() {
            break;
        }
        result.push(blueprint.unwrap());
    }
    return result;
}

fn blueprint_quality(b: &Blueprint, t: i32) -> i32 {
    return b.number * max_geodes(&b, 24);
}

fn max_geodes(b: &Blueprint, t: i32) -> i32 {
    let initial_resources = zero();
    let initial_income = Resources([1, 0, 0, 0]);
    let max_requirements = b
        .robot_costs
        .iter()
        .copied()
        .reduce(|cost1, cost2| piecewise_max(cost1, cost2))
        .unwrap();

    let mut cache = HashMap::new();
    return max_geodes_internal(
        b,
        t,
        initial_resources,
        initial_income,
        &max_requirements,
        0,
        &mut cache,
    );
}

fn max_geodes_internal(
    b: &Blueprint,
    t: i32,
    resources: Resources,
    income: Resources,
    max_costs: &Resources,
    best_so_far: i32,
    cache: &mut HashMap<(i32, Resources, Resources), i32>,
) -> i32 {
    if t == 0 {
        return resources.geode();
    }
    let key = (t, resources, income);
    if cache.contains_key(&key) {
        return cache[&key];
    }

    let mut result = resources.geode() + t * income.geode();
    let mut current_best = best_so_far;
    for i in 0..4 {
        let next_robot_cost = b.robot_costs[i];
        let mut robot_income = zero();
        robot_income[i] = 1;

        if i != 3 && max_costs[i] <= income[i] {
            // no point in trying to get more robots for this resource (except for geode)
            // already have more than (or equal to) what we can spend each turn
            continue;
        }

        let need = next_robot_cost - resources;
        let mut can_buy = true;
        let mut turns_required = 0;
        for j in 0..4 {
            let (need_r, income_r) = (need[j], income[j]);
            if need_r > 0 && income_r == 0 {
                can_buy = false;
                break;
            }
            if need_r <= 0 {
                continue;
            }
            turns_required = max(
                turns_required,
                need_r / income_r + (need_r % income_r != 0) as i32,
            );
        }

        if !can_buy {
            continue;
        }

        if t >= turns_required + 1 {
            // Score estimate is done by assuming we can buy a geode robot for each turn until we run out of time
            let score_estimate = resources.geode() + income.geode() * t + (t * (t - 1)) / 2;
            if score_estimate <= best_so_far {
                continue;
            }

            result = max(
                result,
                max_geodes_internal(
                    b,
                    t - turns_required - 1,
                    resources + income * (turns_required + 1) - next_robot_cost,
                    income + robot_income,
                    max_costs,
                    current_best,
                    cache,
                ),
            );
            current_best = max(current_best, result);
        }
    }

    cache.insert(key, result);
    return result;
}

fn part_one() {
    let blueprints = read_blueprints();
    let available_time = 24;

    let mut result = 0;
    for blueprint in blueprints {
        result += blueprint_quality(&blueprint, available_time);
    }

    println!("{result}");
}

fn part_two() {
    let blueprints = read_blueprints();
    let available_time = 32;

    let result: i32 = blueprints[0..3]
        .iter()
        .map(|b| max_geodes(b, available_time))
        .product();

    println!("{result}");
}

fn main() {
    part_two();
}
