static INPUT_A: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 7);
    assert_eq!(solve(INPUT_X), 1553);
    println!("{:?}", solve(INPUT_X));

    assert_eq!(solve_2(INPUT_A), 5);
    assert_eq!(solve_2(INPUT_X), 1597);
    println!("{:?}", solve_2(INPUT_X));
}

fn solve(input: &str) -> usize {
    let input: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut count = 0;
    let mut prev = 1_000_000_000;
    for x in input {
        if x > prev {
            count += 1;
        }
        prev = x;
    }

    count
}

fn solve_2(input: &str) -> usize {
    let input: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut count = 0;
    for n in 0..input.len() - 3 {
        if input[n] < input[n + 3] {
            count += 1;
        }
    }

    count
}
