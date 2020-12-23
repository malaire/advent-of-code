use std::collections::VecDeque;

static INPUT_A: &str = "389125467";
static INPUT_X: &str = "157623984";

fn main() {
    assert_eq!(solve(INPUT_A, 10), 92658374);
    assert_eq!(solve(INPUT_A, 100), 67384529);
    assert_eq!(solve(INPUT_X, 100), 58427369);

    println!("{:?}", solve(INPUT_X, 100));
}

fn get_index(cup: usize, cups: &VecDeque<usize>) -> usize {
    let mut index = 0;
    while cups[index] != cup {
        index += 1;
    }
    index
}

fn solve(input: &str, moves: usize) -> usize {
    let mut cups: VecDeque<usize> = input
        .chars()
        .map(|ch| (ch as usize) - ('0' as usize))
        .collect();

    let min_cup = *cups.iter().min().unwrap();
    let max_cup = *cups.iter().max().unwrap();

    let mut current = *cups.front().unwrap();
    for _ in 0..moves {
        let a = cups
            .remove((get_index(current, &cups) + 1) % cups.len())
            .unwrap();
        let b = cups
            .remove((get_index(current, &cups) + 1) % cups.len())
            .unwrap();
        let c = cups
            .remove((get_index(current, &cups) + 1) % cups.len())
            .unwrap();

        let mut destination = current - 1;
        if destination < min_cup {
            destination = max_cup;
        }
        while destination == a || destination == b || destination == c {
            destination -= 1;
            if destination < min_cup {
                destination = max_cup;
            }
        }

        let destination_index = get_index(destination, &cups);
        cups.insert(destination_index + 1, c);
        cups.insert(destination_index + 1, b);
        cups.insert(destination_index + 1, a);

        current = cups[(get_index(current, &cups) + 1) % cups.len()];
    }

    while cups.front().unwrap() != &1 {
        let cup = cups.pop_front().unwrap();
        cups.push_back(cup);
    }
    cups.pop_front().unwrap();

    let mut result = 0;
    for cup in &cups {
        result *= 10;
        result += cup;
    }

    result
}
