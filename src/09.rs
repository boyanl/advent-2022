use scanf::sscanf;
use std::ops;
use std::{
    collections::HashSet,
    io::{self, stdin},
};

struct Command {
    direction: Vec2,
    amount: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        return Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        };
    }
}
impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Mul<i32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: i32) -> Self::Output {
        return Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

#[derive(Clone)]
struct Rope {
    knots: Vec<Vec2>,
}

fn new_rope(len: usize) -> Rope {
    return Rope {
        knots: vec![Vec2 { x: 0, y: 0 }; len],
    };
}

fn next_knot_pos(prev: Vec2, next: Vec2) -> Vec2 {
    if (prev.x - next.x).abs() == 2 {
        if prev.y == next.y {
            let d = if next.x < prev.x { 1 } else { -1 };
            return next + Vec2 { x: d, y: 0 };
        } else {
            let dx = if next.x < prev.x { 1 } else { -1 };
            let dy = if next.y < prev.y { 1 } else { -1 };
            return next + Vec2 { x: dx, y: dy };
        }
    } else if (prev.y - next.y).abs() == 2 {
        if prev.x == next.x {
            let d = if next.y < prev.y { 1 } else { -1 };
            return next + Vec2 { x: 0, y: d };
        } else {
            let dx = if next.x < prev.x { 1 } else { -1 };
            let dy = if next.y < prev.y { 1 } else { -1 };
            return next + Vec2 { x: dx, y: dy };
        }
    }
    return next;
}

fn move_rope(rope: &mut Rope, dir: Vec2) {
    let head = &mut rope.knots[0];
    *head += dir;

    for i in 0..rope.knots.len() - 1 {
        let next_pos = next_knot_pos(rope.knots[i], rope.knots[i + 1]);
        if next_pos == rope.knots[i + 1] {
            break;
        }
        rope.knots[i + 1] = next_pos;
    }
}

fn tail(rope: &Rope) -> Vec2 {
    return *rope.knots.last().unwrap();
}

fn visited_tail_positions(mut rope: Rope, commands: &Vec<Command>) -> usize {
    let mut visited = HashSet::new();

    for command in commands {
        for _ in 0..command.amount {
            move_rope(&mut rope, command.direction);
            visited.insert(tail(&rope));
        }
    }
    return visited.len();
}

fn read_commands() -> Vec<Command> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut dir: char = ' ';
        let mut cnt: u32 = 0;
        if sscanf!(&line, "{} {}", dir, cnt).is_ok() {
            let dir_vec = match dir {
                'L' => Vec2 { x: -1, y: 0 },
                'R' => Vec2 { x: 1, y: 0 },
                'U' => Vec2 { x: 0, y: 1 },
                'D' => Vec2 { x: 0, y: -1 },
                _ => todo!("not supposed to happen"),
            };
            result.push(Command {
                direction: dir_vec,
                amount: cnt,
            });
        }
    }
    return result;
}

fn part_one() {
    let moves = read_commands();
    let result = visited_tail_positions(new_rope(2), &moves);
    println!("{result}")
}

fn part_two() {
    let moves = read_commands();
    let result = visited_tail_positions(new_rope(10), &moves);
    println!("{result}")
}

fn main() {
    part_two();
}
