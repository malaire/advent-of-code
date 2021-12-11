use malaire_aoc::{parse_numbers, run};

static INPUT_A: &str = "3,4,3,1,2";
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 5934);
    run(1, solve_1, INPUT_X, 391888);

    run(0, solve_2, INPUT_A, 26984457539);
    run(2, solve_2, INPUT_X, 1754597645339);
}

fn solve_1(input: &str) -> usize {
    solve_generic(input, 80)
}

fn solve_2(input: &str) -> usize {
    solve_generic(input, 256)
}

fn solve_generic(input: &str, days: usize) -> usize {
    let mut fishes_per_timer: [usize; 9] = [0; 9];

    for fish in parse_numbers::<usize>(input) {
        fishes_per_timer[fish] += 1;
    }

    for _day in 1..=days {
        fishes_per_timer[7] += fishes_per_timer[0];
        fishes_per_timer.rotate_left(1);
    }

    fishes_per_timer.iter().sum()
}
