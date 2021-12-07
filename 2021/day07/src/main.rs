static INPUT_A: &str = "16,1,2,0,4,2,7,1,2,14";
static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve_1(INPUT_A), 37);
    let x = solve_1(INPUT_X);
    assert_eq!(x, 340056);
    println!("{:?}", x);

    assert_eq!(solve_2(INPUT_A), 168);
    let x = solve_2(INPUT_X);
    assert_eq!(x, 96592275);
    println!("{:?}", x);
}

fn solve_1(input: &str) -> isize {
    let input: Vec<isize> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut min_fuel = isize::MAX;
    for target in min..=max {
        let mut fuel = 0;
        for pos in &input {
            fuel += (pos - target).abs();
        }
        min_fuel = min_fuel.min(fuel);
    }

    min_fuel
}

fn solve_2(input: &str) -> isize {
    let input: Vec<isize> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut min_fuel = isize::MAX;
    for target in min..=max {
        let mut fuel = 0;
        for pos in &input {
            let delta = (pos - target).abs();
            fuel += (1 + delta) * delta / 2;
        }
        min_fuel = min_fuel.min(fuel);
    }

    min_fuel
}
