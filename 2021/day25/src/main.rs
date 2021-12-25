use malaire_aoc::{read_byte_array, run};

static INPUT_A: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 58);
    run(1, solve_1, INPUT_X, 579);
}

fn solve_1(input: &str) -> usize {
    let mut state = read_byte_array(input);
    let dim = state.dim();

    for step in 1.. {
        let mut new_state = state.map(|_| b'.');
        let mut moved = false;

        for (pos, &b) in state.indexed_iter() {
            let next_pos = (pos.0, (pos.1 + 1) % dim.1);
            if b == b'>' && state[next_pos] == b'.' {
                new_state[next_pos] = b;
                moved = true;
            } else if b != b'.' {
                new_state[pos] = b;
            }
        }

        state = new_state;
        new_state = state.map(|_| b'.');

        for (pos, &b) in state.indexed_iter() {
            let next_pos = ((pos.0 + 1) % dim.0, pos.1);
            if b == b'v' && state[next_pos] == b'.' {
                new_state[next_pos] = b;
                moved = true;
            } else if b != b'.' {
                new_state[pos] = b;
            }
        }

        if !moved {
            return step;
        }

        state = new_state;
    }

    panic!();
}
