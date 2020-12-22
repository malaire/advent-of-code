use std::collections::VecDeque;

static INPUT_A: &str = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 306);
    assert_eq!(solve(INPUT_X), 31754);

    println!("{:?}", solve(INPUT_X));
}

fn solve(input: &str) -> usize {
    let mut deck1: VecDeque<usize> = VecDeque::new();
    let mut deck2: VecDeque<usize> = VecDeque::new();

    let mut input_player_1 = true;
    for line in input.lines() {
        if line == "Player 1:" || line == "" {
            // IGNORE
        } else if line == "Player 2:" {
            input_player_1 = false;
        } else if input_player_1 {
            deck1.push_back(line.parse::<usize>().unwrap());
        } else {
            deck2.push_back(line.parse::<usize>().unwrap());
        }
    }

    while !(deck1.is_empty() || deck2.is_empty()) {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    let mut winners_deck = if deck1.is_empty() { deck2 } else { deck1 };

    let mut score = 0;
    let mut multiplier = 1;
    while !winners_deck.is_empty() {
        score += multiplier * winners_deck.pop_back().unwrap();
        multiplier += 1;
    }

    score
}
