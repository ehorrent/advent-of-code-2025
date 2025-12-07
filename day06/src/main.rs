use shared::{Grid, Vector};

enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn from_char(input: &str) -> Option<Operation> {
        match input {
            "+" => Some(Operation::Add),
            "*" => Some(Operation::Mul),
            _ => None,
        }
    }
}

fn parse_input(raw_data: &str) -> (Grid<usize>, Vec<Operation>) {
    let rows = raw_data
        .lines()
        .map(|line| {
            let row: Vec<usize> = line
                .split_whitespace()
                .flat_map(|c| c.parse::<usize>())
                .collect();

            row
        })
        .filter(|row| row.len() > 0)
        .collect::<Vec<Vec<usize>>>();

    let ops: Vec<Operation> = raw_data
        .lines()
        .skip(rows.len())
        .flat_map(|line| {
            line.split_whitespace()
                .filter_map(|c| Operation::from_char(c))
                .collect::<Vec<Operation>>()
        })
        .collect();

    (Grid::new(rows), ops)
}

fn solve_part1(grid: &Grid<usize>, ops: &Vec<Operation>) -> usize {
    let grid_size = grid.size();

    let mut total = 0;
    for column_index in 0..grid_size.x as usize {
        let mut value = 0;
        let op = &ops[column_index];
        for row_index in 0..grid_size.x {
            let cell_value = grid
                .get(&Vector {
                    x: column_index as i64,
                    y: row_index as i64,
                })
                .unwrap();

            if 0 == row_index {
                value = *cell_value;
                continue;
            }

            match op {
                Operation::Add => value += cell_value,
                Operation::Mul => value *= cell_value,
            }
        }

        total += value;
    }

    total
}

fn solve_part2(raw_data: &str) -> usize {
    let rows = raw_data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Transpose rows to columns and parse values
    // Get a sequence of: Number / Number / None / Number ... / Number / None / ...
    let transposed_rows = (0..rows[0].len())
        .map(|i| {
            rows.iter()
                .take(rows.len() - 1)
                .map(|row| row[i])
                .collect::<String>()
        })
        .map(|str| {
            let str = str.trim();
            if str.is_empty() {
                return None;
            }

            Some(str.parse::<usize>().unwrap())
        })
        .collect::<Vec<Option<usize>>>();

    let ops: Vec<Operation> = raw_data
        .lines()
        .skip(rows.len() - 1)
        .flat_map(|line| {
            line.split_whitespace()
                .filter_map(|c| Operation::from_char(c))
                .collect::<Vec<Operation>>()
        })
        .collect();

    let mut total = 0;
    let mut current_value = 0;
    let mut op_index = 0;
    let mut current_operation = &ops[op_index];

    for value in transposed_rows {
        match value {
            Some(value) => {
                if 0 == current_value {
                    current_value = value;
                    continue;
                }

                match current_operation {
                    Operation::Add => current_value += value,
                    Operation::Mul => current_value *= value,
                }
            }
            None => {
                total += current_value;
                current_value = 0;
                op_index += 1;
                current_operation = &ops[op_index];
            }
        }
    }

    total + current_value
}

fn main() {
    let raw_data = include_str!("../input/input-06.txt");
    let (grid, ops) = parse_input(raw_data);

    println!("Solver - day 06:");

    // Part 1
    let result = solve_part1(&grid, &ops);
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let result = solve_part2(raw_data);
    println!("  Part 2 - Final code: {}", result);
    // 11601712780573 - KO
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn check_part1_result() {
        let (grid, ops) = parse_input(RAW_INPUT);
        let result = solve_part1(&grid, &ops);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn check_part2_result() {
        let result = solve_part2(RAW_INPUT);
        assert_eq!(result, 3263827);
    }
}
