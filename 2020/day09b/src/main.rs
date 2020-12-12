static INPUT_A: &str =
    "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A, 127), 62);
    assert_eq!(solve(INPUT_B, 14360655), 1962331);

    println!("{:?}", solve(INPUT_B, 14360655));
}

fn solve(input: &str, sum: usize) -> usize {
    let mut all: Vec<usize> = Vec::new();

    for n in input.lines().map(|line| line.parse().unwrap()) {
        all.push(n);
    }

    for first in 0..all.len() {
        for last in (first + 1)..all.len() {
            let mut range_sum = 0;
            let mut min = 1_000_000_000_000;
            let mut max = 0;
            for index in first..(last + 1) {
                let n = all[index];
                range_sum += n;
                if n > max {
                    max = n
                }
                if n < min {
                    min = n
                }
            }
            if sum == range_sum {
                return min + max;
            }
        }
    }

    panic!();
}
