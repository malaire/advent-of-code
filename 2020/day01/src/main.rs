use itertools::iproduct;

use malaire_aoc::{parse_numbers, run};

static INPUT_A: &str = "1721\n979\n366\n299\n675\n1456";
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 514579);
    run(1, solve_1, INPUT_X, 545379);

    run(0, solve_2, INPUT_A, 241861950);
    run(2, solve_2, INPUT_X, 257778836);
}

fn solve_1(input: &str) -> usize {
    let numbers = parse_numbers::<usize>(input);

    for (a, b) in iproduct!(&numbers, &numbers) {
        if a + b == 2020 {
            return a * b;
        }
    }

    panic!();
}

fn solve_2(input: &str) -> usize {
    let numbers = parse_numbers::<usize>(input);

    for (a, b, c) in iproduct!(&numbers, &numbers, &numbers) {
        if a + b + c == 2020 {
            return a * b * c;
        }
    }

    panic!();
}
