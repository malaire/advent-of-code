use std::collections::HashMap;

static INPUT_A: &str = "0,3,6";
static INPUT_B: &str = "1,3,2";
static INPUT_C: &str = "2,1,3";

static INPUT_X: &str = "12,20,0,6,1,17,7";

fn main() {
    assert_eq!(solve(INPUT_A), 175594);
    assert_eq!(solve(INPUT_B), 2578);
    assert_eq!(solve(INPUT_C), 3544142);

    println!("{:?}", solve(INPUT_X));
}

fn solve(input: &str) -> usize {
    let mut turn = 1;
    let mut spoken: HashMap<usize, usize> = HashMap::new();
    let mut latest = None;

    for n in input.split(",").map(|n| n.parse::<usize>().unwrap()) {
        if let Some(latest) = latest {
            spoken.insert(latest, turn);
        }
        turn += 1;
        latest = Some(n);
    }
    let mut latest = latest.unwrap();

    while turn <= 30000000 {
        let next = if let Some(last_spoken) = spoken.get(&latest) {
            turn - last_spoken
        } else {
            0
        };
        spoken.insert(latest, turn);

        turn += 1;
        latest = next;
    }

    latest
}
