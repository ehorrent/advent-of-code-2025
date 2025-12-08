use std::collections::HashSet;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

struct Connection {
    box_id1: usize,
    box_id2: usize,
    squared_distance: i64,
}

impl Connection {
    fn new(box_id1: usize, box_id2: usize, squared_distance: i64) -> Self {
        Connection {
            box_id1,
            box_id2,
            squared_distance,
        }
    }
}

impl Vector3 {
    fn squared_distance(&self, other: &Vector3) -> i64 {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;
        let delta_z = self.z - other.z;
        delta_x * delta_x + delta_y * delta_y + delta_z * delta_z
    }
}

enum Part {
    Part1(usize),
    Part2,
}

struct Solver {
    boxes: Vec<Vector3>,
}

impl Solver {
    fn new(boxes: Vec<Vector3>) -> Self {
        Solver { boxes }
    }

    pub fn solve(&mut self, part: Part) -> usize {
        // Compute all possible connections and the corresponding pairwise squared distance
        let mut connections: Vec<Connection> = vec![];
        for i in 0..self.boxes.len() - 1 {
            for j in (i + 1)..self.boxes.len() {
                let dist = self.boxes[i].squared_distance(&self.boxes[j]);
                connections.push(Connection::new(i, j, dist));
            }
        }

        // Sort by distances
        connections.sort_by(|a, b| a.squared_distance.cmp(&b.squared_distance));

        let connection_count = match part {
            Part::Part1(count) => count,
            Part::Part2 => connections.len(),
        };

        let mut circuits: Vec<HashSet<usize>> = vec![];
        let mut connected_circuits: Vec<usize> = vec![];
        for connection in &connections[0..connection_count] {
            // Check if there is already some circuits containing either of the boxes
            connected_circuits.clear();
            for (index, circuit) in circuits.iter().enumerate() {
                if circuit.contains(&connection.box_id1) || circuit.contains(&connection.box_id2) {
                    connected_circuits.push(index);
                }
            }

            if connected_circuits.len() > 1 {
                // Connection created between multiple circuits -> merge them
                let mut merged_circuit = HashSet::from([connection.box_id1, connection.box_id2]);
                connected_circuits.sort();
                connected_circuits.reverse();
                for group_index in &connected_circuits {
                    let circuit = &circuits[*group_index];
                    merged_circuit.extend(circuit);
                    circuits.remove(*group_index);
                }

                circuits.push(merged_circuit);
            } else if connected_circuits.len() == 1 {
                // Add connection to the existing circuit
                let group_index = connected_circuits[0];
                let circuit = &mut circuits[group_index];
                circuit.insert(connection.box_id1);
                circuit.insert(connection.box_id2);
            } else {
                // Create a new circuit
                let new_group = HashSet::from([connection.box_id1, connection.box_id2]);
                circuits.push(new_group);
            }

            if let Part::Part2 = part {
                // Check if we have a single circuit containing all boxes
                if let [circuit] = &circuits[..]
                    && circuit.len() == self.boxes.len()
                {
                    let pos1 = &self.boxes[connection.box_id1];
                    let pos2 = &self.boxes[connection.box_id2];
                    return (pos1.x * pos2.x) as usize;
                }
            }
        }

        if let Part::Part1(_) = part {
            circuits.sort_by(|h1, h2| h1.len().cmp(&h2.len()));
            return circuits.iter().rev().take(3).map(|g| g.len()).product();
        }

        panic!("No solution found");
    }

    pub fn solve_part1(&mut self, connection_count: usize) -> usize {
        self.solve(Part::Part1(connection_count))
    }

    pub fn solve_part2(&mut self) -> usize {
        self.solve(Part::Part2)
    }
}

fn parse_input(raw_data: &str) -> Vec<Vector3> {
    raw_data
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|num| num.trim().parse().unwrap())
                .collect();

            Vector3 {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect()
}

fn main() {
    let raw_data = include_str!("../input/input-08.txt");
    let junctions = parse_input(raw_data);
    let mut solver = Solver::new(junctions);

    println!("Solver - day 08:");

    // Part 1
    let result = solver.solve_part1(1000);
    println!("  Part 1 - Final code: {}", result);

    // Part 2
    let result = solver.solve_part2();
    println!("  Part 2 - Final code: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn check_part1_result() {
        let junctions = parse_input(RAW_INPUT);
        let mut solver = Solver::new(junctions);
        let result = solver.solve_part1(10);
        assert_eq!(result, 40);
    }

    #[test]
    fn check_part2_result() {
        let junctions = parse_input(RAW_INPUT);
        let mut solver = Solver::new(junctions);
        let result = solver.solve_part2();

        assert_eq!(result, 25272);
    }
}
