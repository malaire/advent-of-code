use regex::Regex;

static INPUT_A: &str = "F10\nN3\nF7\nR90\nF11";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 286);
    assert_eq!(solve(INPUT_B), 66614);

    println!("{:?}", solve(INPUT_B));
}

#[derive(Copy, Clone)]
struct Position {
    ns: i64, // North positive
    ew: i64, // East positive
}

fn turn_left(p: Position) -> Position {
    Position {
        ns: p.ew,
        ew: -p.ns,
    }
}

fn turn_right(p: Position) -> Position {
    Position {
        ns: -p.ew,
        ew: p.ns,
    }
}

fn solve(input: &str) -> i64 {
    let re = Regex::new(r"(?m)^(.)(\d+)$").unwrap();

    let mut ship = Position { ns: 0, ew: 0 };
    let mut waypoint = Position { ns: 1, ew: 10 };

    for cap in re.captures_iter(input) {
        let action = &cap[1];
        let value: i64 = cap[2].parse().unwrap();
        match action {
            "N" => waypoint.ns += value,
            "S" => waypoint.ns -= value,
            "E" => waypoint.ew += value,
            "W" => waypoint.ew -= value,
            "L" if value == 90 => waypoint = turn_left(waypoint),
            "L" | "R" if value == 180 => waypoint = turn_left(turn_left(waypoint)),
            "L" if value == 270 => waypoint = turn_right(waypoint),
            "R" if value == 90 => waypoint = turn_right(waypoint),
            "R" if value == 270 => waypoint = turn_left(waypoint),
            "F" => {
                ship.ns += value * waypoint.ns;
                ship.ew += value * waypoint.ew;
            }
            _ => panic!(),
        }
    }

    ship.ns.abs() + ship.ew.abs()
}
