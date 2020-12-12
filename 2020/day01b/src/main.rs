use itertools::iproduct;

static INPUT_A: &str = "1721
979
366
299
675
1456";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 241861950);
    assert_eq!(solve(INPUT_B), 257778836);

    println!("{:?}", solve(INPUT_B));
}

fn solve(input: &str) -> usize {
    let numbers: Vec<usize> = input
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();

    for (a, b, c) in iproduct!(&numbers, &numbers, &numbers) {
        if a + b + c == 2020 {
            return a * b * c;
        }
    }

    panic!();
}
