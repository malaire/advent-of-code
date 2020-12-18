use regex::Regex;

static INPUT_A: &str = "1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)";

static INPUT_B: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
static INPUT_C: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 231 + 51 + 46 + 1445);
    assert_eq!(solve(INPUT_B), 669060);
    assert_eq!(solve(INPUT_C), 23340);
    assert_eq!(solve(INPUT_X), 216975281211165);

    println!("{:?}", solve(INPUT_X));
}

fn calculate(expr: &str) -> usize {
    let re_parentheses = Regex::new(r"^(.+)\(([^()]+)\)(.+)$").unwrap();
    let re_add = Regex::new(r"^(.*?)(\d+) \+ (\d+)(.+)$").unwrap();
    let re_mul = Regex::new(r"^(.*?)(\d+) \* (\d+)(.+)$").unwrap();
    let re_done = Regex::new(r"^\((\d+)\)$").unwrap();

    let mut expr = format!("({})", expr);
    loop {
        if let Some(cap) = re_parentheses.captures(&expr) {
            let pre = &cap[1];
            let sub_expr = &cap[2];
            let post = &cap[3];
            expr = format!("{}{}{}", pre, calculate(sub_expr), post);
        } else if let Some(cap) = re_add.captures(&expr) {
            let pre = &cap[1];
            let a = cap[2].parse::<usize>().unwrap();
            let b = cap[3].parse::<usize>().unwrap();
            let post = &cap[4];
            expr = format!("{}{}{}", pre, a + b, post);
        } else if let Some(cap) = re_mul.captures(&expr) {
            let pre = &cap[1];
            let a = cap[2].parse::<usize>().unwrap();
            let b = cap[3].parse::<usize>().unwrap();
            let post = &cap[4];
            expr = format!("{}{}{}", pre, a * b, post);
        } else if let Some(cap) = re_done.captures(&expr) {
            return cap[1].parse::<usize>().unwrap();
        }
    }
}

fn solve(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        sum += calculate(line);
    }
    sum
}
