static INPUT_A: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
static INPUT_B: &str = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";

static INPUT_C: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 8);
    assert_eq!(solve(INPUT_B), 19208);
    assert_eq!(solve(INPUT_C), 16198260678656);

    println!("{:?}", solve(INPUT_C));
}

fn solve(input: &str) -> u64 {
    let mut adapters = Vec::new();
    adapters.push(0);
    for adapter in input.lines().map(|line| line.parse().unwrap()) {
        adapters.push(adapter);
    }
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);

    let mut arrangements = vec![0; adapters.len()];
    arrangements[0] = 1;
    for n in 0..adapters.len() {
        for d in 1..=3 {
            if n + d < adapters.len() && adapters[n + d] <= adapters[n] + 3 {
                arrangements[n + d] += arrangements[n];
            }
        }
    }
    *arrangements.last().unwrap()
}
