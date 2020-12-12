use itertools::iproduct;

static INPUT_A: &str = "1721
979
366
299
675
1456";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 514579);
    assert_eq!(solve(INPUT_B), 545379);

    println!("{:?}", solve(INPUT_B));
}

fn solve(input: &str) -> usize {
    let numbers: Vec<usize> = input
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();

    for (a, b) in iproduct!(&numbers, &numbers) {
        if a + b == 2020 {
            return a * b;
        }
    }

    panic!();
}
