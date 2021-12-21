use malaire_aoc::{parse_numbers, run};

static INPUT_A: &str = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
static INPUT_X: &str = "Player 1 starting position: 9\nPlayer 2 starting position: 4";

fn main() {
    run(0, solve_1, INPUT_A, 739785);
    run(1, solve_1, INPUT_X, 998088);

    run(0, solve_2, INPUT_A, 444356092776315);
    run(2, solve_2, INPUT_X, 306621346123766);
}

fn solve_1(input: &str) -> usize {
    let numbers = parse_numbers::<usize>(input);

    let mut pos1 = numbers[1];
    let mut pos2 = numbers[3];

    let mut dice = 0;

    let mut score1 = 0;
    let mut score2 = 0;

    loop {
        pos1 = ((pos1 + ((dice + 1) % 100) + ((dice + 2) % 100) + ((dice + 3) % 100)) - 1) % 10 + 1;
        dice += 3;
        score1 += pos1;
        if score1 >= 1000 {
            return score2 * dice;
        }

        pos2 = ((pos2 + ((dice + 1) % 100) + ((dice + 2) % 100) + ((dice + 3) % 100)) - 1) % 10 + 1;
        dice += 3;
        score2 += pos2;
        if score2 >= 1000 {
            return score1 * dice;
        }
    }
}

fn solve_2(input: &str) -> usize {
    let numbers = parse_numbers::<usize>(input);

    let (wins1, wins2) = wins(numbers[1], numbers[3], 0, 0);
    std::cmp::max(wins1, wins2)
}

// In how many ways these three-dice-sums can be reached.
// 3: 1x 111
// 4: 3x 112 121 211
// 5: 6x 113 122 131 212 221 311
// 6: 7x 123 132 213 222 231 312 321
// 7: 6x 133 223 232 313 322 331
// 8: 3x 233 323 332
// 9: 1x 333
const COMBINATIONS: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

// returns (player1_win_count, player2_win_count)
fn wins(pos1: usize, pos2: usize, score1: usize, score2: usize) -> (usize, usize) {
    let mut wins1 = 0;
    let mut wins2 = 0;

    for roll_sum1 in 3..=9 {
        let new_pos1 = (pos1 + roll_sum1 - 1) % 10 + 1;
        let new_score1 = score1 + new_pos1;
        if new_score1 >= 21 {
            wins1 += COMBINATIONS[roll_sum1];
        } else {
            for roll_sum2 in 3..=9 {
                let new_pos2 = (pos2 + roll_sum2 - 1) % 10 + 1;
                let new_score2 = score2 + new_pos2;
                if new_score2 >= 21 {
                    wins2 += COMBINATIONS[roll_sum1] * COMBINATIONS[roll_sum2];
                } else {
                    let (sub_wins1, sub_wins2) = wins(new_pos1, new_pos2, new_score1, new_score2);
                    wins1 += sub_wins1 * COMBINATIONS[roll_sum1] * COMBINATIONS[roll_sum2];
                    wins2 += sub_wins2 * COMBINATIONS[roll_sum1] * COMBINATIONS[roll_sum2];
                }
            }
        }
    }

    (wins1, wins2)
}
