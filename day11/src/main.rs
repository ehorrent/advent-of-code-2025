use std::collections::HashMap;

struct Device<'a> {
    name: &'a str,
    targets: Vec<&'a str>,
}

fn parse_input(raw_data: &str) -> Vec<Device<'_>> {
    let mut devices: Vec<Device> = raw_data
        .lines()
        .map(|line| {
            let name_target_split: Vec<&str> = line.split(": ").collect();
            let source_device = name_target_split[0];
            let target_devices = name_target_split[1].split(' ').collect::<Vec<&str>>();

            Device {
                name: source_device,
                targets: target_devices,
            }
        })
        .collect();

    devices.push(Device {
        name: "out",
        targets: vec![],
    });

    devices
}

fn count_paths_rec(
    device_name: &str,
    target_device_name: &str,
    devices: &Vec<Device>,
    count_cache: &mut HashMap<String, usize>,
) -> usize {
    // Found a path!
    if device_name == target_device_name {
        return 1;
    }

    // Already processed
    if let Some(&count) = count_cache.get(device_name) {
        return count;
    }

    // Recursion over next devices
    let device = devices.iter().find(|d| d.name == device_name).unwrap();
    let mut count = 0;
    for next_device_name in &device.targets {
        count += count_paths_rec(next_device_name, target_device_name, devices, count_cache);
    }

    count_cache.insert(device_name.to_string(), count);
    count
}

fn count_paths(start_device_name: &str, path: &[&str], devices: &Vec<Device>) -> usize {
    let mut current_device_name = start_device_name;

    let mut total = 1;
    for target_device_name in path {
        let paths_count = count_paths_rec(
            current_device_name,
            target_device_name,
            devices,
            &mut HashMap::new(),
        );
        if paths_count == 0 {
            return 0;
        }

        total *= paths_count;
        current_device_name = target_device_name;
    }

    total
}

fn solve_part1(devices: &Vec<Device>) -> usize {
    count_paths_rec("you", "out", devices, &mut HashMap::new())
}

fn solve_part2(devices: &Vec<Device>) -> usize {
    count_paths("svr", &["dac", "fft", "out"], devices)
        + count_paths("svr", &["fft", "dac", "out"], devices)
}

fn main() {
    let raw_data = include_str!("../input/input-11.txt");
    let input = parse_input(raw_data);

    println!("Solver - day 11:");

    // Part 1
    let result = solve_part1(&input);
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let result = solve_part2(&input);
    println!("  Part 2 - Final code: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_result() {
        let raw_data = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        let input = parse_input(raw_data);
        let result = solve_part1(&input);
        assert_eq!(result, 5);
    }

    #[test]
    fn check_part2_result() {
        let raw_data = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        let input = parse_input(raw_data);
        let result = solve_part2(&input);
        assert_eq!(result, 2);
    }
}
