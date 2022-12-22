use self::Instruction::{Move, Turn};
use self::RotationDirection::{Left, Right};
use std::cmp;
use std::{collections::HashMap, io::stdin};

mod util;
type Point = util::vec2::Point2<i32>;
type Dir = util::vec2::Vec2<i32>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    pos: Point,
    dir: Dir,
}

#[derive(Debug, Clone, Copy)]
enum RotationDirection {
    Left,
    Right,
}

enum Instruction {
    Move(usize),
    Turn(RotationDirection),
}

struct Bounds {
    min_x: Vec<usize>,
    max_x: Vec<usize>,
    min_y: Vec<usize>,
    max_y: Vec<usize>,
}

fn move_in_dir(
    maze: &HashMap<(usize, usize), char>,
    bounds: &Bounds,
    state: &State,
    amount: usize,
) -> Point {
    let mut pos = state.pos;
    for _ in 0..amount {
        let mut new_pos = pos.clone();
        new_pos += state.dir;
        if new_pos.x > bounds.max_x[pos.y as usize] as i32 {
            new_pos.x = bounds.min_x[pos.y as usize] as i32;
        } else if new_pos.x < bounds.min_x[pos.y as usize] as i32 {
            new_pos.x = bounds.max_x[pos.y as usize] as i32;
        } else if new_pos.y > bounds.max_y[pos.x as usize] as i32 {
            new_pos.y = bounds.min_y[pos.x as usize] as i32;
        } else if new_pos.y < bounds.min_y[pos.x as usize] as i32 {
            new_pos.y = bounds.max_y[pos.x as usize] as i32;
        }
        if maze[&(new_pos.y as usize, new_pos.x as usize)] == '#' {
            return pos;
        }
        pos = new_pos;
    }

    return pos;
}

const RIGHT: Dir = Dir { x: 1, y: 0 };
const DOWN: Dir = Dir { x: 0, y: 1 };
const LEFT: Dir = Dir { x: -1, y: 0 };
const UP: Dir = Dir { x: 0, y: -1 };
const DIRECTIONS: [Dir; 4] = [RIGHT, DOWN, LEFT, UP];

fn turn_left(dir: &Dir) -> Dir {
    let idx = DIRECTIONS
        .iter()
        .position(|p| *p == *dir)
        .expect("should find direction");
    return DIRECTIONS[(idx + DIRECTIONS.len() - 1).rem_euclid(DIRECTIONS.len())];
}

fn turn_right(dir: &Dir) -> Dir {
    let idx = DIRECTIONS
        .iter()
        .position(|p| *p == *dir)
        .expect("should find direction");
    return DIRECTIONS[(idx + 1).rem_euclid(DIRECTIONS.len())];
}

fn opposite(dir: &Dir) -> Dir {
    let idx = DIRECTIONS
        .iter()
        .position(|p| *p == *dir)
        .expect("should find direction");
    return DIRECTIONS[(idx + 2).rem_euclid(DIRECTIONS.len())];
}

fn follow_path(
    maze: &HashMap<(usize, usize), char>,
    bounds: &Bounds,
    starting: &State,
    instructions: &Vec<Instruction>,
) -> State {
    let mut current = starting.clone();

    for instr in instructions {
        match instr {
            Move(n) => current.pos = move_in_dir(maze, bounds, &current, *n),
            TurnLeft => current.dir = turn_left(&current.dir),
            TurnRight => current.dir = turn_right(&current.dir),
        }
    }
    return current;
}
type Maze = HashMap<(usize, usize), char>;
fn read_maze() -> Maze {
    let mut maze = HashMap::new();
    for (i, line) in stdin().lines().map(|l| l.unwrap()).enumerate() {
        if line.is_empty() {
            break;
        }
        for (j, c) in line.chars().enumerate() {
            if c != ' ' {
                maze.insert((i, j), c);
            }
        }
    }
    return maze;
}

fn ensure_len(v: &mut Vec<usize>, len: usize, filler: usize) {
    if v.len() < len {
        v.resize(len, filler);
    }
}

fn get_bounds_x(maze: &Maze) -> (Vec<usize>, Vec<usize>) {
    let mut min_x = Vec::new();
    let mut max_x = Vec::new();
    for &(y, x) in maze.keys() {
        ensure_len(&mut min_x, y + 1, usize::MAX);
        ensure_len(&mut max_x, y + 1, usize::MIN);
        min_x[y] = cmp::min(min_x[y], x);
        max_x[y] = cmp::max(max_x[y], x);
    }

    return (min_x, max_x);
}

