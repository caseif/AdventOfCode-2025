use std::collections::{HashMap, HashSet};
use aoc2024_common::file::read_input_lines;

const DAY: u32 = 9;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Vector2u {
    x: u64,
    y: u64,
}

impl Vector2u {
    fn new(x: impl Into<u64>, y: impl Into<u64>) -> Self {
        Self { x: x.into(), y: y.into() }
    }
}

fn main() {
    let coords = parse_input();
    println!("Part 1: {}", solve_p1(&coords));
    println!("Part 2: {}", solve_p2(&coords));
}

fn parse_input() -> Vec<Vector2u> {
    read_input_lines(DAY).iter()
        .map(|line| line.split(",").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .map(|spl| Vector2u::new(spl[0], spl[1]))
        .collect()
}

fn solve_p1(coords: &Vec<Vector2u>) -> u64 {
    let mut largest_area = 0;
    for (i, coord_1) in coords.iter().enumerate() {
        for (_, coord_2) in coords.iter().enumerate().skip(i + 1) {
            let cur_area = (coord_1.x.abs_diff(coord_2.x) + 1) *
                (coord_1.y.abs_diff(coord_2.y) + 1);
            if cur_area > largest_area {
                largest_area = cur_area;
            }
        }
    }
    return largest_area;
}

fn solve_p2(coords: &Vec<Vector2u>) -> u64 {
    let mut largest_area = 0;
    for (i, corner_1) in coords.iter().enumerate() {
        'outer:
        for (j, corner_2) in coords.iter().enumerate().skip(i + 1) {
            let cur_area = (corner_1.x.abs_diff(corner_2.x) + 1) *
                (corner_1.y.abs_diff(corner_2.y) + 1);
            if cur_area <= largest_area {
                continue;
            }

            let rect_x_min = corner_1.x.min(corner_2.x);
            let rect_x_max = corner_1.x.max(corner_2.x);
            let rect_y_min = corner_1.y.min(corner_2.y);
            let rect_y_max = corner_1.y.max(corner_2.y);
            for k in 0..coords.len() {
                if k == i || k == j {
                    continue;
                }

                let endpoint_1 = coords[k];
                let endpoint_2 = if k < coords.len() - 1 { coords[k + 1] } else { coords[0] };
                let ep_x_min = endpoint_1.x.min(endpoint_2.x);
                let ep_x_max = endpoint_1.x.max(endpoint_2.x);
                let ep_y_min = endpoint_1.y.min(endpoint_2.y);
                let ep_y_max = endpoint_1.y.max(endpoint_2.y);
                // modified AABB check
                if ep_x_min == ep_x_max {
                    if ep_x_min > rect_x_min && ep_x_min < rect_x_max &&
                        ep_y_min <= rect_y_max && ep_y_max >= rect_y_min {
                        continue 'outer;
                    }
                } else {
                    if ep_y_min > rect_y_min && ep_y_min < rect_y_max &&
                        ep_x_min <= rect_x_max && ep_x_max >= rect_x_min {
                        continue 'outer;
                    }
                }
            }

            largest_area = cur_area;
        }
    }
    return largest_area;
}
