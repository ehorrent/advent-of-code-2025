use shared::{Grid, Vector};
use std::collections::{HashMap, HashSet};

struct Region {
    size: Vector,
    shape_count_by_id: HashMap<usize, usize>,
}

const SHAPE_SIZE: Vector = Vector { x: 3, y: 3 };

struct Shape {
    occupied_count: usize,       // Number of occupied cells
    shapes: HashSet<Grid<bool>>, // All combinations (rotated/flipped)
}

impl Shape {
    fn new(shape: Grid<bool>) -> Self {
        let mut occupied_count = 0;
        for y in 0..SHAPE_SIZE.y {
            for x in 0..SHAPE_SIZE.x {
                if shape[Vector { x, y }] {
                    occupied_count += 1;
                }
            }
        }

        // Rotate and flip to get all possible variants
        let mut variants = HashSet::new();
        let mut last_shape = shape;
        for _ in 0..4 {
            let rotated = Self::rotate(&last_shape);
            let flipped = Self::flip(&rotated);
            variants.insert(rotated.clone());
            variants.insert(flipped);

            last_shape = rotated;
        }

        Shape {
            occupied_count,
            shapes: variants,
        }
    }

    fn flip(shape: &Grid<bool>) -> Grid<bool> {
        let mut new_shape = Grid::with_capacity(SHAPE_SIZE, false);

        // Flip on Y axis
        for y in 0..SHAPE_SIZE.y {
            for x in 0..SHAPE_SIZE.x {
                let value = shape[Vector { x, y }];
                let new_pos = Vector {
                    x: SHAPE_SIZE.x - 1 - x,
                    y,
                };

                new_shape[new_pos] = value;
            }
        }

        new_shape
    }

    fn rotate(shape: &Grid<bool>) -> Grid<bool> {
        let mut new_shape = Grid::with_capacity(SHAPE_SIZE, false);

        for y in 0..SHAPE_SIZE.y {
            for x in 0..SHAPE_SIZE.x {
                let value = shape[Vector { x, y }];
                let new_pos = Vector {
                    x: SHAPE_SIZE.y - 1 - y,
                    y: x,
                };

                new_shape[new_pos] = value;
            }
        }

        new_shape
    }
}

fn parse_input(raw_data: &str) -> (Vec<Region>, HashMap<usize, Shape>) {
    let shapes_re = regex::Regex::new(r"(\d+):\n((?:[.#]+\n?)+)").unwrap();
    let regions_re = regex::Regex::new(r"(\d+x\d+): ((?:\d+ )+\d+)").unwrap();

    let shapes_by_id: HashMap<usize, Shape> = shapes_re
        .captures_iter(raw_data)
        .map(|cap| {
            let id: usize = cap[1].parse().unwrap();
            let shape_lines: Vec<&str> = cap[2].trim().lines().collect();
            let mut grid = Grid::with_capacity(SHAPE_SIZE, false);
            for (y, line) in shape_lines.iter().enumerate() {
                for (x, ch) in line.chars().enumerate() {
                    let pos = Vector {
                        x: x as i64,
                        y: y as i64,
                    };
                    grid[pos] = ch == '#';
                }
            }

            (id, Shape::new(grid))
        })
        .collect();

    let regions = regions_re
        .captures_iter(raw_data)
        .map(|cap| {
            let parts: Vec<&str> = cap[1].split('x').collect();
            let region_size = Vector {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            };

            let shape_ids: HashMap<usize, usize> = cap[2]
                .trim()
                .split_whitespace()
                .enumerate()
                .map(|(index, s)| (index, s.parse().unwrap()))
                .filter(|(_, count)| *count > 0)
                .collect();

            Region {
                size: region_size,
                shape_count_by_id: shape_ids,
            }
        })
        .collect();

    (regions, shapes_by_id)
}

fn try_put_shape(pos: &Vector, shape: &Grid<bool>, grid: &Grid<bool>) -> Option<Grid<bool>> {
    let mut next_grid = grid.clone();
    for x in 0..SHAPE_SIZE.x {
        for y in 0..SHAPE_SIZE.y {
            let relative_pos = Vector { x, y };
            let grid_pos = *pos + relative_pos;
            let grid_occupied = next_grid[grid_pos];
            let shape_occupied = shape[relative_pos];

            if shape_occupied {
                // Already occupied in the grid
                if grid_occupied {
                    return None;
                }

                // Mark position as occupied
                next_grid[grid_pos] = true;
            }
        }
    }

    Some(next_grid)
}

fn can_fit_in_region(region: &Region, shapes_by_id: &HashMap<usize, Shape>) -> bool {
    let region_area = (region.size.x * region.size.y) as usize;
    let mut total_occupied = 0;
    for (shape_id, shape_count) in &region.shape_count_by_id {
        let shape = &shapes_by_id[shape_id];
        total_occupied += shape.occupied_count * (*shape_count);
        if total_occupied > region_area {
            return false;
        }
    }

    true
}

fn solve_part1_rec(shape_count_by_id: &HashMap<usize, usize>, shapes_by_id: &HashMap<usize, Shape>, grid: &Grid<bool>) -> bool
{
    for (shape_id, shape_count) in shape_count_by_id {
        let shape = &shapes_by_id[&shape_id];
        if 0 == *shape_count
        {
            continue;
        }

        for x in 0..grid.size().x - SHAPE_SIZE.x + 1 {
            for y in 0..grid.size().y - SHAPE_SIZE.y + 1 {
                let pos = Vector { x, y };

                for shape_variant in &shape.shapes {
                    if let Some(next_grid) = try_put_shape(&pos, shape_variant, grid) {
                        // Successfully placed shape, continue with next shape
                        let mut next_shape_count_by_id = shape_count_by_id.clone();
                        let count = next_shape_count_by_id.get_mut(&shape_id).unwrap();
                        *count -= 1;
                        if *count == 0 {
                            next_shape_count_by_id.remove(&shape_id);
                        }

                        if next_shape_count_by_id.is_empty() {
                            return true; // All shapes placed
                        }

                        if solve_part1_rec(&next_shape_count_by_id, shapes_by_id, &next_grid) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

fn solve_part1(regions: &Vec<Region>, shapes_by_id: &HashMap<usize, Shape>, check_only_areas: bool) -> usize {
    let mut total = 0;

    for region in regions {
        if can_fit_in_region(region, shapes_by_id) {
            if check_only_areas {
                total += 1;
                continue;
            }

            // Try all combinations
            // This solution is killing my CPU even with the example input...
            let grid = Grid::with_capacity(region.size, false);
            if solve_part1_rec(&region.shape_count_by_id, shapes_by_id, &grid)
            {
                total += 1;
            }
        }
    }

    total
}

fn solve_part2() -> usize {
    0
}

fn main() {
    let raw_data = include_str!("../input/input-12.txt");
    let (regions, shapes_by_id) = parse_input(raw_data);

    println!("Solver - day 12:");

    // Part 1
    let result = solve_part1(&regions, &shapes_by_id, true);
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let result = solve_part2();
    println!("  Part 2 - Final code: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn check_part1_result() {
        let (regions, shapes_by_id) = parse_input(RAW_INPUT);
        let result = solve_part1(&regions, &shapes_by_id, true);
        assert_eq!(result, 3);
    }

    // #[test]
    // fn check_part2_result() {
    //     let input = parse_input(RAW_INPUT);
    //     let input = parse_input(RAW_INPUT);
    //     let result = solve_part2(&input);
    //     assert_eq!(result, 2);
    // }
}
