use malaire_aoc::{read_byte_array, run, Tuple2Ext};

static INPUT_A: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

static INPUT_X: &str = "4472562264
8631517827
7232144146
2447163824
1235272671
5133527146
6511372417
3841841614
8621368782
3246336677";

fn main() {
    run(0, solve_1, INPUT_A, 1656);
    run(1, solve_1, INPUT_X, 1755);

    run(0, solve_2, INPUT_A, 195);
    run(2, solve_2, INPUT_X, 212);
}

fn solve_1(input: &str) -> usize {
    solve_generic(input, true)
}

fn solve_2(input: &str) -> usize {
    solve_generic(input, false)
}

fn solve_generic(input: &str, is_step_1: bool) -> usize {
    let mut array = read_byte_array(input).map(|x| x - b'0');

    let mut flash_count = 0;
    for step in 1.. {
        let mut todo = Vec::new();

        for (index, energy) in array.indexed_iter_mut() {
            *energy += 1;
            if *energy > 9 {
                todo.push(index);
            }
        }

        let mut has_flashed = array.map(|_| false);

        while let Some(index) = todo.pop() {
            if !has_flashed[index] {
                has_flashed[index] = true;
                flash_count += 1;
                for neighbour_index in index.neighbours(array.dim()) {
                    array[neighbour_index] += 1;
                    if array[neighbour_index] > 9 {
                        todo.push(neighbour_index);
                    }
                }
            }
        }

        for (index, energy) in array.indexed_iter_mut() {
            if has_flashed[index] {
                *energy = 0;
            }
        }

        if is_step_1 {
            if step == 100 {
                return flash_count;
            }
        } else {
            if has_flashed.iter().all(|&x| x) {
                return step;
            }
        }
    }
    panic!();
}
