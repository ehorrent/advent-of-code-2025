use shared::{Grid, Position};

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Roll,
}

struct Solver {
    grid: Grid<Cell>,
}

impl Solver {
    fn new(grid: Grid<Cell>) -> Self {
        Solver { grid }
    }

    pub fn remove_rolls(&mut self) -> usize {
        let mut to_remove = vec![];
        for (y, row) in self.grid.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let pos = Position {
                    x: x as i32,
                    y: y as i32,
                };

                if let Cell::Roll = cell {
                    // Count rolls around this position
                    let cells_around = self.count_rolls_around(&pos);
                    if cells_around < 4 {
                        to_remove.push(pos);
                    }
                }
            }
        }

        for pos in to_remove.iter() {
            if let Some(cell) = self.grid.get_mut(pos) {
                *cell = Cell::Empty;
            }
        }

        to_remove.len()
    }
    pub fn solve_part1(&mut self) -> usize {
        self.remove_rolls()
    }

    pub fn solve_part2(&mut self) -> usize {
        let mut count = 0;
        loop {
            let removed = self.remove_rolls();
            if removed == 0 {
                break;
            }

            count += removed;
        }

        count
    }

    fn count_rolls_around(&self, pos: &Position) -> usize {
        let directions = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        let mut count = 0;
        for (dir_x, dir_y) in directions {
            let next_pos = Position {
                x: pos.x + dir_x,
                y: pos.y + dir_y,
            };

            if next_pos.x < 0 || next_pos.y < 0 {
                continue;
            }

            if let Some(cell) = self.grid.get(&next_pos) {
                if let Cell::Roll = cell {
                    count += 1;
                }
            }
        }

        count
    }
}

fn parse_input(raw_data: &str) -> Grid<Cell> {
    let rows = raw_data
        .lines()
        .map(|line| {
            let row: Vec<Cell> = line
                .chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Roll,
                    _ => panic!("Invalid character in input"),
                })
                .collect();

            row
        })
        .collect::<Vec<Vec<Cell>>>();

    Grid::new(rows)
}

fn main() {
    let raw_data = include_str!("../input/input-04.txt");
    let grid = parse_input(raw_data);

    println!("Solver - day 04:");

    // Part 1
    let mut solver = Solver::new(grid.clone());
    let result = solver.solve_part1();
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let mut solver = Solver::new(grid.clone());
    let result = solver.solve_part2();
    println!("  Part 2 - Final code: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn check_part1_result() {
        let grid = parse_input(RAW_INPUT);
        let mut solver = Solver::new(grid);
        let result = solver.solve_part1();
        assert_eq!(result, 13);
    }

    #[test]
    fn check_part2_result() {
        let grid = parse_input(RAW_INPUT);
        let mut solver = Solver::new(grid);
        let result = solver.solve_part2();

        assert_eq!(result, 43);
    }
}
