use scanf::sscanf;
use std::{
    collections::{HashSet, VecDeque},
    io::{self, stdin},
};

mod util;
type Point = util::vec3::Point3<i32>;

fn count_adjacent(pts: &HashSet<Point>, pt: Point, dir: Point) -> i32 {
    return (pts.contains(&(pt + dir)) as i32) + (pts.contains(&(pt - dir)) as i32);
}

static DIR_X: Point = Point { x: 1, y: 0, z: 0 };
static DIR_Y: Point = Point { x: 0, y: 1, z: 0 };
static DIR_Z: Point = Point { x: 0, y: 0, z: 1 };

fn read_points() -> HashSet<Point> {
    let mut pts = HashSet::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut x, mut y, mut z) = (0, 0, 0);
        if sscanf!(&line, "{i32},{i32},{i32}", x, y, z).is_ok() {
            pts.insert(Point::from((x, y, z)));
        }
    }
    return pts;
}

fn part_one() {
    let pts = read_points();

    let mut result = 0;
    for &pt in &pts {
        let mut adjacent_cnt = 0;
        for dir in [DIR_X, DIR_Y, DIR_Z] {
            adjacent_cnt += count_adjacent(&pts, pt, dir);
        }
        result += 6 - adjacent_cnt;
    }

    println!("{result}");
}

fn other_directions(dir: Point) -> [Point; 4] {
    if dir == DIR_X || dir == -DIR_X {
        return [DIR_Y, -DIR_Y, DIR_Z, -DIR_Z];
    } else if dir == DIR_Y || dir == -DIR_Y {
        return [DIR_X, -DIR_X, DIR_Z, -DIR_Z];
    } else if dir == DIR_Z || dir == -DIR_Z {
        return [DIR_X, -DIR_X, DIR_Y, -DIR_Y];
    }
    todo!();
}

fn part_two() {
    let pts = read_points();
    let leftmost_pt = *pts.iter().min_by(|p1, p2| p1.x.cmp(&p2.x)).unwrap();
    let steam_start = leftmost_pt - Point::from((1, 0, 0));

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let mut total_area = 0;

    q.push_back(steam_start);
    while !q.is_empty() {
        let pos = q.pop_front().unwrap();
        visited.insert(pos);

        for dir in [DIR_X, DIR_Y, DIR_Z] {
            let adjacent_cnt = count_adjacent(&pts, pos, dir);
            total_area += adjacent_cnt;
            if adjacent_cnt > 0 {
                for other_dir in other_directions(dir) {
                    let other = pos + other_dir;
                    if !pts.contains(&other) && !visited.contains(&other) {
                        q.push_back(other);
                        visited.insert(other);
                    }
                }
            } else {
                for other_dir in other_directions(dir) {
                    for p in [pos + dir, pos - dir] {
                        let diagonal = p + other_dir;
                        if pts.contains(&diagonal) && !visited.contains(&p) {
                            q.push_back(p);
                            visited.insert(p);
                        }
                    }
                }
            }
        }
    }

    println!("{total_area}");
}

fn main() {
    part_two();
}
