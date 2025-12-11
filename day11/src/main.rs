struct Device {
    name: String,
    targets: Vec<String>,
}

fn parse_input(raw_data: &str) -> Vec<Device> {
    raw_data
        .lines()
        .map(|line| {
            let mut name_target_split = line.split(": ");

            let source_device = name_target_split.nth(0).unwrap();
            let target_devices = name_target_split
                .nth(0)
                .unwrap()
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            Device {
                name: source_device.to_string(),
                targets: target_devices,
            }
        })
        .collect()
}

fn solve_part1() -> usize {
    0
}

fn solve_part2() -> usize {
    0
}

fn main() {
    let raw_data = include_str!("../input/input-11.txt");
    let input = parse_input(raw_data);

    println!("Solver - day 10:");

    // Part 1
    let result = solve_part1();
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let result = solve_part2();
    println!("  Part 2 - Final code: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn check_part1_result() {
        let input = parse_input(RAW_INPUT);
        let result = solve_part1();
        assert_eq!(result, 5);
    }

    // #[test]
    // fn check_part2_result() {
    //     let input = parse_input(RAW_INPUT);
    //     let result = solve_part2();
    //     assert_eq!(result, 6);
    // }
}
