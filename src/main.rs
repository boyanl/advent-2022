use std::{io::{self, stdin}, collections::HashMap};
use std::ops::{Add, Sub, AddAssign, Mul};
use scanf::sscanf;
use std::cmp::max;


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Resources {
    pub ore: i32,
    pub clay: i32,
    pub obsidian: i32,
    pub geode: i32,
}


impl Add for Resources {
    type Output = Resources;

    fn add(self, rhs: Resources) -> Self::Output {
        return Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode
        };
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        return Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode
        };
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Resources) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl Mul<i32> for Resources {
    type Output = Resources;
    fn mul(self, rhs: i32) -> Self::Output {
        return Resources { ore: self.ore * rhs, clay: self.clay * rhs, obsidian: self.obsidian * rhs, geode: self.geode * rhs } ;
    }
}



#[derive(Debug)]
struct Blueprint {
    number: i32,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

fn parse_cost(cost_str: &str) -> Resources {
    let mut result: Resources = Resources { ore: 0, clay: 0, obsidian: 0, geode: 0 };
    for part in cost_str.split(" and ") {
        let mut amount: i32 = 0;
        let mut resource_type = String::new();
        if sscanf!(part, "{} {}", amount, resource_type).is_ok() {
            match resource_type.as_str() {
                "ore" => result.ore = amount,
                "clay" => result.clay = amount,
                "obsidian" => result.obsidian = amount,
                "geode" => result.geode = amount,
                _ => todo!()
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
            ore_robot_cost: parse_cost(&ore_cost),
            clay_robot_cost: parse_cost(&clay_cost),
            obsidian_robot_cost: parse_cost(&obsidian_cost),
            geode_robot_cost: parse_cost(&geode_cost)
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
    let initial_resources = Resources{ ore: 0, clay: 0, obsidian: 0, geode: 0 };
    let initial_income = Resources { ore: 1, clay: 0, obsidian: 0, geode: 0 };
    return b.number * max_geodes(&b, 24, initial_resources, initial_income, &mut HashMap::new());
}

fn max_geodes(b: &Blueprint, t: i32, resources: Resources, income: Resources, cache: &mut HashMap<(i32, Resources, Resources), i32>) -> i32 {
    if t == 0 {
        return resources.geode;
    }
    let key = (t, resources, income);
    if cache.contains_key(&key) {
        return cache[&key];
    }
    
    let mut result = resources.geode + t * income.geode;
    for (next_robot_cost, robot_income) in [(b.ore_robot_cost, Resources{ ore: 1, clay: 0, obsidian: 0, geode: 0}), (b.clay_robot_cost, Resources{ ore: 0, clay: 1, obsidian: 0, geode: 0}),
     (b.obsidian_robot_cost, Resources{ ore: 0, clay: 0, obsidian: 1, geode: 0}), (b.geode_robot_cost, Resources{ ore: 0, clay: 0, obsidian: 0, geode: 1})] {
       let need = next_robot_cost - resources; 
       let mut can_buy = true;
       let mut turns_required = 0;
       for (r, income_r) in [(need.ore, income.ore), (need.clay, income.clay), (need.obsidian, income.obsidian)] {
        if r > 0 && income_r == 0 {
            can_buy = false;
            break;
        }
        if r <= 0 {
            continue;
        }
        turns_required = max(turns_required, r / income_r + (r%income_r != 0) as i32);
       }
       
    //    println!("Turns required for robot cost: {:?} = {}, resources: {:?}, income = {:?}", next_robot_cost, turns_required, resources, income);
       
       if !can_buy {
        continue;
       }
       
    //    println!("Turns required: {}, next robot cost: {:?}", turns_required, next_robot_cost);
       
       if t >= turns_required + 1 {
       result = max(result, max_geodes(b, t - turns_required - 1, resources + income * (turns_required + 1) - next_robot_cost, income + robot_income, cache))
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
   
   let mut result = 0;
   for blueprint in blueprints {
    let initial_resources = Resources{ ore: 0, clay: 0, obsidian: 0, geode: 0 };
    let initial_income = Resources { ore: 1, clay: 0, obsidian: 0, geode: 0 };
       result = max(result, max_geodes(&blueprint, available_time, initial_resources, initial_income, &mut HashMap::new()));
       println!("Result: {result}");
   }
   
   println!("{result}");
    
}

fn main() {
    part_two();
}