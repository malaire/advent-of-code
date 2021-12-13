use std::collections::HashSet;

use malaire_aoc::{parse_numbers, run};

static INPUT_A: &str = include_str!("input_a");
static INPUT_X: &str = include_str!("input");

static OUTPUT_A: &str = "#####\n#...#\n#...#\n#...#\n#####\n";
static OUTPUT_X: &str = include_str!("output");

fn main() {
    run(0, solve_1, INPUT_A, 17);
    run(1, solve_1, INPUT_X, 592);

    run(0, solve_2, INPUT_A, OUTPUT_A.to_string());
    run(2, solve_2, INPUT_X, OUTPUT_X.to_string());
}

fn solve_1(input: &str) -> usize {
    solve_generic(input, true).iter().count()
}

fn solve_2(input: &str) -> String {
    let paper = solve_generic(input, false);

    let max_x: usize = *paper.iter().map(|(x, _)| x).max().unwrap();
    let max_y: usize = *paper.iter().map(|(_, y)| y).max().unwrap();

    let mut code = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.contains(&(x, y)) {
                code.push('#');
            } else {
                code.push('.');
            }
        }
        code.push('\n');
    }
    code
}

fn solve_generic(input: &str, single_fold: bool) -> HashSet<(usize, usize)> {
    let sections: Vec<_> = input.split("\n\n").collect();

    let mut paper = HashSet::new();

    for xy in parse_numbers::<usize>(sections[0]).chunks(2) {
        paper.insert((xy[0], xy[1]));
    }

    for line in sections[1].lines() {
        let axis_is_x = line.contains('x');
        let pos: usize = parse_numbers(line)[0];

        for (x, y) in paper.clone() {
            if axis_is_x {
                if x > pos {
                    paper.insert((2 * pos - x, y));
                    paper.remove(&(x, y));
                }
            } else {
                if y > pos {
                    paper.insert((x, 2 * pos - y));
                    paper.remove(&(x, y));
                }
            }
        }

        if single_fold {
            break;
        }
    }

    paper
}