fn get_bounds_y(maze: &Maze) -> (Vec<usize>, Vec<usize>) {
    let mut min_y = Vec::new();
    let mut max_y = Vec::new();
    for &(y, x) in maze.keys() {
        ensure_len(&mut min_y, x + 1, usize::MAX);
        ensure_len(&mut max_y, x + 1, usize::MIN);
        min_y[x] = cmp::min(min_y[x], y);
        max_y[x] = cmp::max(max_y[x], y);
    }

    return (min_y, max_y);
}

fn get_bounds(maze: &Maze) -> Bounds {
    let (min_x, max_x) = get_bounds_x(maze);
    let (min_y, max_y) = get_bounds_y(maze);
    return Bounds {
        min_x: min_x,
        max_x: max_x,
        min_y: min_y,
        max_y: max_y,
    };
}

fn starting_pos(maze: &Maze) -> Point {
    let x = maze
        .keys()
        .filter(|(i, _)| *i == 0)
        .map(|(_, j)| *j)
        .min()
        .unwrap();
    return Point { x: x as i32, y: 0 };
}

fn parse_instructions(instructions_str: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    let mut amount = 0;

    for c in instructions_str.chars() {
        if c.is_ascii_digit() {
            amount *= 10;
            amount += (c as usize) - '0' as usize;
        } else if c == 'L' || c == 'R' {
            result.push(Move(amount));
            result.push(if c == 'L' { Turn(Left) } else { Turn(Right) });
            amount = 0;
        }
    }
    if amount > 0 {
        result.push(Move(amount));
    }

    return result;
}

fn calculate_score(state: &State) -> i32 {
    return (state.pos.y + 1) * 1000
        + (state.pos.x + 1) * 4
        + (DIRECTIONS.iter().position(|p| *p == state.dir).unwrap() as i32);
}

fn part_one() {
    let maze = read_maze();

    let mut instructions_str = String::new();
    stdin()
        .read_line(&mut instructions_str)
        .expect("should have instructions");

    let bounds = get_bounds(&maze);
    let start = starting_pos(&maze);
    let starting_dir = Dir { x: 1, y: 0 };
    let instructions = parse_instructions(&instructions_str);

    let final_state = follow_path(
        &maze,
        &bounds,
        &State {
            pos: start,
            dir: starting_dir,
        },
        &instructions,
    );
    let score = calculate_score(&final_state);
    println!("{score}");
}

