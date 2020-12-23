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
    let (first_current_cup, mut cups) = parse_input(input, false);

    play_game(&mut cups, first_current_cup, moves);

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
    let (first_current_cup, mut cups) = parse_input(input, fill_to_million);

    play_game(&mut cups, first_current_cup, moves);

    let a = cups[0];
    let b = cups[a];
    (a + 1) * (b + 1)
}

// ================================================================================
// HELPERS

// Return `(first_cup, cups)`
// - `cups` is indexed by cup and contains next cup
fn parse_input(input: &str, fill_to_million: bool) -> (usize, Vec<usize>) {
    // normalize cups to start from 0 instead of 1
    let mut initial_cups: Vec<usize> = input
        .chars()
        .map(|ch| (ch as usize) - ('1' as usize))
        .collect();

    if fill_to_million {
        for cup in initial_cups.len()..1_000_000 {
            initial_cups.push(cup);
        }
    }

    let len = initial_cups.len();
    let mut cups = vec![0; len];

    let first_cup = initial_cups[0];
    let mut prev_cup = None;
    for cup in &initial_cups {
        if let Some(prev_cup) = prev_cup {
            cups[prev_cup] = *cup;
        }
        prev_cup = Some(*cup);
    }
    cups[prev_cup.unwrap()] = first_cup;

    (first_cup, cups)
}

fn play_game(cups: &mut Vec<usize>, mut current_cup: usize, moves: usize) {
    for _ in 0..moves {
        let a_cup = cups[current_cup];
        let b_cup = cups[a_cup];
        let c_cup = cups[b_cup];
        let next_current_cup = cups[c_cup];

        let len = cups.len();

        let mut destination_cup = if current_cup == 0 {
            len - 1
        } else {
            current_cup - 1
        };
        while destination_cup == a_cup || destination_cup == b_cup || destination_cup == c_cup {
            if destination_cup == 0 {
                destination_cup = len - 1;
            } else {
                destination_cup -= 1;
            }
        }

        cups[current_cup] = next_current_cup;
        cups[c_cup] = cups[destination_cup];
        cups[destination_cup] = a_cup;

        current_cup = next_current_cup
    }
}
