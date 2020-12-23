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
        for cup in (initial_cups.len() + 1)..=1_000_000 {
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
    input
        .chars()
        .map(|ch| (ch as usize) - ('0' as usize))
        .collect()
}
// ================================================================================
// Ring

struct Node {
    cup: usize,
    next_pos: usize,
}

struct Ring {
    len: usize,
    nodes: Vec<Node>,
    // positions[cup] is position of the node which contains cup
    positions: Vec<usize>,
}

impl Ring {
    fn new(cups: Vec<usize>) -> Self {
        let len = cups.len();
        let mut nodes = Vec::with_capacity(len);
        let mut positions = vec![0; len + 1];

        for (pos, cup) in cups.into_iter().enumerate() {
            nodes.push(Node {
                cup,
                next_pos: (pos + 1) % len,
            });
            positions[cup] = pos;
        }
        Ring {
            len,
            nodes,
            positions,
        }
    }

    // O(1)
    fn move_three(&mut self, current_cup: usize) -> usize {
        let current_pos = self.positions[current_cup];

        let current = &self.nodes[current_pos];
        let a_pos = current.next_pos;
        let a = &self.nodes[a_pos];
        let b = &self.nodes[a.next_pos];
        let c_pos = b.next_pos;
        let c = &self.nodes[c_pos];

        let mut destination_cup = if current_cup == 1 {
            self.len
        } else {
            current_cup - 1
        };
        while destination_cup == a.cup || destination_cup == b.cup || destination_cup == c.cup {
            if destination_cup == 1 {
                destination_cup = self.len;
            } else {
                destination_cup -= 1;
            }
        }

        let destination_pos = self.positions[destination_cup];

        let next_current_cup = self.nodes[c.next_pos].cup;

        self.nodes[current_pos].next_pos = c.next_pos;
        self.nodes[c_pos].next_pos = self.nodes[destination_pos].next_pos;
        self.nodes[destination_pos].next_pos = a_pos;

        next_current_cup
    }

    fn solution_1(&self) -> usize {
        let mut solution = 0;
        let mut pos = self.positions[1];
        for _ in 0..(self.len - 1) {
            pos = self.nodes[pos].next_pos;
            solution *= 10;
            solution += self.nodes[pos].cup;
        }
        solution
    }

    fn solution_2(&self) -> usize {
        let pos_1 = &self.nodes[self.positions[1]];
        let a = &self.nodes[pos_1.next_pos];
        let b = &self.nodes[a.next_pos];
        a.cup * b.cup
    }
}
