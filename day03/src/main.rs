type Bank = Vec<usize>;

fn parse_input(raw_data: &str) -> Vec<Bank> {
    raw_data
        .lines()
        .map(|line| {
            let bank: Bank = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();

            bank
        })
        .collect()
}

/// Recursive function used to add the maximum joltage from a bank of batteries
fn add_max_joltage_rec(acc_max_joltage: &mut usize, batteries: &[usize], digit_count: usize) {
    if digit_count == 0 {
        return;
    }

    let mut max_joltage = 0;
    let mut max_index = 0;
    for i in 0..batteries.len() - digit_count + 1 {
        if batteries[i] > max_joltage {
            max_joltage = batteries[i];
            max_index = i;
        }
    }

    *acc_max_joltage += max_joltage * 10_usize.pow((digit_count - 1) as u32);

    add_max_joltage_rec(acc_max_joltage, &batteries[(max_index + 1)..], digit_count - 1);
}

fn solve_part1(banks: &Vec<Bank>) -> usize {
    let mut code = 0;
    for bank in banks {
        add_max_joltage_rec(&mut code, &bank[..], 2);
    }
    
    code
}

fn solve_part2(banks: &Vec<Bank>) -> usize {
    let mut max_joltage_sum = 0;
    for bank in banks {
        add_max_joltage_rec(&mut max_joltage_sum, &bank[..], 12);
    }

    max_joltage_sum
}

fn main() {
    let raw_data = include_str!("../input/input-03.txt");
    let all_banks = parse_input(raw_data);

    println!("Solver - day 03:");

    // Part 1
    let result01 = solve_part1(&all_banks);
    println!("  Part 1 - Final code: {}", result01);

    // Part 2
    let result01 = solve_part2(&all_banks);
    println!("  Part 2 - Final code: {}", result01);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_result() {
        let raw_data = "987654321111111
811111111111119
234234234234278
818181911112111";

        let all_banks = parse_input(raw_data);
        let result = solve_part1(&all_banks);
        assert_eq!(result, 357);
    }

    #[test]
    fn check_part2_result() {
        let raw_data = "987654321111111
811111111111119
234234234234278
818181911112111";

        let all_banks = parse_input(raw_data);
        let result = solve_part2(&all_banks);
        assert_eq!(result, 3121910778619);
    }
}
