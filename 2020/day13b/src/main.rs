static INPUT_A: &str = "939\n7,13,x,x,59,x,31,19";
static INPUT_B: &str = "\n17,x,13,19";
static INPUT_C: &str = "\n67,7,59,61";
static INPUT_D: &str = "\n67,x,7,59,61";
static INPUT_E: &str = "\n67,7,x,59,61";
static INPUT_F: &str = "\n1789,37,47,1889";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 1068781);
    assert_eq!(solve(INPUT_B), 3417);
    assert_eq!(solve(INPUT_C), 754018);
    assert_eq!(solve(INPUT_D), 779210);
    assert_eq!(solve(INPUT_E), 1261476);
    assert_eq!(solve(INPUT_F), 1202161486);
    assert_eq!(solve(INPUT_X), 1001569619313439);

    println!("{:?}", solve(INPUT_X));
}

fn solve(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut timestamp = 0;
    let mut step = 1;

    for (offset, id) in lines[1].split(",").enumerate() {
        if id != "x" {
            let id: usize = id.parse().unwrap();
            while (timestamp + offset) % id != 0 {
                timestamp += step;
            }
            step *= id;
        }
    }

    timestamp
}
