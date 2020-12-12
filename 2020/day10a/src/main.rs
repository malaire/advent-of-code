static INPUT_A: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
static INPUT_B: &str = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";

static INPUT_C: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 7 * 5);
    assert_eq!(solve(INPUT_B), 22 * 10);
    assert_eq!(solve(INPUT_C), 2100);

    println!("{:?}", solve(INPUT_C));
}

fn solve(input: &str) -> usize {
    let mut adapters: Vec<usize> = Vec::new();
    adapters.push(0);

    for n in input.lines().map(|line| line.parse().unwrap()) {
        adapters.push(n);
    }
    adapters.sort();

    let mut one_counts = 0;
    let mut three_counts = 1;

    for n in 1..adapters.len() {
        let delta = adapters[n] - adapters[n - 1];
        if delta == 1 {
            one_counts += 1;
        } else if delta == 3 {
            three_counts += 1;
        }
    }

    one_counts * three_counts
}
