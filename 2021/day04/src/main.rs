use ndarray::Array2;

use malaire_aoc::{parse_numbers, run, ArrayBaseExt, VecExt};

static INPUT_A: &str = include_str!("input_a");
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 4512);
    run(1, solve_1, INPUT_X, 33462);

    run(0, solve_2, INPUT_A, 1924);
    run(2, solve_2, INPUT_X, 30070);
}

fn solve_1(input: &str) -> usize {
    solve_generic(input, true)
}

fn solve_2(input: &str) -> usize {
    solve_generic(input, false)
}

fn solve_generic(input: &str, first_winner: bool) -> usize {
    let blocks: Vec<_> = input.split("\n\n").collect();
    let numbers_to_call = parse_numbers::<usize>(blocks[0]);

    // Marked numbers will be `None`
    let mut boards: Vec<Array2<Option<usize>>> = Vec::new();
    for block in &blocks[1..] {
        boards.push(parse_numbers(block).map(|&x| Some(x)).into_array((5, 5)));
    }

    let mut has_won: Vec<bool> = vec![false; boards.len()];

    for called in numbers_to_call {
        for (index, board) in boards.iter_mut().enumerate() {
            if !has_won[index] {
                for item in board.iter_mut() {
                    if *item == Some(called) {
                        *item = None;
                    }
                }

                if board.rowcol_iter().any(|rc| rc.iter().all(|x| x.is_none())) {
                    has_won[index] = true;
                    if first_winner || has_won.iter().all(|&x| x) {
                        let sum: usize = board.iter().filter_map(|&x| x).sum();
                        return called * sum;
                    }
                }
            }
        }
    }

    panic!();
}
