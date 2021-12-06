static INPUT_A: &str = "3,4,3,1,2";
static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve_1(INPUT_A), 5934);
    let x = solve_1(INPUT_X);
    assert_eq!(x, 391888);
    println!("{:?}", x);

    assert_eq!(solve_2(INPUT_A), 26984457539);
    let x = solve_2(INPUT_X);
    assert_eq!(x, 1754597645339);
    println!("{:?}", x);
}

fn solve_1(input: &str) -> usize {
    let mut fishes: Vec<usize> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    for _day in 1..=80 {
        let mut new_count = 0;
        for fish in &mut fishes {
            if *fish > 0 {
                *fish -= 1;
            } else {
                *fish = 6;
                new_count += 1;
            }
        }

        for _ in 0..new_count {
            fishes.push(8);
        }
    }

    fishes.len()
}

fn solve_2(input: &str) -> usize {
    let fishes: Vec<usize> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut fishes_per_timer: [usize; 9] = [0; 9];

    for fish in fishes {
        fishes_per_timer[fish] += 1;
    }

    for _day in 1..=256 {
        let new_count = fishes_per_timer[0];
        fishes_per_timer[0] = fishes_per_timer[1];
        fishes_per_timer[1] = fishes_per_timer[2];
        fishes_per_timer[2] = fishes_per_timer[3];
        fishes_per_timer[3] = fishes_per_timer[4];
        fishes_per_timer[4] = fishes_per_timer[5];
        fishes_per_timer[5] = fishes_per_timer[6];
        fishes_per_timer[6] = fishes_per_timer[7] + new_count;
        fishes_per_timer[7] = fishes_per_timer[8];
        fishes_per_timer[8] = new_count;
    }

    fishes_per_timer.iter().sum()
}
