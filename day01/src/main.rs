enum Move {
    Left(i32),
    Right(i32),
}

struct Cursor {
    target: i32,
    size: i32,
}

fn modulo(value: i32, n: i32) -> i32 {
    (value % n + n) % n
}

impl Cursor {
    fn new(value: i32, size: i32) -> Self {
        Cursor {
            target: value,
            size,
        }
    }

    fn mv(&mut self, mv: &Move) {
        match mv {
            Move::Left(steps) => self.target = modulo(self.target - steps, self.size),
            Move::Right(steps) => self.target = modulo(self.target + steps, self.size),
        }
    }

    fn mv_with_zero_count(&mut self, mv: &Move) -> i32 {
        match mv {
            Move::Left(steps) => {
                // "Flip" direction to reuse the right move logic
                let flipped_target = modulo(self.size - self.target, self.size);
                let zero_count = (flipped_target + *steps) / self.size;
                self.target = modulo(self.target - *steps, self.size);

                zero_count
            }
            Move::Right(steps) => {
                let next_target = self.target + *steps;
                let zero_count = next_target / self.size;
                self.target = modulo(next_target, self.size);

                zero_count
            }
        }
    }
}

fn parse_input(raw_data: &str) -> Vec<Move> {
    raw_data
        .lines()
        .map(|line| {
            let (dir, count) = line.split_at(1);
            let steps = count.trim().parse::<i32>().unwrap();
            match dir {
                "L" => Move::Left(steps),
                "R" => Move::Right(steps),
                _ => panic!("Unknown direction"),
            }
        })
        .collect()
}

fn solve_part1(cursor: &mut Cursor, all_moves: &Vec<Move>) -> usize {
    let mut code = 0;
    for mv in all_moves {
        cursor.mv(mv);
        if cursor.target == 0 {
            code += 1;
        }
    }

    code
}

fn solve_part2(cursor: &mut Cursor, all_moves: &Vec<Move>) -> i32 {
    let mut code = 0;
    for mv in all_moves {
        code += cursor.mv_with_zero_count(mv);
    }

    code
}

fn main() {
    let raw_data = include_str!("../input/input-01.txt");
    let all_moves = parse_input(raw_data);

    println!("Solver - day 01:");

    // Part 1
    let mut cursor = Cursor::new(50, 100);
    let result = solve_part1(&mut cursor, &all_moves);
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let mut cursor = Cursor::new(50, 100);
    let result = solve_part2(&mut cursor, &all_moves);
    println!("  Part 2 - Final code: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn check_part1_result() {
        let mut cursor = Cursor::new(50, 100);
        let all_moves = parse_input(RAW_INPUT);
        let result = solve_part1(&mut cursor, &all_moves);
        assert_eq!(result, 3);
    }

    #[test]
    fn check_part2_result() {
        let mut cursor = Cursor::new(50, 100);
        let all_moves = parse_input(RAW_INPUT);
        let result = solve_part2(&mut cursor, &all_moves);
        assert_eq!(result, 6);
    }
}
