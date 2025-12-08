use std::collections::VecDeque;
use aoc2024_common::file::read_input_lines;

const DAY: u32 = 6;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Operator {
    Undefined,
    Add,
    Multiply,
}

impl Default for Operator {
    fn default() -> Self {
        Operator::Undefined
    }
}

#[derive(Clone, Default)]
struct Problem {
    operands: Vec<u32>,
    operator: Operator,
}

fn main() {
    println!("Part 1: {}", solve_p1(&parse_problems_1()));
    println!("Part 2: {}", solve_p2(&read_input_lines(DAY)));
}

fn parse_problems_1() -> Vec<Problem> {
    let lines = read_input_lines(DAY);
    let count = lines[0].split_ascii_whitespace().count();
    let mut problems = Vec::<Problem>::with_capacity(count);
    problems.resize_with(count, || Problem::default());

    for line in lines {
        for (i, item) in line.split_ascii_whitespace().enumerate() {
            if item == "+" {
                problems[i].operator = Operator::Add;
            } else if item == "*" {
                problems[i].operator = Operator::Multiply;
            } else {
                problems[i].operands.push(item.parse::<u32>().unwrap());
            }
        }
    }

    problems
}

fn solve_p1(problems: &Vec<Problem>) -> u64 {
    let mut total: u64 = 0;

    for problem in problems {
        let mut result: u64 = if problem.operator == Operator::Add { 0 } else { 1 };
        for operand in &problem.operands {
            match problem.operator {
                Operator::Add => { result += *operand as u64; }
                Operator::Multiply => { result *= *operand as u64; }
                Operator::Undefined => { panic!(); }
            }
        }
        total += result;
    }

    return total;
}

fn solve_p2(lines: &Vec<String>) -> u64 {
    let max_len = lines.iter().map(|line| line.len()).max().unwrap();

    let mut line_its: Vec<_> = lines.iter().map(|line| line.chars().collect::<VecDeque<_>>()).collect();
    let mut total: u64 = 0;
    let mut cur_operands: Vec<u32> = vec![];
    let mut cur_operator = Operator::Undefined;
    for i in 0..=max_len {
        let cur_chars: Vec<_> = line_its.iter_mut().map(|chars| chars.pop_front()).collect();
        if i == max_len || cur_chars.iter().all(|opt| opt.map(|c| c == ' ').unwrap_or(true)) {
            let mut result: u64 = if cur_operator == Operator::Add { 0 } else { 1 };
            for operand in &cur_operands {
                match cur_operator {
                    Operator::Add => { result += *operand as u64; }
                    Operator::Multiply => { result *= *operand as u64; }
                    Operator::Undefined => { panic!(); }
                }
            }
            total += result;
            cur_operator = Operator::Undefined;
            cur_operands.clear();
            continue;
        }

        match cur_chars[lines.len() - 1].unwrap_or(' ') {
            '+' => { cur_operator = Operator::Add; }
            '*' => { cur_operator = Operator::Multiply; }
            _ => {}
        }

        let mut cur_exp = 0;
        let mut cur_number = 0;
        for j in 0..=(lines.len() - 2) {
            let c = cur_chars[lines.len() - 2 - j].unwrap_or(' ');
            if c == ' ' {
                continue;
            }
            cur_number += (c as u32 - '0' as u32) * 10u32.pow(cur_exp as u32);
            cur_exp += 1;
        }
        cur_operands.push(cur_number);
    }
    return total;
}
