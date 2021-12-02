use regex::Regex;

static INPUT_A: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 150);
    assert_eq!(solve(INPUT_X), 2272262);
    println!("{:?}", solve(INPUT_X));

    assert_eq!(solve_2(INPUT_A), 900);
    assert_eq!(solve_2(INPUT_X), 2134882034);
    println!("{:?}", solve_2(INPUT_X));
}

fn solve(input: &str) -> usize {
    let re = Regex::new(r"(?m)^(\S+) (\d)$").unwrap();

    let mut horizontal = 0;
    let mut depth = 0;
    for cap in re.captures_iter(input) {
        let dir = &cap[1];
        let n: usize = cap[2].parse().unwrap();
        match dir {
            "forward" => horizontal += n,
            "down" => depth += n,
            "up" => depth -= n,
            _ => panic!(),
        }
    }

    horizontal * depth
}

fn solve_2(input: &str) -> usize {
    let re = Regex::new(r"(?m)^(\S+) (\d)$").unwrap();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for cap in re.captures_iter(input) {
        let dir = &cap[1];
        let n: usize = cap[2].parse().unwrap();
        match dir {
            "forward" => {
                horizontal += n;
                depth += n * aim;
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => panic!(),
        }
    }

    horizontal * depth
}
