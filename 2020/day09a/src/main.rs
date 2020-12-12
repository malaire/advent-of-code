static INPUT_A: &str =
    "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A, 5), 127);
    assert_eq!(solve(INPUT_B, 25), 14360655);

    println!("{:?}", solve(INPUT_B, 25));
}

fn solve(input: &str, preamble: usize) -> usize {
    let mut prev: Vec<usize> = Vec::new();

    for n in input.lines().map(|line| line.parse().unwrap()) {
        if prev.len() < preamble {
            prev.push(n)
        } else {
            let mut ok = false;
            for a in &prev {
                for b in &prev {
                    if a + b == n && a != b {
                        ok = true;
                    }
                }
            }
            if !ok {
                return n;
            }
            prev.remove(0);
            prev.push(n);
        }
    }

    panic!();
}
