use std::collections::HashSet;

struct Range {
    min: usize,
    max: usize,
}

fn parse_input(raw_data: &str) -> Vec<Range> {
    raw_data
        .split(",")
        .map(|part| {
            let bounds: Vec<&str> = part.split("-").collect();
            Range {
                min: bounds[0].parse::<usize>().unwrap(),
                max: bounds[1].parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn solve_part1(ranges: &Vec<Range>) -> usize {
    let mut invalid_ids = HashSet::new();

    for range in ranges {
        for value in range.min..=range.max {
            let str_value = value.to_string();
            if str_value.len() % 2 != 0 {
                continue;
            }

            let half_len = str_value.len() / 2;
            let first_half = &str_value[0..half_len];
            let second_half = &str_value[half_len..];
            if first_half == second_half {
                invalid_ids.insert(value);
            }
        }
    }

    invalid_ids.iter().sum()
}

fn solve_part2(ranges: &Vec<Range>) -> usize {
    let mut invalid_ids = HashSet::new();

    for range in ranges {
        for value in range.min..=range.max {
            let str_value = value.to_string();

            for chunk_size in 1..=str_value.len() / 2 {
                if str_value.len() % chunk_size != 0 {
                    continue;
                }

                let chunks_count = str_value.len() / chunk_size;
                let first_chunk = &str_value[0..chunk_size];
                for i in 1..chunks_count {
                    let start = i * chunk_size;
                    let end = start + chunk_size;
                    if &str_value[start..end] != first_chunk {
                        break;
                    }

                    if i == chunks_count - 1 {
                        invalid_ids.insert(value);
                    }
                }
            }
        }
    }

    invalid_ids.iter().sum()
}

fn main() {
    let raw_data = include_str!("../input/input-02.txt");
    let all_ranges = parse_input(raw_data);

    println!("Solver - day 02:");

    // Part 1
    let result01 = solve_part1(&all_ranges);
    println!("  Part 1 - Final code: {}", result01);

    // Part 2
    let result01 = solve_part2(&all_ranges);
    println!("  Part 2 - Final code: {}", result01);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_result() {
        let raw_data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let all_ranges = parse_input(raw_data);
        let result = solve_part1(&all_ranges);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn check_part2_result() {
        let raw_data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let all_ranges = parse_input(raw_data);
        let result = solve_part2(&all_ranges);
        assert_eq!(result, 4174379265);
    }
}
