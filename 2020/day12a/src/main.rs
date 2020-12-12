use regex::Regex;

static INPUT_A: &str = "F10\nN3\nF7\nR90\nF11";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 25);
    assert_eq!(solve(INPUT_B), 820);

    println!("{:?}", solve(INPUT_B));
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
use Direction::*;

fn turn_left(dir: Direction) -> Direction {
    match dir {
        North => West,
        West => South,
        South => East,
        East => North,
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        North => East,
        East => South,
        South => West,
        West => North,
    }
}

fn solve(input: &str) -> i64 {
    let re = Regex::new(r"(?m)^(.)(\d+)$").unwrap();

    let mut ns: i64 = 0; // North positive
    let mut ew: i64 = 0; // East positive
    let mut dir = East;

    for cap in re.captures_iter(input) {
        let action = &cap[1];
        let value: i64 = cap[2].parse().unwrap();
        match action {
            "N" => ns += value,
            "S" => ns -= value,
            "E" => ew += value,
            "W" => ew -= value,
            "L" if value == 90 => dir = turn_left(dir),
            "L" | "R" if value == 180 => dir = turn_left(turn_left(dir)),
            "L" if value == 270 => dir = turn_right(dir),
            "R" if value == 90 => dir = turn_right(dir),
            "R" if value == 270 => dir = turn_left(dir),
            "F" => match dir {
                North => ns += value,
                South => ns -= value,
                East => ew += value,
                West => ew -= value,
            },
            _ => panic!(),
        }
    }

    ns.abs() + ew.abs()
}
