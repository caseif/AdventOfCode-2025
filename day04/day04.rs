use aoc2024_common::file::read_input_lines;

const DAY: u32 = 4;

fn main() {
    let map = parse_map();
    println!("Part 1: {}", solve_p1(&map));
    println!("Part 2: {}", solve_p2(&map));
}

fn parse_map() -> Vec<Vec<bool>> {
    read_input_lines(DAY).iter()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn solve_p1(map: &Vec<Vec<bool>>) -> u32 {
    const OFFSETS: [(i32, i32); 8] =
        [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let mut total: u32 = 0;
    for y in 0..map.len() as i32 {
        for x in 0..map[0].len() as i32 {
            if !map[y as usize][x as usize] {
                continue;
            }

            let mut adj_rolls = 0;
            for offset in OFFSETS {
                let check_y = y + offset.0;
                let check_x = x + offset.1;
                if check_y < 0 || check_y >= map.len() as i32 ||
                    check_x < 0 || check_x >= map[0].len() as i32 {
                    continue;
                }
                if map[check_y as usize][check_x as usize] {
                    adj_rolls += 1;
                }
            }

            if adj_rolls < 4 {
                total += 1;
            }
        }
    }

    return total;
}

fn solve_p2(map: &Vec<Vec<bool>>) -> u32 {
    let mut map_copy = map.clone();

    const OFFSETS: [(i32, i32); 8] =
        [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let mut total: u32 = 0;

    loop {
        let mut pass_total = 0;

        for y in 0..map.len() as i32 {
            for x in 0..map[0].len() as i32 {
                if !map_copy[y as usize][x as usize] {
                    continue;
                }

                let mut adj_rolls = 0;
                for offset in OFFSETS {
                    let check_y = y + offset.0;
                    let check_x = x + offset.1;
                    if check_y < 0 || check_y >= map.len() as i32 ||
                        check_x < 0 || check_x >= map[0].len() as i32 {
                        continue;
                    }
                    if map_copy[check_y as usize][check_x as usize] {
                        adj_rolls += 1;
                    }
                }

                if adj_rolls < 4 {
                    total += 1;
                    pass_total += 1;
                    map_copy[y as usize][x as usize] = false;
                }
            }
        }

        if pass_total == 0 {
            break;
        }
    }

    return total;
}
