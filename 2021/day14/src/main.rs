use std::collections::HashMap;

use malaire_aoc::run;

static INPUT_A: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 1588);
    run(1, solve_1, INPUT_X, 3058);

    run(0, solve_2, INPUT_A, 2188189693529);
    run(2, solve_2, INPUT_X, 3447389044530);
}

fn solve_1(input: &str) -> usize {
    solve_generic(input, 10)
}

fn solve_2(input: &str) -> usize {
    solve_generic(input, 40)
}

fn solve_generic(input: &str, steps: usize) -> usize {
    let lines: Vec<_> = input.lines().collect();

    let polymer_template = lines[0].as_bytes().to_vec();

    let mut rules = HashMap::new();
    for line in &lines[2..] {
        let line = line.as_bytes();
        rules.insert((line[0], line[1]), line[6]);
    }

    let mut counts = [0; 256];
    for &x in &polymer_template {
        counts[x as usize] += 1;
    }

    let mut memoized = HashMap::new();

    for pair in polymer_template.windows(2) {
        let new_counts = insert(pair[0], pair[1], steps, &rules, &mut memoized);
        for n in 0..256 {
            counts[n] += new_counts[n];
        }
    }

    counts.iter().max().unwrap() - counts.iter().filter(|x| **x > 0).min().unwrap()
}

fn insert(
    a: u8,
    b: u8,
    steps: usize,
    rules: &HashMap<(u8, u8), u8>,
    memoized: &mut HashMap<(u8, u8, usize), [usize; 256]>,
) -> [usize; 256] {
    if let Some(counts) = memoized.get(&(a, b, steps)) {
        *counts
    } else {
        let mut counts = [0; 256];
        if let Some(&x) = rules.get(&(a, b)) {
            counts[x as usize] += 1;
            if steps > 1 {
                let counts_ax = insert(a, x, steps - 1, rules, memoized);
                let counts_xb = insert(x, b, steps - 1, rules, memoized);
                for n in 0..256 {
                    counts[n] += counts_ax[n] + counts_xb[n];
                }
            }
        }
        memoized.insert((a, b, steps), counts);
        counts
    }
}
