use regex::Regex;

use malaire_aoc::run;

static INPUT_A: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 2);
    run(1, solve_1, INPUT_X, 607);

    run(0, solve_2, INPUT_A, 1);
    run(2, solve_2, INPUT_X, 321);
}

fn solve_1(input: &str) -> usize {
    // 1-3 b: cdefg
    let re = Regex::new(r"(?m)^(\d+)-(\d+) (.): (.+)$").unwrap();

    let mut valid_count: usize = 0;

    for x in re.captures_iter(input) {
        let min: usize = x[1].parse().unwrap();
        let max: usize = x[2].parse().unwrap();
        let needle = &x[3];
        let haystack = &x[4];

        let match_count = haystack.matches(needle).count();

        if match_count >= min && match_count <= max {
            valid_count += 1;
        }
    }

    valid_count
}

fn solve_2(input: &str) -> usize {
    // 1-3 b: cdefg
    let re = Regex::new(r"(?m)^(\d+)-(\d+) (.): (.+)$").unwrap();

    let mut valid_count: usize = 0;

    for x in re.captures_iter(input) {
        let pos_a: usize = x[1].parse().unwrap();
        let pos_b: usize = x[2].parse().unwrap();
        let needle = x[3].as_bytes()[0];
        let haystack = &x[4].as_bytes();

        if (haystack[pos_a - 1] == needle) ^ (haystack[pos_b - 1] == needle) {
            valid_count += 1;
        }
    }

    valid_count
}
