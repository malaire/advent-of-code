static INPUT_A: &str = "389125467";
static INPUT_X: &str = "157623984";

const MAX_CUP: usize = 1_000_000;
const MOVES: usize = 10_000_000;

fn main() {
    assert_eq!(solve(INPUT_A, 10, false), 9 * 2);
    assert_eq!(solve(INPUT_A, MOVES, true), 149245887792);
    assert_eq!(solve(INPUT_X, MOVES, true), 111057672960);

    println!("{:?}", solve(INPUT_X, MOVES, true));
}

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

    fn solution(&self) -> usize {
        let pos_1 = &self.nodes[self.positions[1]];
        let a = &self.nodes[pos_1.next_pos];
        let b = &self.nodes[a.next_pos];
        a.cup * b.cup
    }
}

fn solve(input: &str, moves: usize, fill: bool) -> usize {
    let mut initial_cups: Vec<usize> = input
        .chars()
        .map(|ch| (ch as usize) - ('0' as usize))
        .collect();

    if fill {
        for cup in (initial_cups.len() + 1)..=MAX_CUP {
            initial_cups.push(cup);
        }
        assert!(initial_cups.len() == MAX_CUP);
    }

    let mut current_cup = initial_cups[0];
    let mut cups = Ring::new(initial_cups);

    for _ in 0..moves {
        current_cup = cups.move_three(current_cup);
    }

    cups.solution()
}
