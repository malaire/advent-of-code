use std::collections::HashMap;

use malaire_aoc::run;

static INPUT_A: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

static INPUT_X: &str = "#############
#...........#
###B#B#D#D###
  #C#C#A#A#
  #########";

fn main() {
    let start = std::time::Instant::now();
    run(0, solve_1, INPUT_A, 12521);
    println!("elapsed {} ms", start.elapsed().as_millis());

    let start = std::time::Instant::now();
    run(1, solve_1, INPUT_X, 10411);
    println!("elapsed {} ms", start.elapsed().as_millis());

    let start = std::time::Instant::now();
    run(0, solve_2, INPUT_A, 44169);
    println!("elapsed {} ms", start.elapsed().as_millis());

    let start = std::time::Instant::now();
    run(2, solve_2, INPUT_X, 46721);
    println!("elapsed {} ms", start.elapsed().as_millis());
}

fn solve_1(input: &str) -> usize {
    solve_generic(&mut parse_input_1(input), 1)
}

fn solve_2(input: &str) -> usize {
    solve_generic(&mut parse_input_2(input), 2)
}

fn solve_generic(pos: &mut [usize], room_size_log: usize) -> usize {
    let room_size = 1 << room_size_log;

    let mut room_finished = [0; 4];

    // only check one finished per room as inputs don't have more
    for (id, pos) in pos.iter_mut().enumerate() {
        let room = id >> room_size_log;
        if *pos == room * room_size {
            *pos = FINISHED_POS;
            room_finished[room] = 1;
        }
    }

    let occupied = pos.iter().map(|pos| 1 << pos).sum();
    let moves = gen_moves(room_size_log);

    let mut state = State {
        pos,
        occupied,
        finished: room_finished.iter().sum(),
        room_finished: &mut room_finished,
        moves_out: &gen_moves_out(&moves, room_size_log),
        moves_out_via: &gen_moves_out_via(&moves, room_size_log),
        moves_in: &gen_moves_in(&moves, room_size_log),

        room_size_log,
        amphipod_max_id: room_size * 4 - 1,
    };

    play(&mut state, usize::MAX).unwrap()
}

// ======================================================================
// PARSE INPUT

fn parse_input_1(input: &str) -> [usize; 8] {
    let mut all_pos = [0; 8];
    let mut indexes = [0; 4];

    let lines: Vec<_> = input.lines().map(|x| x.as_bytes()).collect();

    for (row, col, pos) in [
        (2, 3, 1),
        (3, 3, 0),
        (2, 5, 3),
        (3, 5, 2),
        (2, 7, 5),
        (3, 7, 4),
        (2, 9, 7),
        (3, 9, 6),
    ] {
        let kind = (lines[row][col] - b'A') as usize;
        all_pos[kind * 2 + indexes[kind]] = pos;
        indexes[kind] += 1;
    }

    all_pos
}

fn parse_input_2(input: &str) -> [usize; 16] {
    let mut all_pos = [0; 16];
    let mut indexes = [0; 4];

    let mut lines: Vec<_> = input.lines().map(|x| x.as_bytes()).collect();

    lines.insert(3, b"  #D#C#B#A#  ");
    lines.insert(4, b"  #D#B#A#C#  ");

    for (row, col, pos) in [
        (2, 3, 3),
        (3, 3, 2),
        (4, 3, 1),
        (5, 3, 0),
        (2, 5, 7),
        (3, 5, 6),
        (4, 5, 5),
        (5, 5, 4),
        (2, 7, 11),
        (3, 7, 10),
        (4, 7, 9),
        (5, 7, 8),
        (2, 9, 15),
        (3, 9, 14),
        (4, 9, 13),
        (5, 9, 12),
    ] {
        let kind = (lines[row][col] - b'A') as usize;
        all_pos[kind * 4 + indexes[kind]] = pos;
        indexes[kind] += 1;
    }

    all_pos
}

// ======================================================================
// CONST

//
// #############
// #...........#
// ###.#.#.#.###
//   #.#.#.#.#
//   #########
//
// 16 17 xx 18 xx 19 xx 20 xx 21 22
//       01    03    05    07
//       00    02    04    06
//
// #############
// #...........#
// ###.#.#.#.###
//   #.#.#.#.#
//   #.#.#.#.#
//   #.#.#.#.#
//   #########
//
// 16 17 xx 18 xx 19 xx 20 xx 21 22
//       03    07    11    15
//       02    06    10    14
//       01    05    09    13
//       00    04    08    12

const HALLWAY_MIN_POS: usize = 16;
const HALLWAY_MAX_POS: usize = HALLWAY_MIN_POS + 6;

const FINISHED_POS: usize = 99;

// ======================================================================
// TYPES

// (source_pos, target_pos) -> (steps, via)
type Moves = HashMap<(usize, usize), (usize, usize)>;

// [source_pos] -> (target_pos, steps, via)
type MovesOut = Vec<Vec<(usize, usize, usize)>>;

// [source_pos] -> via
type MovesOutVia = Vec<usize>;

// [room][pos - 8][room_finished_count] -> (steps, via)
type MovesIn = Vec<Vec<Vec<(usize, usize)>>>;

struct State<'a> {
    pos: &'a mut [usize],
    occupied: usize,
    finished: usize,
    room_finished: &'a mut [usize; 4],
    moves_out: &'a MovesOut,
    moves_out_via: &'a MovesOutVia,
    moves_in: &'a MovesIn,

    room_size_log: usize,
    amphipod_max_id: usize,
}

// ======================================================================
// PLAY

