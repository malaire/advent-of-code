static INPUT_A: &str = "0,3,6";
static INPUT_B: &str = "1,3,2";
static INPUT_C: &str = "2,1,3";

static INPUT_X: &str = "12,20,0,6,1,17,7";

fn main() {
    // PART 1

    assert_eq!(solve(INPUT_A, 2020), 436);
    assert_eq!(solve(INPUT_B, 2020), 1);
    assert_eq!(solve(INPUT_C, 2020), 10);
    assert_eq!(solve(INPUT_X, 2020), 866);

    println!("{:?}", solve(INPUT_X, 2020));

    // PART 2

    assert_eq!(solve(INPUT_A, 30000000), 175594);
    assert_eq!(solve(INPUT_B, 30000000), 2578);
    assert_eq!(solve(INPUT_C, 30000000), 3544142);
    assert_eq!(solve(INPUT_X, 30000000), 1437692);

    println!("{:?}", solve(INPUT_X, 30000000));
}

fn solve(input: &str, last_turn: usize) -> usize {
    let mut turn: usize = 1;
    let mut spoken: Vec<i32> = vec![-1; last_turn];
    let mut latest = None;

    for n in input.split(",").map(|n| n.parse::<usize>().unwrap()) {
        if let Some(latest) = latest {
            spoken[latest] = turn as i32;
        }
        turn += 1;
        latest = Some(n);
    }
    let mut latest = latest.unwrap();

    while turn <= last_turn {
        let next = if spoken[latest] != -1 {
            turn - spoken[latest] as usize
        } else {
            0
        };
        spoken[latest] = turn as i32;

        turn += 1;
        latest = next;
    }

    latest
}
