use microlp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem};
use regex::Regex;
use std::collections::HashMap;

struct JoltageMachine {
    expected_joltage: Vec<usize>,
    buttons: Vec<Vec<usize>>,
}

struct LightMachine {
    expected_lights: usize,
    buttons: Vec<usize>,
}

struct LightContext<'a> {
    light_state: usize,
    step: usize,
    best_step_count: &'a mut Option<usize>,
    state_cost: &'a mut HashMap<usize, usize>,
}

impl LightMachine {
    fn toggle(&self, button_mask: usize, light_state: usize) -> usize {
        light_state ^ button_mask
    }

    fn solve_rec(&self, context: LightContext) {
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

            let next_context = LightContext {
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
        self.solve_rec(LightContext {
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

fn parse_input(raw_data: &str) -> (Vec<LightMachine>, Vec<JoltageMachine>) {
    // I'm lazy...
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

            let buttons: Vec<Vec<usize>> = buttons_re
                .captures_iter(line)
                .map(|cap| {
                    cap[1]
                        .split(',')
                        .map(|s| s.trim().parse().unwrap())
                        .collect::<Vec<usize>>()
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

            (
                LightMachine {
                    expected_lights,
                    buttons: buttons
                        .iter()
                        .map(|button| button.iter().map(|index| 1 << *index).sum())
                        .collect(),
                },
                JoltageMachine {
                    expected_joltage: joltages,
                    buttons,
                },
            )
        })
        .collect()
}

fn solve_part1(machines: &Vec<LightMachine>) -> usize {
    let mut total = 0;
    for machine in machines.iter() {
        total += machine.solve();
    }

    total
}

fn solve_part2(machines: &Vec<JoltageMachine>) -> usize {
    let mut total = 0;

    for machine in machines.iter() {
        let mut problem = Problem::new(OptimizationDirection::Minimize);

        // Add variables representing button presses
        let mut press_count_vars = Vec::new();
        for button in &machine.buttons {
            press_count_vars.push(
                problem.add_integer_var(
                    1.0,
                    (
                        0,
                        // Get the max possible number of presses for this button
                        button
                            .iter()
                            .map(|index| machine.expected_joltage[*index] as i32)
                            .min()
                            .unwrap(),
                    ),
                ),
            );
        }

        // Create expression for each expected joltage
        for (joltage_index, expected_joltage) in machine.expected_joltage.iter().enumerate() {
            let mut expression = LinearExpr::empty();
            for (button_index, button) in machine.buttons.iter().enumerate() {
                if button.contains(&joltage_index) {
                    let press_count_var = press_count_vars[button_index];
                    expression.add(press_count_var, 1_f64);
                }
            }

            problem.add_constraint(expression, ComparisonOp::Eq, *expected_joltage as f64);
        }

        // Solve the "press count" problem
        let solution = problem.solve().unwrap();
        total += solution.objective().round() as usize;
    }

    total
}

fn main() {
    let raw_data = include_str!("../input/input-10.txt");
    let (light_machines, joltage_machines) = parse_input(raw_data);

    println!("Solver - day 10:");

    // Part 1
    let result = solve_part1(&light_machines);
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let result = solve_part2(&joltage_machines);
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
        let (light_machines, _) = parse_input(RAW_INPUT);
        let result = solve_part1(&light_machines);
        assert_eq!(result, 7);
    }

    #[test]
    fn check_part2_result() {
        let (_, joltage_machines) = parse_input(RAW_INPUT);
        let result = solve_part2(&joltage_machines);
        assert_eq!(result, 33);
    }
}
