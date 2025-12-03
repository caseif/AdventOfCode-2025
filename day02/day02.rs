use std::collections::HashSet;
use aoc2024_common::file::read_input_string;

const DAY: u32 = 2;

struct Range {
    min: u64,
    max: u64,
}

fn main() {
    let list = parse_list();
    println!("Part 1: {}", solve_p1(&list));
    println!("Part 2: {}", solve_p2(&list));
}

fn parse_list() -> Vec<Range> {
    read_input_string(DAY).split(",")
        .map(|r| r.split_once("-").unwrap())
        .map(|rt| Range { min: rt.0.parse::<u64>().unwrap(), max: rt.1.parse::<u64>().unwrap()})
        .collect()
}

fn solve_p1(ranges: &Vec<Range>) -> u64 {
    let mut total: u64 = 0;
    for range in ranges {
        let min_len: u32 = range.min.ilog10() + 1;
        let max_len: u32 = range.max.ilog10() + 1;
        // range only includes numbers with odd number of digits, no invalid IDs possible
        if min_len == max_len && min_len % 2 != 0 {
            continue
        }

        // hack: all ranges cross at most single order of magnitude, so we can shift the min/max
        // to a power of 10 to get the sub-range in which invalid IDs can appear
        let eff_min = if min_len % 2 == 0 { range.min } else { 10u64.pow(max_len - 1) };
        let eff_max = if max_len % 2 == 0 { range.max } else { 10u64.pow(min_len) - 1 };
        let range_order = if min_len % 2 == 0 { min_len } else { max_len };

        let min_nibble_top = eff_min / 10u64.pow(range_order / 2);
        let max_nibble_top = eff_max / 10u64.pow(range_order / 2);

        for i in min_nibble_top..=max_nibble_top {
            let id = i * 10u64.pow(range_order / 2) + i;
            if id >= eff_min && id <= eff_max {
                total += id;
            }
        }
    }

    return total;
}

fn solve_p2(ranges: &Vec<Range>) -> u64 {
    let mut total: u64 = 0;
    let mut seen = HashSet::<u64>::new();
    for range in ranges {
        let min_len = range.min.ilog10() + 1;
        let max_len = range.max.ilog10() + 1;
        let seq_len_max = max_len / 2;
        for seq_len in 1..=seq_len_max {
            if min_len % seq_len != 0 && max_len % seq_len != 0 {
                continue;
            }

            let lens = if min_len != max_len { &[min_len, max_len] } else { &[min_len] as &[u32] };
            'outer: for len in lens {
                // https://redd.it/1pc2b0m
                if *len == 1 {
                    continue;
                }
                for i in 10u64.pow(seq_len - 1)..=(10u64.pow(seq_len) - 1) {
                    let mut num = 0;
                    for j in (0..*len).step_by(seq_len as usize) {
                        num += i * 10u64.pow(j);
                    }
                    if seen.contains(&num) {
                        continue;
                    }
                    if num > range.max {
                        break 'outer;
                    }
                    if num >= range.min {
                        total += num;
                        seen.insert(num);
                    }
                }
            }
        }
    }

    return total;
}
