use malaire_aoc::run;

static INPUT_A: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 26397);
    run(1, solve_1, INPUT_X, 265527);

    run(0, solve_2, INPUT_A, 288957);
    run(2, solve_2, INPUT_X, 3969823589);
}

fn solve_1(input: &str) -> usize {
    let mut total_score = 0;

    for line in input.lines() {
        let mut stack = Vec::new();

        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        total_score += syntax_error_score(ch);
                        break;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        total_score += syntax_error_score(ch);
                        break;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        total_score += syntax_error_score(ch);
                        break;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        total_score += syntax_error_score(ch);
                        break;
                    }
                }
                _ => panic!(),
            }
        }
    }

    total_score
}

fn solve_2(input: &str) -> usize {
    let mut scores = Vec::new();

    'lines: for line in input.lines() {
        let mut stack = Vec::new();
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        continue 'lines;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        continue 'lines;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        continue 'lines;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        continue 'lines;
                    }
                }
                _ => panic!(),
            }
        }

        let mut score = 0;
        while let Some(ch) = stack.pop() {
            score *= 5;
            match ch {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => panic!(),
            }
        }
        scores.push(score);
    }

    scores.sort();
    scores[scores.len() / 2]
}

// ======================================================================
// UTIL

fn syntax_error_score(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}
