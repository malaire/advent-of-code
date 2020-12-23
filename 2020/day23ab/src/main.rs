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
    let initial_cups = parse_input(input);

    let mut current_cup = initial_cups[0];
    let mut cups = Ring::new(initial_cups);

    for _ in 0..moves {
        current_cup = cups.move_three(current_cup);
    }

    cups.solution_1()
}

fn solve_2(input: &str, moves: usize, fill_to_million: bool) -> usize {
    let mut initial_cups = parse_input(input);

    if fill_to_million {
        for cup in initial_cups.len()..1_000_000 {
            initial_cups.push(cup);
        }
    }

    let mut current_cup = initial_cups[0];
    let mut cups = Ring::new(initial_cups);

    for _ in 0..moves {
        current_cup = cups.move_three(current_cup);
    }

    cups.solution_2()
}

// ================================================================================
// HELPERS

fn parse_input(input: &str) -> Vec<usize> {
    // normalize cups to start from 0 instead of 1
    input
        .chars()
        .map(|ch| (ch as usize) - ('1' as usize))
        .collect()
}
// ================================================================================
// Ring

struct Ring {
    // indexed by cup
    next_cup: Vec<usize>,
}

impl Ring {
    fn new(cups: Vec<usize>) -> Self {
        let len = cups.len();
        let mut next_cup = vec![0; len];

        let first_cup = cups[0];
        let mut prev_cup = None;
        for cup in &cups {
            if let Some(prev_cup) = prev_cup {
                next_cup[prev_cup] = *cup;
            }
            prev_cup = Some(*cup);
        }
        next_cup[prev_cup.unwrap()] = first_cup;
        Ring { next_cup }
    }

    // O(1)
    fn move_three(&mut self, current_cup: usize) -> usize {
        let a_cup = self.next_cup[current_cup];
        let b_cup = self.next_cup[a_cup];
        let c_cup = self.next_cup[b_cup];
        let next_current_cup = self.next_cup[c_cup];

        let len = self.next_cup.len();

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

        self.next_cup[current_cup] = next_current_cup;
        self.next_cup[c_cup] = self.next_cup[destination_cup];
        self.next_cup[destination_cup] = a_cup;

        next_current_cup
    }

    fn solution_1(&self) -> usize {
        let mut solution = 0;
        let mut cup = self.next_cup[0];
        while cup != 0 {
            solution *= 10;
            solution += cup + 1;
            cup = self.next_cup[cup];
        }
        solution
    }

    fn solution_2(&self) -> usize {
        let a = self.next_cup[0];
        let b = self.next_cup[a];
        (a + 1) * (b + 1)
    }
}
