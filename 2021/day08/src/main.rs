use malaire_aoc::run;

static INPUT_A: &str =
    "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
static INPUT_B: &str = include_str!("input_b");
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 0);
    run(0, solve_1, INPUT_B, 26);
    run(1, solve_1, INPUT_X, 344);

    run(0, solve_2, INPUT_A, 5353);
    run(0, solve_2, INPUT_B, 61229);
    run(2, solve_2, INPUT_X, 1048410);
}

fn solve_1(input: &str) -> usize {
    let mut easy_count = 0;
    for line in input.lines() {
        let parts: Vec<_> = line.split(" | ").collect();
        let output_digits: Vec<_> = parts[1].split_whitespace().collect();

        for digit in output_digits {
            if digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7 {
                easy_count += 1;
            }
        }
    }
    easy_count
}

fn solve_2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let parts: Vec<_> = line.split(" | ").collect();
        let patterns: Vec<_> = parts[0].split_whitespace().collect();
        let output_digits: Vec<_> = parts[1].split_whitespace().collect();

        let mut patterns_by_len = vec![Vec::new(); 8];
        for pattern in patterns {
            patterns_by_len[pattern.len()].push(pattern.as_bytes());
        }

        let mut solved: Vec<Vec<u8>> = vec![Vec::new(); 10];

        // LEN 2: 1
        // LEN 3: 7
        // LEN 4: 4
        // LEN 7: 8

        solved[1] = patterns_by_len[2][0].to_vec();
        solved[4] = patterns_by_len[4][0].to_vec();
        solved[7] = patterns_by_len[3][0].to_vec();
        solved[8] = patterns_by_len[7][0].to_vec();

        // LEN 6:  0 6 9

        for n in 0..3 {
            let pattern = patterns_by_len[6][n].to_vec();
            if contains_all(&pattern, &solved[4]) {
                solved[9] = pattern;
            } else if contains_all(&pattern, &solved[1]) {
                solved[0] = pattern;
            } else {
                solved[6] = pattern;
            }
        }

        // LEN 5: 2 3 5

        for n in 0..3 {
            let pattern = patterns_by_len[5][n].to_vec();
            if contains_all(&pattern, &solved[1]) {
                solved[3] = pattern;
            } else if contains_all(&solved[6], &pattern) {
                solved[5] = pattern;
            } else {
                solved[2] = pattern;
            }
        }

        for solved in &mut solved {
            solved.sort();
        }

        let mut output = 0;
        for digit in output_digits {
            output *= 10;

            let mut digit = digit.as_bytes().to_vec();
            digit.sort();

            for n in 0..10 {
                if digit == solved[n] {
                    output += n;
                    break;
                }
            }
        }
        sum += output;
    }

    sum
}

// ======================================================================
// UTIL

fn contains_all(haystack: &[u8], needles: &[u8]) -> bool {
    for needle in needles {
        if !haystack.contains(needle) {
            return false;
        }
    }
    true
}
