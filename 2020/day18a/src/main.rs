use regex::Regex;

static INPUT_A: &str = "1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 71 + 51);
    assert_eq!(solve(INPUT_X), 45283905029161);

    println!("{:?}", solve(INPUT_X));
}

fn solve(input: &str) -> usize {
    let re_one_op = Regex::new(r"^(.*\()(\d+) ([+*]) (\d+)(.+)$").unwrap();
    let re_number_in_parentheses = Regex::new(r"^(.*)\((\d+)\)(.+)$").unwrap();
    let re_done = Regex::new(r"^\((\d+)\)$").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let mut line = format!("({})", line);
        sum += loop {
            if let Some(cap) = re_one_op.captures(&line) {
                let pre = &cap[1];
                let a = cap[2].parse::<usize>().unwrap();
                let op = &cap[3];
                let b = cap[4].parse::<usize>().unwrap();
                let post = &cap[5];
                if op == "+" {
                    line = format!("{}{}{}", pre, a + b, post);
                } else {
                    line = format!("{}{}{}", pre, a * b, post);
                }
            } else if let Some(cap) = re_number_in_parentheses.captures(&line) {
                let pre = &cap[1];
                let n = cap[2].parse::<usize>().unwrap();
                let post = &cap[3];
                line = format!("{}{}{}", pre, n, post);
            } else if let Some(cap) = re_done.captures(&line) {
                break cap[1].parse::<usize>().unwrap();
            }
        }
    }
    sum
}
