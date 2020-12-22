use std::collections::{HashSet, VecDeque};

static INPUT_A: &str = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
static INPUT_B: &str = "Player 1:\n43\n19\n\nPlayer 2:\n2\n29\n14";
static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 291);
    assert_eq!(solve(INPUT_B), 105);
    assert_eq!(solve(INPUT_X), 35436);

    println!("{:?}", solve(INPUT_X));
}

#[derive(PartialEq)]
enum Winner {
    Player1,
    Player2,
}
use Winner::*;

fn subgame(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>) -> (Winner, VecDeque<usize>) {
    let mut seen_configurations: HashSet<String> = HashSet::new();

    loop {
        let configuration = format!(
            "{};{}",
            deck1
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(","),
            deck2
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(","),
        );
        if seen_configurations.contains(&configuration) {
            return (Player1, deck1);
        }
        seen_configurations.insert(configuration);

        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        let winner = if deck1.len() >= card1 && deck2.len() >= card2 {
            subgame(
                deck1.iter().take(card1).cloned().collect(),
                deck2.iter().take(card2).cloned().collect(),
            )
            .0
        } else if card1 > card2 {
            Player1
        } else {
            Player2
        };

        if winner == Player1 {
            deck1.push_back(card1);
            deck1.push_back(card2);
            if deck2.is_empty() {
                return (Player1, deck1);
            }
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
            if deck1.is_empty() {
                return (Player2, deck2);
            }
        }
    }
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

    let mut winners_deck = subgame(deck1, deck2).1;

    let mut score = 0;
    let mut multiplier = 1;
    while !winners_deck.is_empty() {
        score += multiplier * winners_deck.pop_back().unwrap();
        multiplier += 1;
    }

    score
}
