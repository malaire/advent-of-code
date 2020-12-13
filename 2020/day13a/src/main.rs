static INPUT_A: &str = "939
7,13,x,x,59,x,31,19";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 295);
    assert_eq!(solve(INPUT_X), 3215);

    println!("{:?}", solve(INPUT_X));
}

fn solve(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\n").collect();
    let depart_time: usize = lines[0].parse().unwrap();

    let mut best_bus = 0;
    let mut best_wait_time = 1_000_000_000;

    for bus in lines[1].split(",") {
        if bus == "x" {
            continue;
        } else {
            let bus: usize = bus.parse().unwrap();
            let wait_time = bus - (depart_time % bus);
            if wait_time < best_wait_time {
                best_wait_time = wait_time;
                best_bus = bus;
            }
        }
    }

    best_bus * best_wait_time
}
