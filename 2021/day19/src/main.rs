use std::collections::{HashMap, HashSet};

use malaire_aoc::run;

static INPUT_A: &str = include_str!("input_a");
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 79);
    run(1, solve_1, INPUT_X, 465);

    run(0, solve_2, INPUT_A, 3621);
    run(2, solve_2, INPUT_X, 12149);
}

struct Scanner {
    // relative to scanner 0
    pos: (i64, i64, i64),
    // relative to scanner 0
    beacons: Vec<(i64, i64, i64)>,
}

fn solve_1(input: &str) -> usize {
    let scanners = solve_scanners(input);

    let mut all_beacons = HashSet::new();
    for scanner in scanners.values() {
        for &beacon in &scanner.beacons {
            all_beacons.insert(beacon);
        }
    }
    all_beacons.len()
}

fn solve_2(input: &str) -> i64 {
    let scanners = solve_scanners(input);

    let mut max_distance = 0;
    for a in 0..scanners.len() {
        for b in a..scanners.len() {
            let pa = scanners[&a].pos;
            let pb = scanners[&b].pos;
            let distance = (pa.0 - pb.0).abs() + (pa.1 - pb.1).abs() + (pa.2 - pb.2).abs();
            max_distance = std::cmp::max(max_distance, distance);
        }
    }
    max_distance
}

const ROTATIONS: [fn((i64, i64, i64)) -> (i64, i64, i64); 24] = [
    // +x
    |(x, y, z)| (x, y, z),
    |(x, y, z)| (x, -y, -z),
    |(x, y, z)| (x, z, -y),
    |(x, y, z)| (x, -z, y),
    // -x
    |(x, y, z)| (-x, z, y),
    |(x, y, z)| (-x, -z, -y),
    |(x, y, z)| (-x, y, -z),
    |(x, y, z)| (-x, -y, z),
    // +y
    |(x, y, z)| (y, z, x),
    |(x, y, z)| (y, -z, -x),
    |(x, y, z)| (y, x, -z),
    |(x, y, z)| (y, -x, z),
    // -y
    |(x, y, z)| (-y, x, z),
    |(x, y, z)| (-y, -x, -z),
    |(x, y, z)| (-y, z, -x),
    |(x, y, z)| (-y, -z, x),
    // +z
    |(x, y, z)| (z, x, y),
    |(x, y, z)| (z, -x, -y),
    |(x, y, z)| (z, y, -x),
    |(x, y, z)| (z, -y, x),
    // -z
    |(x, y, z)| (-z, y, x),
    |(x, y, z)| (-z, -y, -x),
    |(x, y, z)| (-z, x, -y),
    |(x, y, z)| (-z, -x, y),
];

fn solve_scanners(input: &str) -> HashMap<usize, Scanner> {
    let sections: Vec<_> = input.split("\n\n").collect();

    let mut beacons_of_scanner = Vec::new();
    for section in sections {
        let mut beacons = Vec::new();
        for line in section.lines().skip(1) {
            let parts: Vec<_> = line.split(",").collect();
            let x: i64 = parts[0].parse().unwrap();
            let y: i64 = parts[1].parse().unwrap();
            let z: i64 = parts[2].parse().unwrap();
            beacons.push((x, y, z));
        }
        beacons_of_scanner.push(beacons);
    }
    let scanner_count = beacons_of_scanner.len();

    let mut solved_scanners: HashMap<usize, Scanner> = HashMap::new();
    solved_scanners.insert(
        0,
        Scanner {
            pos: (0, 0, 0),
            beacons: beacons_of_scanner[0].clone(),
        },
    );

    let mut base_ids_todo = vec![0];

    while solved_scanners.len() != scanner_count {
        let base_id = base_ids_todo.pop().unwrap();
        let base_beacons = solved_scanners[&base_id].beacons.clone();

        for other_id in 0..scanner_count {
            if !solved_scanners.contains_key(&other_id) {
                let other_beacons = &beacons_of_scanner[other_id];

                for rotation in ROTATIONS {
                    if let Some(((dx, dy, dz), beacons)) =
                        try_match_12(&base_beacons, other_beacons, rotation)
                    {
                        let beacons: Vec<_> = beacons
                            .iter()
                            .map(|(x, y, z)| (x - dx, y - dy, z - dz))
                            .collect();

                        solved_scanners.insert(
                            other_id,
                            Scanner {
                                pos: (-dx, -dy, -dz),
                                beacons: beacons,
                            },
                        );

                        base_ids_todo.push(other_id);
                    }
                }
            }
        }
    }

    solved_scanners
}

fn try_match_12<F>(
    base_beacons: &[(i64, i64, i64)],
    other_beacons: &[(i64, i64, i64)],
    rotation: F,
) -> Option<((i64, i64, i64), HashSet<(i64, i64, i64)>)>
where
    F: Fn((i64, i64, i64)) -> (i64, i64, i64),
{
    let other_beacons_rotated: HashSet<_> = other_beacons.iter().map(|&p| rotation(p)).collect();

    for a in base_beacons {
        for b in &other_beacons_rotated {
            let dx = b.0 - a.0;
            let dy = b.1 - a.1;
            let dz = b.2 - a.2;

            let mut matches = 0;
            for (x, y, z) in base_beacons {
                if other_beacons_rotated.contains(&(x + dx, y + dy, z + dz)) {
                    matches += 1;
                }
            }

            if matches >= 12 {
                return Some(((dx, dy, dz), other_beacons_rotated));
            }
        }
    }

    None
}
