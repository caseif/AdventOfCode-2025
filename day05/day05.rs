use aoc2024_common::file::read_input_lines;

const DAY: u32 = 5;

fn main() {
    let (ranges, ingredients) = parse_list();
    println!("Part 1: {}", solve_p1(&ranges, &ingredients));
    println!("Part 2: {}", solve_p2(&ranges));
}

fn parse_list() -> (Vec<(u64, u64)>, Vec<u64>) {
    let lines = read_input_lines(DAY);
    let mut it = lines.iter();
    let ranges = it.by_ref().take_while(|line| line.contains("-"))
        .map(|line| line.split_once("-").unwrap())
        .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
        .collect();
    let ingredients = it.map(|line| line.parse::<u64>().unwrap()).collect();
    (ranges, ingredients)
}

fn solve_p1(ranges: &Vec<(u64, u64)>, ingredients: &Vec<u64>) -> u32 {
    let mut total: u32 = 0;

    for ingred in ingredients {
        for (min, max) in ranges {
            if ingred >= min && ingred <= max {
                total += 1;
                break;
            }
        }
    }

    return total;
}

fn solve_p2(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut new_ranges: Vec<(u64, u64)> = ranges.clone();
    new_ranges.sort_by_key(|(min, _)| *min);
    new_ranges = new_ranges.iter()
        .fold(Vec::new(), |acc, (min_1, max_1)| {
            let mut new_acc = acc.clone();
            if new_acc.len() > 0 {
                let (_, max_0) = new_acc.last_mut().unwrap();
                if min_1 <= max_0 {
                    *max_0 = (*max_0).max(*max_1);
                    return new_acc;
                }
            }
            new_acc.push((*min_1, *max_1));
            new_acc
        });
    new_ranges.iter()
        .map(|(min, max)| max - min + 1)
        .sum()
}
