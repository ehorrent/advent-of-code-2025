use shared::{Grid, Vector};
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    Splitter,
}

struct Solver {
    grid: Grid<Cell>,
}

impl Solver {
    fn new(grid: Grid<Cell>) -> Self {
        Solver { grid }
    }

    pub fn solve_part1(&self) -> usize {
        let grid_size = self.grid.size();

        let mut collision_count = 0;

        let start_pos = Vector {
            x: grid_size.x / 2,
            y: 0,
        };
        let mut beams: HashSet<Vector> = HashSet::new();
        beams.insert(start_pos);

        let down = Vector { x: 0, y: 1 };
        for _step in 0..grid_size.y - 1{
            let mut next_beams: HashSet<Vector> = HashSet::new();
            for beam_pos in beams.iter() {
                let next_pos = *beam_pos + down;

                if self.grid.get(&next_pos) == Some(&Cell::Splitter) {
                    collision_count += 1;

                    // Split the beam and check if position is inside the grid
                    let left_pos = Vector {
                        x: beam_pos.x - 1,
                        y: beam_pos.y + 1,
                    };

                    if self.grid.is_inside(&left_pos) {
                        next_beams.insert(left_pos);
                    }

                    let right_pos = Vector {
                        x: beam_pos.x + 1,
                        y: beam_pos.y + 1,
                    };

                    if self.grid.is_inside(&right_pos) {
                        next_beams.insert(right_pos);
                    }
                } else {
                    // Continue straight down
                    next_beams.insert(next_pos);
                }
            }

            beams = next_beams;
        }

        collision_count
    }

    pub fn solve_part2(&self) -> usize {
        let grid_size = self.grid.size();

        let start_pos = Vector {
            x: grid_size.x / 2,
            y: 0,
        };
        
        // Hashmap of beam positions -> number of timelines for that position
        let mut beams: HashMap<Vector, usize> = HashMap::new();
        beams.insert(start_pos, 1);

        let down = Vector { x: 0, y: 1 };
        for _step in 0..grid_size.y - 1{
            let mut next_beams: HashMap<Vector, usize> = HashMap::with_capacity(beams.len());
            for (beam_pos, timeline_count) in beams.iter() {
                let next_pos = *beam_pos + down;

                // Split the beam and check if position is inside the grid
                if self.grid.get(&next_pos) == Some(&Cell::Splitter) {
                    let left_pos = Vector {
                        x: beam_pos.x - 1,
                        y: beam_pos.y + 1,
                    };

                    if self.grid.is_inside(&left_pos) {
                        let entry = next_beams.entry(left_pos).or_insert(0);
                        *entry += timeline_count;
                    }

                    let right_pos = Vector {
                        x: beam_pos.x + 1,
                        y: beam_pos.y + 1,
                    };

                    if self.grid.is_inside(&right_pos) {
                        let entry = next_beams.entry(right_pos).or_insert(0);
                        *entry += timeline_count;
                    }
                } else {
                    let entry = next_beams.entry(next_pos).or_insert(0);
                    *entry += timeline_count;
                }
            }

            beams = next_beams;
        }

        beams.iter().map(|(_pos, count)| count).sum()
    }
}

fn parse_input(raw_data: &str) -> Grid<Cell> {
    let rows = raw_data
        .lines()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0) // Remove odd lines (always empty)
        .map(|(_, line)| {
            let row: Vec<Cell> = line
                .chars()
                .map(|c| match c {
                    '.' | 'S' => Cell::Empty,
                    '^' => Cell::Splitter,
                    _ => panic!("Invalid character in input"),
                })
                .collect();

            row
        })
        .collect::<Vec<Vec<Cell>>>();

    Grid::new(rows)
}

fn main() {
    let raw_data = include_str!("../input/input-07.txt");
    let grid = parse_input(raw_data);
    let solver = Solver::new(grid);

    println!("Solver - day 07:");

    // Part 1
    let result = solver.solve_part1();
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let result = solver.solve_part2();
    println!("  Part 2 - Final code: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn check_part1_result() {
        let grid = parse_input(RAW_INPUT);
        let mut solver = Solver::new(grid);
        let result = solver.solve_part1();
        assert_eq!(result, 21);
    }

    #[test]
    fn check_part2_result() {
        let grid = parse_input(RAW_INPUT);
        let mut solver = Solver::new(grid);
        let result = solver.solve_part2();

        assert_eq!(result, 40);
    }
}
