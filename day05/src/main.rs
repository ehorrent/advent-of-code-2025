use regex::Regex;

#[derive(Clone)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn is_inside(&self, value: usize) -> bool {
        value >= self.min && value <= self.max
    }

    fn merge(&mut self, other: &Range) -> bool {
        if self.min <= other.max && self.max >= other.min {
            self.min = self.min.min(other.min);
            self.max = self.max.max(other.max);
            true
        } else {
            false
        }
    }

    fn size(&self) -> usize {
        self.max - self.min + 1
    }
}

fn solve_part1(fresh_ranges: &Vec<Range>, ids: &Vec<usize>) -> usize {
    let mut count = 0;
    for id in ids {
        if fresh_ranges.iter().any(|range| range.is_inside(*id)) {
            count += 1;
        }
    }

    count
}

fn merge_ranges_rec(fresh_ranges: &Vec<Range>) -> Vec<Range> {
    let mut merged_ranges: Vec<Range> = vec![];
    for range in fresh_ranges {
        let mut merged = false;
        for merged_range in &mut merged_ranges {
            if merged_range.merge(range) {
                merged = true;
                break;
            }
        }

        if !merged {
            merged_ranges.push(range.clone());
        }
    }

    if merged_ranges.len() == fresh_ranges.len() {
        return merged_ranges;
    }

    // Continue until no more merges are possible
    merge_ranges_rec(&merged_ranges)
}

fn solve_part2(fresh_ranges: &Vec<Range>) -> usize {
    let merged_ranges = merge_ranges_rec(fresh_ranges);
    merged_ranges.iter().map(|r| r.size()).sum()
}

fn parse_input(raw_data: &str) -> (Vec<Range>, Vec<usize>) {
    let re = Regex::new(r"\r\n\r\n|\n\n").unwrap();
    let sections: Vec<&str> = re.split(raw_data).collect();
    let ranges: Vec<Range> = sections[0]
        .lines()
        .map(|part| {
            let bounds: Vec<&str> = part.split("-").collect();
            Range {
                min: bounds[0].parse::<usize>().unwrap(),
                max: bounds[1].parse::<usize>().unwrap(),
            }
        })
        .collect();

    let ids = sections[1]
        .lines()
        .map(|part| part.parse::<usize>().unwrap())
        .collect();

    (ranges, ids)
}

fn main() {
    let raw_data = include_str!("../input/input-05.txt");
    let (ranges, ids) = parse_input(raw_data);

    println!("Solver - day 04:");

    // Part 1
    let result = solve_part1(&ranges, &ids);
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let result = solve_part2(&ranges);
    println!("  Part 2 - Final code: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn check_part1_result() {
        let (ranges, ids) = parse_input(RAW_INPUT);
        let result = solve_part1(&ranges, &ids);
        assert_eq!(result, 3);
    }

    #[test]
    fn check_part2_result() {
        let (ranges, _) = parse_input(RAW_INPUT);
        let result = solve_part2(&ranges);

        assert_eq!(result, 14);
    }
}
