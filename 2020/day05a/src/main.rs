static INPUT_A: &str = "FBFBBFFRLR
BFFFBBFRRR";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 567);
    assert_eq!(solve(INPUT_B), 906);

    println!("{:?}", solve(INPUT_B));
}

fn solve(input: &str) -> usize {
    let mut highest = 0;

    for line in input.lines() {
        let mut seat_id = 0;
        for ch in line.chars() {
            seat_id *= 2;
            if ch == 'B' || ch == 'R' {
                seat_id += 1;
            }
        }
        if seat_id > highest {
            highest = seat_id;
        }
    }

    highest
}
