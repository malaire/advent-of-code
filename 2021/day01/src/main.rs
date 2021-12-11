use malaire_aoc::{parse_numbers, run};

static INPUT_A: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 7);
    run(1, solve_1, INPUT_X, 1553);

    run(0, solve_2, INPUT_A, 5);
    run(2, solve_2, INPUT_X, 1597);
}

fn solve_1(input: &str) -> usize {
    solve_generic(input, 1)
}

fn solve_2(input: &str) -> usize {
    solve_generic(input, 3)
}

fn solve_generic(input: &str, delta: usize) -> usize {
    let input = parse_numbers::<usize>(input);

    let mut count = 0;

    for n in 0..input.len() - delta {
        if input[n] < input[n + delta] {
            count += 1;
        }
    }

    count
}
