use ahash::AHashMap;

use malaire_aoc::run;

static INPUT_X: &str = include_str!("input");

// About 255s per solution. Uses >3 GB RAM.

fn main() {
    let start = std::time::Instant::now();
    run(1, solve_1, INPUT_X, 51939397989999);
    println!("elapsed {} s", start.elapsed().as_secs());

    let start = std::time::Instant::now();
    run(2, solve_2, INPUT_X, 11717131211195);
    println!("elapsed {} s", start.elapsed().as_secs());
}

fn solve_1(input: &str) -> u64 {
    solve_generic(input, true)
}

fn solve_2(input: &str) -> u64 {
    solve_generic(input, false)
}

const MAX_EXPECTED_STATES: usize = 60_000_000;

fn solve_generic(input: &str, is_part_1: bool) -> u64 {
    // [w x y z] -> best input for this state
    let mut states: AHashMap<[i32; 4], u64> = AHashMap::with_capacity(MAX_EXPECTED_STATES);
    states.insert([0, 0, 0, 0], 0);

    let mut max_states = 0;

    for (index, line) in input.lines().enumerate() {
        if (index + 1) % 10 == 0 {
            println!("{:3} {}", index + 1, line);
        }

        max_states = std::cmp::max(max_states, states.len());

        let line = line.as_bytes();
        let cmd = &line[..3];
        let index_a = (line[4] - b'w') as usize;

        let mut new_states: AHashMap<[i32; 4], u64>;

        if cmd == b"inp" {
            new_states = AHashMap::with_capacity(MAX_EXPECTED_STATES);

            for (state, best) in &states {
                for input in 1..=9 {
                    let mut new_state: [i32; 4] = *state;
                    new_state[index_a] = input;
                    let new_input = best * 10 + input as u64;

                    if let Some(&prev_best) = new_states.get(&new_state) {
                        if (is_part_1 && new_input > prev_best)
                            || (!is_part_1 && new_input < prev_best)
                        {
                            new_states.insert(new_state, new_input);
                        }
                    } else {
                        new_states.insert(new_state, new_input);
                    }
                }
            }
        } else {
            new_states = AHashMap::with_capacity(states.len());

            if line[6] >= b'w' && line[6] <= b'z' {
                let index_b = (line[6] - b'w') as usize;

                for (state, best) in &states {
                    let mut new_state: [i32; 4] = *state;
                    match cmd {
                        b"add" => new_state[index_a] += new_state[index_b],
                        b"mul" => new_state[index_a] *= new_state[index_b],
                        b"div" => new_state[index_a] /= new_state[index_b],
                        b"mod" => new_state[index_a] &= new_state[index_b],
                        b"eql" => {
                            new_state[index_a] = (new_state[index_a] == new_state[index_b]) as i32
                        }
                        _ => panic!(),
                    }

                    if let Some(&prev_best) = new_states.get(&new_state) {
                        if (is_part_1 && *best > prev_best) || (!is_part_1 && *best < prev_best) {
                            new_states.insert(new_state, *best);
                        }
                    } else {
                        new_states.insert(new_state, *best);
                    }
                }
            } else {
                let value_b: i32 = std::str::from_utf8(&line[6..]).unwrap().parse().unwrap();

                for (state, best) in &states {
                    let mut new_state: [i32; 4] = *state;
                    match cmd {
                        b"add" => new_state[index_a] += value_b,
                        b"mul" => new_state[index_a] *= value_b,
                        b"div" => new_state[index_a] /= value_b,
                        b"mod" => new_state[index_a] %= value_b,
                        b"eql" => new_state[index_a] = (new_state[index_a] == value_b) as i32,
                        _ => panic!(),
                    }

                    if let Some(&prev_best) = new_states.get(&new_state) {
                        if (is_part_1 && *best > prev_best) || (!is_part_1 && *best < prev_best) {
                            new_states.insert(new_state, *best);
                        }
                    } else {
                        new_states.insert(new_state, *best);
                    }
                }
            }
        }

        states = new_states;
    }

    println!("max states: {}", max_states);

    if is_part_1 {
        states
            .iter()
            .filter_map(|(v, input)| if v[3] == 0 { Some(*input) } else { None })
            .max()
            .unwrap()
    } else {
        states
            .iter()
            .filter_map(|(v, input)| if v[3] == 0 { Some(*input) } else { None })
            .min()
            .unwrap()
    }
}
