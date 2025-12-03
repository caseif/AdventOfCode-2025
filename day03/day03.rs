use aoc2024_common::file::read_input_lines;

const DAY: u32 = 3;

fn main() {
    let list = parse_list();
    println!("Part 1: {}", solve_p1(&list));
    println!("Part 2: {}", solve_p2(&list));
}

fn parse_list() -> Vec<Vec<u8>> {
    read_input_lines(DAY).iter()
        .map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect()
}

fn solve_p1(banks: &Vec<Vec<u8>>) -> u32 {
    let mut total: u32 = 0;
    for bank in banks {
        let mut first_digit = 0;
        let mut first_digit_pos = 0;
        for (pos, digit) in bank.iter().enumerate() {
            if pos == bank.len() - 1 {
                break;
            }
            if *digit > first_digit {
                first_digit = *digit;
                first_digit_pos = pos;
            }
        }
        let mut second_digit = 0;
        for digit in bank.iter().skip(first_digit_pos + 1) {
            if *digit > second_digit {
                second_digit = *digit;
            }
        }

        let bank_joltage = (first_digit * 10 + second_digit) as u32;
        total += bank_joltage;
    }

    return total;
}

fn solve_p2(banks: &Vec<Vec<u8>>) -> u64 {
    let mut total: u64 = 0;
    for bank in banks {
        let mut bank_joltage = 0;
        let mut last_digit_pos: isize = -1;
        for i in 0..12 {
            let mut best_digit = 0;
            let mut best_digit_pos = 0;
            for (pos, digit) in bank.iter().enumerate().skip((last_digit_pos + 1) as usize) {
                if *digit > best_digit {
                    best_digit_pos = pos;
                    best_digit = *digit;
                }
                if pos == bank.len() - (12 - i) {
                    break;
                }
            }
            last_digit_pos = best_digit_pos as isize;
            bank_joltage += best_digit as u64 * 10u64.pow((11 - i) as u32);
        }

        total += bank_joltage;
    }

    return total;
}
