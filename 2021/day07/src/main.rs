use malaire_aoc::prelude::*;

static INPUT_A: &str = "16,1,2,0,4,2,7,1,2,14";
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 37);
    run(1, solve_1, INPUT_X, 340056);

    run(0, solve_2, INPUT_A, 168);
    run(2, solve_2, INPUT_X, 96592275);
}

fn solve_1(input: &str) -> isize {
    solve_generic(input, true)
}

fn solve_2(input: &str) -> isize {
    solve_generic(input, false)
}

fn solve_generic(input: &str, constant_rate: bool) -> isize {
    let input = parse_numbers::<isize>(input);

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut min_fuel = isize::MAX;

    for target in min..=max {
        let mut fuel = 0;

        for pos in &input {
            let delta = (pos - target).abs();

            if constant_rate {
                fuel += delta;
            } else {
                fuel += delta * (1 + delta) / 2;
            }
        }

        min_fuel = min_fuel.min(fuel);
    }

    min_fuel
}
