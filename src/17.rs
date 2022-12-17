use std::{
    cmp,
    collections::{HashMap, HashSet},
    io::{self, stdin},
};

mod util;
type Point = util::vec2::Point2<i64>;

type Shape = Vec<Point>;

fn parse_shape(s: &Vec<String>) -> Shape {
    let mut result = Vec::new();
    for (i, line) in s.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                result.push(Point {
                    x: j as i64,
                    y: (s.len() - i - 1) as i64,
                });
            }
        }
    }

    return result;
}

fn to_dir(c: char) -> Point {
    match c {
        '>' => Point::from((1, 0)),
        '<' => Point::from((-1, 0)),
        _ => todo!(),
    }
}

fn visualize_state(taken: &HashSet<Point>, max_y: i32, width: i32) {
    for y in (0..=max_y as i64).rev() {
        for x in 0..width as i64 {
            if !taken.contains(&Point::from((x, y))) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn get_shapes() -> Vec<Shape> {
    return [
        r"####",
        r".#.
          ###
          .#.",
        r"..#
          ..#
          ###",
        r"#
          #
          #
          #",
        r"##
          ##",
    ]
    .iter()
    .map(|&shape_str| {
        shape_str
            .split("\n")
            .map(|line| line.trim().into())
            .collect()
    })
    .map(|shape| parse_shape(&shape))
    .collect();
}

fn read_directions() -> Vec<Point> {
    let mut directions_str = String::new();
    stdin()
        .read_line(&mut directions_str)
        .expect("should have directions");

    return directions_str[..directions_str.len() - 1]
        .chars()
        .map(|c| to_dir(c))
        .collect();
}

const WIDTH: i32 = 7;

type Heights = [i64; WIDTH as usize];
type State = (Heights, usize, usize);

fn relative_heights(s: &Heights) -> Heights {
    let min_el = *s.iter().min().unwrap();
    let mut result = s.clone();
    for v in result.iter_mut() {
        *v -= min_el;
    }
    return result;
}

fn simulate_tetris(directions: &Vec<Point>, rounds: u64) -> i64 {
    let shapes: Vec<Shape> = get_shapes();
    let (sx, sy) = (2, 3);

    let mut highest_y = -1i64;
    let mut offset_highest_y = 0i64;
    let mut height_at_rnd = HashMap::new();
    let mut highest_ys: Heights = [-1; WIDTH as usize];
    let mut taken = HashSet::new();
    let mut last_seen_at: HashMap<State, u64> = HashMap::new();
    let mut current_dir = 0;

    let mut check_for_cycles = true;

    let mut i = 0u64;
    while i < rounds {
        let shape_idx = (i % (shapes.len() as u64)) as usize;
        let mut new_shape = shapes[shape_idx].clone();
        let dy = highest_y + sy + 1;

        for pt in new_shape.iter_mut() {
            pt.x += sx;
            pt.y += dy;
        }

        loop {
            let dir = directions[current_dir];
            current_dir = (current_dir + 1) % directions.len();

            let mut can_move = true;
            for pt in &new_shape {
                let new_pos = *pt + dir;
                if new_pos.x >= (WIDTH as i64) || new_pos.x < 0 || taken.contains(&new_pos) {
                    can_move = false;
                }
            }

            if can_move {
                for j in 0..new_shape.len() {
                    new_shape[j] += dir;
                }
            }

            let mut can_fall = true;
            let down = Point { x: 0, y: -1 };
            for j in 0..new_shape.len() {
                let new_pos = new_shape[j] + down;
                if new_pos.y < 0 || taken.contains(&new_pos) {
                    can_fall = false;
                    break;
                }
            }

            if can_fall {
                for j in 0..new_shape.len() {
                    new_shape[j] += down;
                }
            } else {
                for j in 0..new_shape.len() {
                    taken.insert(new_shape[j]);
                    highest_y = cmp::max(highest_y, new_shape[j].y);
                    highest_ys[new_shape[j].x as usize] =
                        cmp::max(highest_ys[new_shape[j].x as usize], new_shape[j].y);
                }
                height_at_rnd.insert(i, highest_y);

                let highest_ys_rel = relative_heights(&highest_ys);
                let new_state = (highest_ys_rel, shape_idx, current_dir);
                if check_for_cycles && last_seen_at.contains_key(&new_state) {
                    let seen_at_rnd = last_seen_at[&new_state];
                    let height_diff = highest_y - height_at_rnd[&seen_at_rnd];
                    let period_len = i - seen_at_rnd;

                    let last_round = rounds - 1;
                    let repetitions = (last_round - i) / period_len;
                    offset_highest_y = (repetitions as i64) * height_diff;
                    i += repetitions * period_len;

                    check_for_cycles = false;
                } else {
                    last_seen_at.insert(new_state, i);
                }

                break;
            }
        }
        i += 1;
    }

    return highest_y + offset_highest_y + 1;
}

fn part_one() {
    let result = simulate_tetris(&read_directions(), 2022);
    println!("{}", result);
}

fn part_two() {
    let result = simulate_tetris(&read_directions(), 1000000000000);
    println!("{}", result);
}

fn main() {
    part_two();
}
