use std::collections::{HashMap, HashSet};

use malaire_aoc::run;

static INPUT_A: &str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
static INPUT_B: &str =
    "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
static INPUT_C: &str = include_str!("input_c");
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 10);
    run(0, solve_1, INPUT_B, 19);
    run(0, solve_1, INPUT_C, 226);
    run(1, solve_1, INPUT_X, 4338);

    run(0, solve_2, INPUT_A, 36);
    run(0, solve_2, INPUT_B, 103);
    run(0, solve_2, INPUT_C, 3509);
    run(2, solve_2, INPUT_X, 114189);
}

fn solve_1(input: &str) -> usize {
    visit_all_1("start", &mut HashSet::new(), &read_input(input))
}

fn visit_all_1(
    current: &str,
    visited: &mut HashSet<String>,
    connections: &HashMap<String, HashSet<String>>,
) -> usize {
    if current == "end" {
        return 1;
    }

    let mut path_count = 0;
    for next in connections.get(current).unwrap() {
        let next_is_big = next.as_bytes()[0].is_ascii_uppercase();

        if next != "start" && (next_is_big || !visited.contains(next)) {
            if !next_is_big {
                visited.insert(next.to_string());
            }
            path_count += visit_all_1(next, visited, connections);
            visited.remove(next);
        }
    }
    path_count
}

fn solve_2(input: &str) -> usize {
    visit_all_2("start", &mut HashSet::new(), &read_input(input), false)
}

fn visit_all_2(
    current: &str,
    visited: &mut HashSet<String>,
    connections: &HashMap<String, HashSet<String>>,
    visited_twice: bool,
) -> usize {
    if current == "end" {
        return 1;
    }

    let mut path_count = 0;
    for next in connections.get(current).unwrap() {
        let next_is_big = next.as_bytes()[0].is_ascii_uppercase();

        if next != "start" && (next_is_big || !visited.contains(next) || !visited_twice) {
            if next_is_big {
                path_count += visit_all_2(next, visited, connections, visited_twice);
            } else if !visited.contains(next) {
                visited.insert(next.to_string());
                path_count += visit_all_2(next, visited, connections, visited_twice);
                visited.remove(next);
            } else {
                path_count += visit_all_2(next, visited, connections, true);
            }
        }
    }
    path_count
}

// ======================================================================
// UTIL

fn read_input(input: &str) -> HashMap<String, HashSet<String>> {
    let mut connections = HashMap::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split('-').collect();
        let a = parts[0].to_owned();
        let b = parts[1].to_owned();

        connections
            .entry(a.clone())
            .or_insert(HashSet::new())
            .insert(b.clone());
        connections.entry(b).or_insert(HashSet::new()).insert(a);
    }
    connections
}
