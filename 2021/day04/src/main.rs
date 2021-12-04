static INPUT_A: &str = include_str!("input_a");
static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 4512);
    assert_eq!(solve(INPUT_X), 33462);
    println!("{:?}", solve(INPUT_X));

    assert_eq!(solve_2(INPUT_A), 1924);
    assert_eq!(solve_2(INPUT_X), 30070);
    println!("{:?}", solve_2(INPUT_X));
}

fn solve(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let numbers_to_call: Vec<usize> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    let mut boards: Vec<Vec<(usize, bool)>> = Vec::new();
    for n in 0.. {
        if n * 6 + 2 > lines.len() {
            break;
        }
        let mut board = Vec::with_capacity(25);
        for line in &lines[n * 6 + 2..=n * 6 + 6] {
            let board_line: Vec<(usize, bool)> = line
                .split_whitespace()
                .map(|x| (x.parse().unwrap(), false))
                .collect();
            board.extend_from_slice(&board_line);
        }
        boards.push(board);
    }

    for called in numbers_to_call {
        for board in boards.iter_mut() {
            // MARK

            for cell in board.iter_mut() {
                if cell.0 == called {
                    cell.1 = true;
                }
            }

            // WIN ?

            for n in 0..5 {
                if (board[n * 5].1
                    && board[n * 5 + 1].1
                    && board[n * 5 + 2].1
                    && board[n * 5 + 3].1
                    && board[n * 5 + 4].1)
                    || (board[n].1
                        && board[n + 5].1
                        && board[n + 10].1
                        && board[n + 15].1
                        && board[n + 20].1)
                {
                    let mut sum = 0;
                    for x in board {
                        if !x.1 {
                            sum += x.0;
                        }
                    }
                    return called * sum;
                }
            }
        }
    }
    panic!();
}

fn solve_2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let numbers_to_call: Vec<usize> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    let mut boards: Vec<Vec<(usize, bool)>> = Vec::new();
    for n in 0.. {
        if n * 6 + 2 > lines.len() {
            break;
        }
        let mut board = Vec::with_capacity(25);
        for line in &lines[n * 6 + 2..=n * 6 + 6] {
            let board_line: Vec<(usize, bool)> = line
                .split_whitespace()
                .map(|x| (x.parse().unwrap(), false))
                .collect();
            board.extend_from_slice(&board_line);
        }
        boards.push(board);
    }

    let board_count = boards.len();
    let mut has_won: Vec<bool> = vec![false; board_count];
    let mut has_won_count = 0;

    for called in numbers_to_call {
        for (index, board) in boards.iter_mut().enumerate() {
            if has_won[index] {
                continue;
            }

            // MARK

            for cell in board.iter_mut() {
                if cell.0 == called {
                    cell.1 = true;
                }
            }

            // WIN ?

            'check: for n in 0..5 {
                if (board[n * 5].1
                    && board[n * 5 + 1].1
                    && board[n * 5 + 2].1
                    && board[n * 5 + 3].1
                    && board[n * 5 + 4].1)
                    || (board[n].1
                        && board[n + 5].1
                        && board[n + 10].1
                        && board[n + 15].1
                        && board[n + 20].1)
                {
                    has_won[index] = true;
                    has_won_count += 1;

                    if has_won_count == board_count {
                        let mut sum = 0;
                        for x in board {
                            if !x.1 {
                                sum += x.0;
                            }
                        }
                        return sum * called;
                    } else {
                        break 'check;
                    }
                }
            }
        }
    }
    panic!();
}