fn find_side_length(bounds: &Bounds) -> usize {
    let mut side = usize::MAX;
    for y in 0..bounds.min_x.len() {
        side = cmp::min(side, bounds.max_x[y] - bounds.min_x[y] + 1);
    }
    return side;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Side {
    min_x: i32,
    min_y: i32,
    len: i32,
}

type SideDir = (Side, Dir);

fn fill_in_connections(conns: &HashMap<SideDir, SideDir>) -> HashMap<SideDir, SideDir> {
    let mut result = HashMap::new();
    for (&c1, &c2) in conns {
        result.insert(c1, c2);
        result.insert(c2, c1);
    }
    return result;
}

// Rotates a point 90° left in a square with given side
fn rotate_left(p: &Point, side: i32) -> Point {
    return Point {
        x: p.y,
        y: side - p.x - 1,
    };
}

fn rotate_right(p: &Point, side: i32) -> Point {
    return Point {
        x: side - p.y - 1,
        y: p.x,
    };
}

fn rotate(p: &Point, rotations: &Vec<RotationDirection>, side: i32) -> Point {
    let mut result = p.clone();
    for r in rotations {
        match r {
            Left => result = rotate_left(&result, side),
            Right => result = rotate_right(&result, side),
        }
    }

    return result;
}

fn turn_until_match(src: &Point, dest: &Point) -> Vec<RotationDirection> {
    let mut current = src.clone();
    let mut result = Vec::new();

    while current != *dest {
        current = turn_left(&current);
        result.push(Left);
    }
    return result;
}

fn follow_path_on_cube(
    maze: &Maze,
    transitions: &HashMap<SideDir, SideDir>,
    start_side: Side,
    start_dir: &Dir,
    instructions: &Vec<Instruction>,
) -> State {
    let mut pos = Point { x: 0, y: 0 };
    let mut dir = start_dir.clone();
    let mut side = start_side;

    for instr in instructions {
        match instr {
            Move(n) => {
                for _ in 0..*n {
                    let new_pos = pos + dir;
                    if new_pos.x < 0
                        || new_pos.x >= side.len
                        || new_pos.y < 0
                        || new_pos.y >= side.len
                    {
                        let (new_side, side_dir) = transitions[&(side, dir)];
                        let new_dir = opposite(&side_dir);
                        let pos_on_new_side;
                        if dir == LEFT || dir == RIGHT {
                            pos_on_new_side = Point {
                                x: (new_pos.x + side.len) % side.len,
                                y: new_pos.y,
                            };
                        } else {
                            pos_on_new_side = Point {
                                x: new_pos.x,
                                y: (new_pos.y + side.len) % side.len,
                            }
                        }

                        let rotations = turn_until_match(&dir, &new_dir);
                        let pos_on_new_side = rotate(&pos_on_new_side, &rotations, side.len);

                        if maze[&(
                            (new_side.min_y + pos_on_new_side.y) as usize,
                            (new_side.min_x + pos_on_new_side.x) as usize,
                        )] == '#'
                        {
                            break;
                        }

                        side = new_side;
                        pos = pos_on_new_side;
                        dir = new_dir;
                        continue;
                    }
                    if maze[&(
                        (side.min_y + new_pos.y) as usize,
                        (side.min_x + new_pos.x) as usize,
                    )] == '#'
                    {
                        break;
                    }
                    pos = new_pos;
                }
            }
            Turn(Left) => dir = turn_left(&dir),
            Turn(Right) => dir = turn_right(&dir),
        }
    }

    return State {
        pos: pos
            + Dir {
                x: side.min_x,
                y: side.min_y,
            },
        dir: dir,
    };
}

fn part_two() {
    let maze = read_maze();

    let mut instructions_str = String::new();
    stdin()
        .read_line(&mut instructions_str)
        .expect("should have instructions");

    let bounds = get_bounds(&maze);
    let instructions = parse_instructions(&instructions_str);

    let side_len = find_side_length(&bounds);

    let mut sides = Vec::new();
    for y in (0..bounds.min_x.len()).step_by(side_len) {
        for x in (bounds.min_x[y]..bounds.max_x[y]).step_by(side_len) {
            sides.push(Side {
                min_x: x as i32,
                min_y: y as i32,
                len: side_len as i32,
            });
        }
    }
    // Hardcoded connections ftw lol
    // Below are the ones for the example, after that for the puzzle input
    // The format is the following:
    // (side1, direction1) -> (side2, direction2) means that
    // the edge in side1 in direction1 is the same edge as side2 in direction2
    // this also means that if we go over the edge in side1 in by going in direction1,
    // our direction in the new side is opposite(direction2)
    // let side_connections = fill_in_connections(&HashMap::from([
    // ((sides[0], UP), (sides[1], UP)),
    // ((sides[0], RIGHT), (sides[5], RIGHT)),
    // ((sides[1], UP), (sides[0], UP)),
    // ((sides[1], RIGHT), (sides[2], LEFT)),
    // ((sides[1], DOWN), (sides[4], DOWN)),
    // ((sides[1], LEFT), (sides[5], DOWN)),
    // ((sides[2], UP), (sides[0], LEFT)),
    // ((sides[2], RIGHT), (sides[3], LEFT)),
    // ((sides[2], DOWN), (sides[4], LEFT)),
    // ((sides[3], UP), (sides[0], DOWN)),
    // ((sides[3], RIGHT), (sides[5], UP)),
    // ((sides[4], UP), (sides[3], DOWN)),
    // ((sides[4], RIGHT), (sides[5], LEFT)),
    // ((sides[5], UP), (sides[3], RIGHT)),
    // ((sides[5], RIGHT), (sides[0], RIGHT)),
    // ]));
    let side_connections = fill_in_connections(&HashMap::from([
        ((sides[0], UP), (sides[5], LEFT)),
        ((sides[0], RIGHT), (sides[1], LEFT)),
        ((sides[0], LEFT), (sides[3], LEFT)),
        ((sides[0], DOWN), (sides[2], UP)),
        ((sides[1], UP), (sides[5], DOWN)),
        ((sides[1], RIGHT), (sides[4], RIGHT)),
        ((sides[1], DOWN), (sides[2], RIGHT)),
        ((sides[2], LEFT), (sides[3], UP)),
        ((sides[2], DOWN), (sides[4], UP)),
        ((sides[3], RIGHT), (sides[4], LEFT)),
        ((sides[3], DOWN), (sides[5], UP)),
        ((sides[4], DOWN), (sides[5], RIGHT)),
    ]));

    let end_state = follow_path_on_cube(&maze, &side_connections, sides[0], &RIGHT, &instructions);
    let score = calculate_score(&end_state);
    println!("{score}");
}

fn main() {
    part_two();

    let mut instructions = String::new();
    stdin()
        .read_line(&mut instructions)
        .expect("should have instructions");
}
