use std::collections::{HashMap, HashSet};
use aoc2024_common::file::read_input_lines;

const DAY: u32 = 7;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Vec2u {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug, Default)]
struct Manifold {
    size: Vec2u,
    start: Vec2u,
    splitters: HashSet<Vec2u>,
}

fn main() {
    let manifold = parse_manifold();
    println!("Part 1: {}", solve_p1(&manifold));
    println!("Part 2: {}", solve_p2(&manifold));
}

fn parse_manifold() -> Manifold {
    let mut manifold = Manifold::default();

    let lines = read_input_lines(DAY);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            manifold.size = Vec2u { x: lines.first().unwrap().len() as u32, y: lines.len() as u32 };
            match c {
                'S' => { manifold.start = Vec2u { x: x as u32, y: y as u32 }; }
                '^' => { manifold.splitters.insert(Vec2u { x: x as u32, y: y as u32 }); }
                _ => {}
            }
        }
    }

    manifold
}

fn solve_p1(manifold: &Manifold) -> u32 {
    let mut splits: u32 = 0;

    let mut prev_beams = HashSet::<u32>::new();
    prev_beams.insert(manifold.start.x);
    for cur_y in (manifold.start.y + 1)..manifold.size.y {
        //println!("checking row y={cur_y}");
        let mut cur_beams = HashSet::<u32>::new();
        for &beam_x in &prev_beams {
            //println!("checking for splitter at x={beam_x}")
            if manifold.splitters.contains(&Vec2u { x: beam_x, y: cur_y + 1 }) {
                splits += 1;
                if beam_x > 0 {
                    cur_beams.insert(beam_x - 1);
                }
                if beam_x < manifold.size.x - 1 {
                    cur_beams.insert(beam_x + 1);
                }
            } else {
                cur_beams.insert(beam_x);
            }

        }

        prev_beams = cur_beams;
    }

    return splits;
}

fn solve_p2(manifold: &Manifold) -> u64 {
    let mut memo = HashMap::<Vec2u, u64>::new();

    return count_timelines(manifold, &manifold.start, &mut memo);
}

fn count_timelines(manifold: &Manifold, beam: &Vec2u, memo: &mut HashMap<Vec2u, u64>) -> u64 {
    if beam.y == manifold.size.y - 1 {
        return 1;
    }

    if let Some(&memo_count) = memo.get(beam) {
        return memo_count;
    }

    let next_beam = Vec2u { x: beam.x, y: beam.y + 1 };
    if !manifold.splitters.contains(&next_beam) {
        return count_timelines(manifold, &next_beam, memo);
    }

    let mut count = 0;

    if beam.x > 0 {
        let left = Vec2u { x: beam.x - 1, y: beam.y + 1 };
        count += count_timelines(manifold, &left, memo);
    }

    if beam.x < manifold.size.x - 1 {
        let right = Vec2u { x: beam.x + 1, y: beam.y + 1 };
        count += count_timelines(manifold, &right, memo);
    }

    memo.insert(*beam, count);
    return count;
}
