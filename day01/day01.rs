use aoc2024_common::file::read_input_lines;
use Direction::{Left, Right};

const DAY: u32 = 1;

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Left,
            'R' => Right,
            _ => panic!("Invalid direction char: {}", c)
        }
    }
}

struct Rotation {
    dir: Direction,
    dist: u32,
}

fn main() {
    let list = parse_list();
    println!("Part 1: {}", solve_p1(&list));
    println!("Part 2: {}", solve_p2(&list));
}

fn parse_list() -> Vec<Rotation> {
    let lines = read_input_lines(DAY);
    lines.iter()
        .map(|line| (line.chars().nth(0).unwrap(), line[1..].to_string()))
        .map(|(dir, dist)|
            Rotation { dir: Direction::from_char(dir), dist: dist.parse::<u32>().unwrap() })
        .collect()
}

fn solve_p1(rots: &Vec<Rotation>) -> u32 {
    let mut password = 0;
    let mut cur_num: i32 = 50;
    for rot in rots {
        match rot.dir {
            Left => {
                cur_num = (cur_num - rot.dist as i32).rem_euclid(100)
            },
            Right => {
                cur_num = (cur_num + rot.dist as i32).rem_euclid(100)
            }
        }

        if cur_num == 0 {
            password += 1;
        }
    }

    password
}

fn solve_p2(rots: &Vec<Rotation>) -> u32 {
    let mut password: u32 = 0;
    let mut cur_num: i32 = 50;
    for rot in rots {
        match rot.dir {
            Left => {
                let orig_num = cur_num;
                cur_num = cur_num - rot.dist as i32;
                if cur_num < 0 {
                    password += (-cur_num / 100) as u32;
                    if orig_num != 0 {
                        password += 1;
                    }
                    cur_num = cur_num.rem_euclid(100);
                } else if cur_num == 0 {
                    password += 1;
                }
            },
            Right => {
                cur_num = cur_num + rot.dist as i32;
                if cur_num > 99 {
                    password += (cur_num / 100) as u32;
                    cur_num = cur_num.rem_euclid(100);
                }
            }
        }
    }

    password
}
