use shared::{Grid, Vector};
use std::collections::HashSet;

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    Occupied,
    Unknown,
}

const DIRECTIONS: [Vector; 4] = [
    Vector { x: 1, y: 0 },
    Vector { x: -1, y: 0 },
    Vector { x: 0, y: 1 },
    Vector { x: 0, y: -1 },
];

struct Solver {
    red_tiles: Vec<Vector>,
}

impl Solver {
    fn new(red_tiles: Vec<Vector>) -> Self {
        Solver { red_tiles }
    }

    fn area(pos1: &Vector, pos2: &Vector) -> i64 {
        ((pos1.x - pos2.x).abs() + 1) * ((pos1.y - pos2.y).abs() + 1)
    }

    fn boundaries(pos1: &Vector, pos2: &Vector) -> [Vector; 2] {
        [
            Vector {
                x: pos1.x.min(pos2.x),
                y: pos1.y.min(pos2.y),
            },
            Vector {
                x: pos1.x.max(pos2.x) + 1,
                y: pos1.y.max(pos2.y) + 1,
            },
        ]
    }

    pub fn solve_part1(&self) -> usize {
        let mut max_area: i64 = 0;
        for i in 0..self.red_tiles.len() - 1 {
            for j in (i + 1)..self.red_tiles.len() {
                let area = Self::area(&self.red_tiles[i], &self.red_tiles[j]);
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area as usize
    }

    fn flood_fill(&self, start_pos: &Vector, grid: &mut Grid<Cell>) {
        use std::collections::VecDeque;
        let mut stack = VecDeque::new();
        stack.push_back(*start_pos);

        let mut closed = true;
        let mut visited = HashSet::new();

        while let Some(current) = stack.pop_back() {
            if current.x < 0
                || current.x >= grid.size().x
                || current.y < 0
                || current.y >= grid.size().y
            {
                closed = false;
                continue;
            }

            if visited.contains(&current) {
                continue;
            }

            match grid.get(&current) {
                Some(Cell::Empty) | Some(Cell::Occupied) => continue,
                Some(Cell::Unknown) => {
                    visited.insert(current);
                    for dir in &DIRECTIONS {
                        let neighbor = current + *dir;
                        stack.push_back(neighbor);
                    }
                }
                None => {
                    // Out of bounds
                    closed = false;
                    continue;
                }
            }
        }

        let cell_value = if closed { Cell::Occupied } else { Cell::Empty };
        for pos in visited {
            grid[&pos] = cell_value.clone();
        }
    }

    fn compress(pos: &Vector, x_values: &Vec<i64>, y_values: &Vec<i64>) -> Vector {
        Vector {
            x: x_values.iter().position(|&x| x == pos.x).unwrap() as i64,
            y: y_values.iter().position(|&y| y == pos.y).unwrap() as i64,
        }
    }

    pub fn solve_part2(&self) -> usize {
        // Compress positions to reduce complexity
        let x_values: HashSet<i64> = HashSet::from_iter(self.red_tiles.iter().map(|pos| pos.x));
        let mut x_values: Vec<i64> = x_values.into_iter().collect();
        x_values.sort();

        let y_values: HashSet<i64> = HashSet::from_iter(self.red_tiles.iter().map(|pos| pos.y));
        let mut y_values: Vec<i64> = y_values.into_iter().collect();
        y_values.sort();

        let compressed_grid_size = Vector {
            x: x_values.len() as i64,
            y: y_values.len() as i64,
        };

        let mut compressed_grid = Grid::with_capacity(compressed_grid_size, Cell::Unknown);

        // Add lines to the grid
        for i in 0..self.red_tiles.len() - 1 {
            for j in (i + 1)..self.red_tiles.len() {
                let pos1 = Self::compress(&self.red_tiles[i], &x_values, &y_values);
                let pos2 = Self::compress(&self.red_tiles[j], &x_values, &y_values);
                compressed_grid[&pos1] = Cell::Occupied;
                compressed_grid[&pos2] = Cell::Occupied;

                if pos1.x == pos2.x {
                    // Vertical line
                    let x = pos1.x;
                    for y in pos1.y.min(pos2.y)..=pos1.y.max(pos2.y) {
                        compressed_grid[&Vector { x, y }] = Cell::Occupied;
                    }
                } else if pos1.y == pos2.y {
                    // Horizontal line
                    let y = pos1.y;
                    for x in pos1.x.min(pos2.x)..=pos1.x.max(pos2.x) {
                        compressed_grid[&Vector { x, y }] = Cell::Occupied;
                    }
                }
            }
        }

        for x in 0..compressed_grid.size().x {
            for y in 0..compressed_grid.size().y {
                let start_pos = Vector { x, y };
                if let Some(Cell::Unknown) = compressed_grid.get(&start_pos) {
                    self.flood_fill(&start_pos, &mut compressed_grid);
                }
            }
        }

        // Find largest area with red corners and only "occupied" tiles
        let mut max_area = 0;
        for i in 0..self.red_tiles.len() - 1 {
            for j in (i + 1)..self.red_tiles.len() {
                let pos1 = &self.red_tiles[i];
                let pos2 = &self.red_tiles[j];
                let area = Self::area(&pos1, &pos2);
                if area < max_area {
                    continue;
                }

                let pos1 = Self::compress(pos1, &x_values, &y_values);
                let pos2 = Self::compress(pos2, &x_values, &y_values);
                let boundaries = Self::boundaries(&pos1, &pos2);
                let mut all_occupied = true;
                for x in boundaries[0].x..boundaries[1].x {
                    for y in boundaries[0].y..boundaries[1].y {
                        let compressed_pos = Vector { x, y };
                        if let Some(Cell::Empty) | Some(Cell::Unknown) =
                            compressed_grid.get(&compressed_pos)
                        {
                            all_occupied = false;
                            break;
                        }
                    }
                }

                if all_occupied {
                    max_area = area;
                }
            }
        }

        max_area as usize
    }
}

fn parse_input(raw_data: &str) -> Vec<Vector> {
    let rows = raw_data
        .lines()
        .map(|line| {
            let parts = line.split(',').collect::<Vec<&str>>();
            Vector {
                x: parts[0].parse::<i64>().unwrap(),
                y: parts[1].parse::<i64>().unwrap(),
            }
        })
        .collect::<Vec<Vector>>();

    rows
}

fn main() {
    let raw_data = include_str!("../input/input-09.txt");
    let grid = parse_input(raw_data);
    let solver = Solver::new(grid);

    println!("Solver - day 09:");

    // Part 1
    let result = solver.solve_part1();
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let result = solver.solve_part2();
    println!("  Part 2 - Final code: {}", result);
    // 4582310446
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn check_part1_result() {
        let grid = parse_input(RAW_INPUT);
        let solver = Solver::new(grid);
        let result = solver.solve_part1();
        assert_eq!(result, 50);
    }

    #[test]
    fn check_part2_result() {
        let grid = parse_input(RAW_INPUT);
        let solver = Solver::new(grid);
        let result = solver.solve_part2();

        assert_eq!(result, 24);
    }
}
