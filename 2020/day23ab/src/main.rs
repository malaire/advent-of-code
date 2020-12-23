static INPUT_A: &str = "389125467";
static INPUT_X: &str = "157623984";

fn main() {
    // PART 1

    assert_eq!(solve_1(INPUT_A, 10), 92658374);
    assert_eq!(solve_1(INPUT_A, 100), 67384529);
    assert_eq!(solve_1(INPUT_X, 100), 58427369);

    println!("{:?}", solve_1(INPUT_X, 100));

    // PART 2

    assert_eq!(solve_2(INPUT_A, 10, false), 9 * 2);
    assert_eq!(solve_2(INPUT_A, 10_000_000, true), 149245887792);
    assert_eq!(solve_2(INPUT_X, 10_000_000, true), 111057672960);

    println!("{:?}", solve_2(INPUT_X, 10_000_000, true));
}

fn solve_1(input: &str, moves: usize) -> usize {
    let (first, mut cups) = parse_input(input, false);

    play_game(&mut cups, first, moves);

    let mut solution = 0;
    let mut cup = cups[0];
    while cup != 0 {
        solution *= 10;
        solution += cup + 1;
        cup = cups[cup];
    }
    solution
}

fn solve_2(input: &str, moves: usize, fill_to_million: bool) -> usize {
    let (first, mut cups) = parse_input(input, fill_to_million);

    play_game(&mut cups, first, moves);

    let a = cups[0];
    let b = cups[a];
    (a + 1) * (b + 1)
}

// ================================================================================
// HELPERS

// Return `(first_cup, cups)`
// - `cups` is indexed by cup and contains next cup
// - normalizes cups to start from 0 instead of 1
fn parse_input(input: &str, fill_to_million: bool) -> (usize, Vec<usize>) {
    let mut initial_cups: Vec<usize> = input
        .chars()
        .map(|ch| (ch as usize) - ('1' as usize))
        .collect();

    if fill_to_million {
        for cup in initial_cups.len()..1_000_000 {
            initial_cups.push(cup);
        }
    }

    let mut cups = vec![0; initial_cups.len()];

    let first = initial_cups[0];
    let mut prev = None;
    for cup in &initial_cups {
        if let Some(prev) = prev {
            cups[prev] = *cup;
        }
        prev = Some(*cup);
    }
    cups[prev.unwrap()] = first;

    (first, cups)
}

fn play_game(cups: &mut Vec<usize>, mut current: usize, moves: usize) {
    let cup_count = cups.len();

    for _ in 0..moves {
        let a = cups[current];
        let b = cups[a];
        let c = cups[b];
        let next_current = cups[c];

        let mut destination = if current == 0 {
            cup_count - 1
        } else {
            current - 1
        };
        while destination == a || destination == b || destination == c {
            if destination == 0 {
                destination = cup_count - 1;
            } else {
                destination -= 1;
            }
        }

        cups[current] = next_current;
        cups[c] = cups[destination];
        cups[destination] = a;

        current = next_current
    }
}
