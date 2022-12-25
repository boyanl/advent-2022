use gcd::Gcd;
use std::{
    collections::{BinaryHeap, HashSet},
    io::stdin,
    time::Instant,
};

mod util;
type Point = util::vec2::Point2<i32>;
const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const RIGHT: Point = Point { x: 1, y: 0 };
const LEFT: Point = Point { x: -1, y: 0 };

struct Blizzard {
    pos: Point,
    dir: Point,
}

fn direction(blizz_sign: char) -> Point {
    return match blizz_sign {
        '^' => UP,
        'v' => DOWN,
        '>' => RIGHT,
        '<' => LEFT,
        _ => todo!(),
    };
}

fn read_input() -> (Vec<Blizzard>, usize, usize) {
    let mut blizzards = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    for (i, line) in lines.iter().enumerate() {
        if i == 0 || i == lines.len() - 1 {
            width = line.len();
            height = lines.len();
            continue;
        } else {
            for (j, c) in line[..line.len() - 1].chars().enumerate() {
                match c {
                    'v' | '^' | '>' | '<' => blizzards.push(Blizzard {
                        pos: Point {
                            x: j as i32,
                            y: i as i32,
                        },
                        dir: direction(c),
                    }),
                    '#' | '.' => continue,
                    _ => todo!("{}", c),
                }
            }
        }
    }

    return (blizzards, width, height);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Point,
    t: usize,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct StateEstimate {
    state: State,
    estimate: usize,
}

impl Ord for StateEstimate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.estimate.cmp(&self.estimate);
    }
}

impl PartialOrd for StateEstimate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_blizzard_positions(
    blizzards: &Vec<Blizzard>,
    w: usize,
    h: usize,
) -> (Vec<HashSet<Point>>, usize) {
    let n = w - 2;
    let m = h - 2;
    let period = (n * m) / n.gcd(m);
    let mut blizzard_positions = Vec::new();

    for t in 0..period {
        let mut pos_at_t = HashSet::new();
        for blizz in blizzards {
            let mut blizz_pos = blizz.pos + blizz.dir * t as i32;
            blizz_pos.x = (blizz_pos.x - 1).rem_euclid(n as i32) + 1;
            blizz_pos.y = (blizz_pos.y - 1).rem_euclid(m as i32) + 1;

            pos_at_t.insert(blizz_pos);
        }
        blizzard_positions.push(pos_at_t);
    }

    return (blizzard_positions, period);
}

fn distance(
    blizzards_at_time: &Vec<HashSet<Point>>,
    period: usize,
    w: usize,
    h: usize,
    start: Point,
    end: Point,
    start_t: usize,
) -> usize {
    let start_state = State {
        pos: start,
        t: start_t,
    };
    let mut q = BinaryHeap::new();
    let mut seen = HashSet::new();
    q.push(StateEstimate {
        state: start_state,
        estimate: 0, // initial estimate is wrong but it doesn't really matter
    });

    while !q.is_empty() {
        let state_estimate = q.pop().unwrap();
        let state = state_estimate.state;
        if state.pos == end {
            return state.t;
        }
        // println!("At pos: {:?}, time: {}", state.pos, state.t);
        let blizzards = &blizzards_at_time[(state.t + 1) % period];

        for dir in [UP, DOWN, LEFT, RIGHT] {
            let new_pos = state.pos + dir;
            let new_t = state.t + 1;
            if new_pos.x >= 1
                && new_pos.x < (w - 1) as i32
                && (new_pos.y >= 1 || (new_pos.y == 0 && new_pos.x == 1))
                && (new_pos.y < (h - 1) as i32
                    || (new_pos.x == (w - 2) as i32 && new_pos.y == (h - 1) as i32))
                && !blizzards.contains(&new_pos)
                && !seen.contains(&State {
                    pos: new_pos,
                    t: new_t % period,
                })
            {
                let remaining_estimate = (new_pos.x - end.x).abs() + (new_pos.y - end.y).abs();
                q.push(StateEstimate {
                    estimate: state.t + remaining_estimate as usize,
                    state: State {
                        pos: new_pos,
                        t: new_t,
                    },
                });
                seen.insert(State {
                    pos: new_pos,
                    t: new_t % period,
                });
            }
        }
        // add state if we just wait
        if !blizzards.contains(&state.pos)
            && !seen.contains(&State {
                pos: state.pos,
                t: (state.t + 1) % period,
            })
        {
            let remaining_estimate = (state.pos.x - end.x).abs() + (state.pos.y - end.y).abs();
            q.push(StateEstimate {
                estimate: state.t + remaining_estimate as usize,
                state: State {
                    pos: state.pos,
                    t: state.t + 1,
                },
            });
            seen.insert(State {
                pos: state.pos,
                t: (state.t + 1) % period,
            });
        }
    }

    return usize::MAX;
}

fn part_one() {
    let (blizzards, width, height) = read_input();
    let (blizzards_at_time, period) = calculate_blizzard_positions(&blizzards, width, height);

    let start = Point { x: 1, y: 0 };
    let end = Point {
        x: (width - 2) as i32,
        y: (height - 1) as i32,
    };

    let result = distance(&blizzards_at_time, period, width, height, start, end, 0);
    println!("{}", result);
}

fn part_two() {
    let (blizzards, width, height) = read_input();
    let (blizzards_at_time, period) = calculate_blizzard_positions(&blizzards, width, height);

    let start = Point { x: 1, y: 0 };
    let end = Point {
        x: (width - 2) as i32,
        y: (height - 1) as i32,
    };

    let t1 = distance(&blizzards_at_time, period, width, height, start, end, 0);
    let t2 = distance(&blizzards_at_time, period, width, height, end, start, t1);
    let t3 = distance(&blizzards_at_time, period, width, height, start, end, t2);
    println!("{}", t3);
}

fn main() {
    part_two();
}
