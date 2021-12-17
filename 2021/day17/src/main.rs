use malaire_aoc::{parse_numbers, run};

static INPUT_A: &str = "target area: x=20..30, y=-10..-5";
static INPUT_X: &str = "target area: x=156..202, y=-110..-69";

fn main() {
    run(0, solve_1, INPUT_A, 45);
    run(1, solve_1, INPUT_X, 5995);

    run(0, solve_2, INPUT_A, 112);
    run(2, solve_2, INPUT_X, 3202);
}

fn solve_1(input: &str) -> i64 {
    solve_both(input).0
}

fn solve_2(input: &str) -> i64 {
    solve_both(input).1
}

// returns (best_max_y, count)
fn solve_both(input: &str) -> (i64, i64) {
    let numbers = parse_numbers::<i64>(input);
    let x1 = numbers[0];
    let x2 = numbers[1];
    let y1 = -numbers[2];
    let y2 = -numbers[3];

    let mut best_max_y = 0;
    let mut count = 0;

    for initial_vx in 0..=999 {
        for initial_vy in -999..=999 {
            let mut vx = initial_vx;
            let mut vy = initial_vy;

            let mut x = 0;
            let mut y = 0;
            let mut reached_target = false;
            let mut max_y = 0;

            while x <= x2 && y >= y1 {
                x += vx;
                y += vy;
                if vx > 0 {
                    vx -= 1
                }
                vy -= 1;

                max_y = std::cmp::max(max_y, y);

                if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
                    reached_target = true;
                    break;
                }
            }

            if reached_target {
                count += 1;
                best_max_y = std::cmp::max(best_max_y, max_y);
            }
        }
    }

    (best_max_y, count)
}
