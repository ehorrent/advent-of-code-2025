use regex::Regex;
use std::collections::HashMap;

struct Machine {
    expected_lights: usize,
    buttons: Vec<usize>,
    _joltages: Vec<usize>,
}

struct Context<'a> {
    light_state: usize,
    step: usize,
    best_step_count: &'a mut Option<usize>,
    state_cost: &'a mut HashMap<usize, usize>,
}

impl Machine {
    fn toggle(&self, button_mask: usize, light_state: usize) -> usize {
        light_state ^ button_mask
    }

    fn solve_rec(&self, context: Context) {
        // Check if we already reached this state with less steps
        if let Some(cost) = context.state_cost.get(&context.light_state)
            && *cost < context.step
        {
            return;
        }

        // Max depth reached
        if context.step >= 10 {
            return;
        }

        // Too many steps already
        if let Some(best) = context.best_step_count {
            if context.step >= *best {
                return;
            }
        }

        context.state_cost.insert(context.light_state, context.step);

        // Expected light state !
        if context.light_state == self.expected_lights {
            *context.best_step_count = Some(context.step);
            return;
        }

        for button in self.buttons.iter() {
            let next_state = self.toggle(*button, context.light_state);

            let next_context = Context {
                light_state: next_state,
                step: context.step + 1,
                best_step_count: context.best_step_count,
                state_cost: context.state_cost,
            };

            self.solve_rec(next_context);
        }
    }

    fn solve(&self) -> usize {
        let mut best_step_count = None;
        let mut state_cost = HashMap::new();
        self.solve_rec(Context {
            light_state: 0,
            step: 0,
            best_step_count: &mut best_step_count,
            state_cost: &mut state_cost,
        });

        if let Some(best_step_count) = best_step_count {
            return best_step_count;
        }

        panic!("No solution found");
    }
}

fn parse_input(raw_data: &str) -> Vec<Machine> {
    let lights_re = Regex::new(r"\[(.*?)\]").unwrap();
    let buttons_re = Regex::new(r"\((.*?)\)").unwrap();
    let joltages_re = Regex::new(r"\{(.*?)\}").unwrap();

    raw_data
        .lines()
        .map(|line| {
            let capture = lights_re.captures_iter(line).next().unwrap();
            let expected_lights: usize = capture[1]
                .chars()
                .enumerate()
                .map(|(index, c)| match c {
                    '.' => 0,
                    '#' => 1 << index,
                    _ => panic!("Unexpected character in lights"),
                })
                .sum();

            let buttons: Vec<usize> = buttons_re
                .captures_iter(line)
                .map(|cap| {
                    cap[1]
                        .split(',')
                        .map(|s| {
                            let index: usize = s.trim().parse().unwrap();
                            1 << index
                        })
                        .sum()
                })
                .collect();

            let joltages: Vec<usize> = joltages_re
                .captures_iter(line)
                .flat_map(|cap| {
                    cap[1]
                        .split(',')
                        .map(|s| s.trim().parse().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect();

            Machine {
                expected_lights,
                buttons,
                _joltages: joltages,
            }
        })
        .collect()
}

fn solve_part1(machines: &Vec<Machine>) -> usize {
    let mut total = 0;
    for machine in machines.iter() {
        total += machine.solve();
    }

    total
}

fn solve_part2() -> i32 {
    0
}

fn main() {
    let raw_data = include_str!("../input/input-10.txt");
    let input = parse_input(raw_data);

    println!("Solver - day 10:");

    // Part 1
    let result = solve_part1(&input);
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let result = solve_part2();
    println!("  Part 2 - Final code: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn check_part1_result() {
        let input = parse_input(RAW_INPUT);
        let result = solve_part1(&input);
        assert_eq!(result, 7);
    }

    // #[test]
    // fn check_part2_result() {
    //     let input = parse_input(RAW_INPUT);
    //     let result = solve_part2();
    //     assert_eq!(result, 6);
    // }
}
