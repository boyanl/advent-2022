use scanf::{scanf, sscanf};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::{self, stdin};

mod util;
use util::vec2::Point2;

#[derive(Debug, Clone, Copy)]
struct Interval {
    min: i32,
    max: i32,
}

fn union(i1: Interval, i2: Interval) -> Interval {
    return Interval {
        min: min(i1.min, i2.min),
        max: max(i1.max, i2.max),
    };
}

fn is_empty(i: Interval) -> bool {
    return i.min > i.max;
}

fn intersection(i1: Interval, i2: Interval) -> Interval {
    return Interval {
        min: max(i1.min, i2.min),
        max: min(i1.max, i2.max),
    };
}

fn intersects(i1: Interval, i2: Interval) -> bool {
    return !is_empty(intersection(i1, i2));
}

fn adjoins(i1: Interval, i2: Interval) -> bool {
    return i1.max + 1 == i2.min || i2.max + 1 == i1.min;
}

fn read_sensors_and_beacons() -> Vec<(Point2, Point2)> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut sx, mut sy, mut bx, mut by) = (0, 0, 0, 0);
        if sscanf!(
            &line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            sx,
            sy,
            bx,
            by
        )
        .is_ok()
        {
            result.push((Point2 { x: sx, y: sy }, Point2 { x: bx, y: by }));
        }
    }
    return result;
}

fn part_one() {
    let sensors_beacons = read_sensors_and_beacons();

    let target_y = 2000000;
    let mut result_intervals = Vec::new();
    let mut beacons_included = HashSet::new();
    for (sensor, beacon) in sensors_beacons {
        let dist = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
        let y_target_dist = (sensor.y - target_y).abs();
        if dist < y_target_dist {
            continue;
        }

        let interval = Interval {
            min: sensor.x - (dist - y_target_dist),
            max: sensor.x + (dist - y_target_dist),
        };

        if target_y == beacon.y {
            beacons_included.insert(beacon);
        }

        result_intervals.push(interval);
    }

    result_intervals.sort_by(|i1, i2| i1.min.cmp(&i2.min));

    let mut result = Vec::new();
    for interval in result_intervals {
        if result.is_empty() {
            result.push(interval);
            continue;
        }
        let last = result.last_mut().unwrap();
        if intersects(*last, interval) || adjoins(*last, interval) {
            *last = union(*last, interval);
        } else {
            result.push(interval);
        }
    }

    let total: i32 = result
        .iter()
        .map(|interval| interval.max - interval.min + 1)
        .sum::<i32>()
        - (beacons_included.len() as i32);
    println!("{total}");
}

fn part_two() {
    let sensors_beacons = read_sensors_and_beacons();

    let max_x = 4000000;
    let maxy = 4000000;
    let x_interval = Interval { min: 0, max: max_x };
    for y in 0..=maxy {
        let mut result_intervals = Vec::new();
        for (sensor, beacon) in &sensors_beacons {
            let dist = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
            let y_dist = (sensor.y - y).abs();

            let interval = intersection(
                Interval {
                    min: sensor.x - (dist - y_dist),
                    max: sensor.x + (dist - y_dist),
                },
                x_interval,
            );

            if !is_empty(interval) {
                result_intervals.push(interval);
            }
        }

        result_intervals.sort_by(|i1, i2| i1.min.cmp(&i2.min));

        let mut result = Vec::new();
        for interval in result_intervals {
            if result.is_empty() {
                result.push(interval);
                continue;
            }
            let last = result.last_mut().unwrap();
            if intersects(*last, interval) || adjoins(*last, interval) {
                *last = union(*last, interval);
            } else {
                result.push(interval);
            }
        }
        let mut x = -1;
        let mut found = false;
        if result.first().unwrap().min > 0 {
            x = 0;
            found = true;
        } else if result.last().unwrap().max < max_x {
            x = max_x;
            found = true;
        } else if result.len() >= 2 {
            x = result.first().unwrap().max + 1;
            found = true;
        }

        if found {
            let result = (x as i64) * (max_x as i64) + (y as i64);
            println!("{result}");
            break;
        }
    }
}

fn main() {
    part_two()
}