fn play(state: &mut State, mut energy_limit: usize) -> Option<usize> {
    let mut best: Option<usize> = None;

    for id in 0..=state.amphipod_max_id {
        let pos = state.pos[id];

        if pos == FINISHED_POS {
            continue;
        }

        let kind = id >> state.room_size_log;

        let cost = match kind {
            0 => 1,
            1 => 10,
            2 => 100,
            3 => 1000,
            _ => panic!(),
        };

        if pos < HALLWAY_MIN_POS {
            // MOVE OUT OF ROOM

            if state.occupied & state.moves_out_via[pos] == 0 {
                for (new_pos, steps, via) in &state.moves_out[pos] {
                    if state.occupied & via == 0 && steps * cost < energy_limit {
                        state.pos[id] = *new_pos;

                        state.occupied ^= 1 << pos;
                        state.occupied ^= 1 << new_pos;

                        if let Some(sub_energy) = play(state, energy_limit - steps * cost) {
                            let energy = steps * cost + sub_energy;

                            if best.is_none() || energy < best.unwrap() {
                                best = Some(energy);
                                energy_limit = energy;
                            }
                        }

                        state.occupied ^= 1 << pos;
                        state.occupied ^= 1 << new_pos;
                    }
                }
            }
        } else {
            // MOVE INTO ROOM

            let (steps, via) =
                state.moves_in[kind][pos - HALLWAY_MIN_POS][state.room_finished[kind]];

            if state.occupied & via == 0 && steps * cost < energy_limit {
                if state.finished == state.amphipod_max_id {
                    return Some(steps * cost);
                } else {
                    state.pos[id] = FINISHED_POS;

                    state.finished += 1;
                    state.room_finished[kind] += 1;
                    state.occupied ^= 1 << pos;

                    if let Some(sub_energy) = play(state, energy_limit - steps * cost) {
                        let energy = steps * cost + sub_energy;

                        if best.is_none() || energy < best.unwrap() {
                            best = Some(energy);
                            energy_limit = energy;
                        }
                    }

                    state.finished -= 1;
                    state.room_finished[kind] -= 1;
                    state.occupied ^= 1 << pos;
                }
            }
        }

        state.pos[id] = pos;
    }

    best
}

// ======================================================================
// GENERATE MOVES

fn gen_moves(room_size_log: usize) -> Moves {
    let room_size = 1 << room_size_log;
    let room_mask = room_size - 1;

    let mut moves = HashMap::new();
    for pos_in_room in 0..room_size * 4 {
        let room = pos_in_room >> room_size_log;
        let max_pos_left_of_room = HALLWAY_MIN_POS + 1 + room;
        let min_pos_right_of_room = HALLWAY_MIN_POS + 2 + room;
        let max_room_pos = (room + 1) * room_size - 1;

        for pos_in_hallway in HALLWAY_MIN_POS..=HALLWAY_MAX_POS {
            let mut steps = room_size - (pos_in_room & room_mask) + 1;

            let mut via = 0;
            for v in pos_in_room + 1..=max_room_pos {
                via |= 1 << v;
            }

            if pos_in_hallway <= max_pos_left_of_room {
                steps += (max_pos_left_of_room - pos_in_hallway)
                    + (max_pos_left_of_room - std::cmp::max(HALLWAY_MIN_POS + 1, pos_in_hallway));
                for v in pos_in_hallway + 1..=max_pos_left_of_room {
                    via |= 1 << v;
                }
            } else {
                steps += (pos_in_hallway - min_pos_right_of_room)
                    + (std::cmp::min(HALLWAY_MIN_POS + 5, pos_in_hallway) - min_pos_right_of_room);
                for v in min_pos_right_of_room..pos_in_hallway {
                    via |= 1 << v;
                }
            }

            moves.insert(
                (pos_in_room, pos_in_hallway),
                (steps, via | 1 << pos_in_hallway),
            );
            moves.insert(
                (pos_in_hallway, pos_in_room),
                (steps, via | 1 << pos_in_room),
            );
        }
    }
    moves
}

fn gen_moves_out(moves: &Moves, room_size_log: usize) -> MovesOut {
    let room_size = 1 << room_size_log;

    let mut moves_out = Vec::new();
    for source_pos in 0..room_size * 4 {
        let mut inner = Vec::new();
        for target_pos in HALLWAY_MIN_POS..=HALLWAY_MAX_POS {
            let (steps, via) = moves[&(source_pos, target_pos)];
            inner.push((target_pos, steps, via));
        }
        moves_out.push(inner);
    }
    moves_out
}

fn gen_moves_out_via(moves: &Moves, room_size_log: usize) -> MovesOutVia {
    let room_size = 1 << room_size_log;

    let mut moves_out_via = Vec::new();
    for source_pos in 0..room_size * 4 {
        let mut via = usize::MAX;
        for target_pos in HALLWAY_MIN_POS..=HALLWAY_MAX_POS {
            via &= moves[&(source_pos, target_pos)].1;
        }
        moves_out_via.push(via);
    }
    moves_out_via
}

fn gen_moves_in(moves: &Moves, room_size_log: usize) -> MovesIn {
    let room_size = 1 << room_size_log;

    let mut moves_in = Vec::new();
    for room in 0..4 {
        let mut a = Vec::new();
        for source_pos in HALLWAY_MIN_POS..=HALLWAY_MAX_POS {
            let mut b = Vec::new();
            for room_finished in 0..room_size {
                let target_pos = room * room_size + room_finished;
                let (steps, via) = moves[&(source_pos, target_pos)];
                b.push((steps, via));
            }
            a.push(b);
        }
        moves_in.push(a);
    }
    moves_in
}
