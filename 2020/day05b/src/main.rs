static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_B), 519);

    println!("{:?}", solve(INPUT_B));
}

fn solve(input: &str) -> usize {
    let mut taken = vec![false; 1024];

    for line in input.lines() {
        let mut seat_id = 0;
        for ch in line.chars() {
            seat_id *= 2;
            if ch == 'B' || ch == 'R' {
                seat_id += 1;
            }
        }
        taken[seat_id] = true;
    }

    let mut skipped_initial_empty_seats = false;
    for (seat_id, &taken) in taken.iter().enumerate() {
        if taken {
            skipped_initial_empty_seats = true;
        } else if !taken && skipped_initial_empty_seats {
            return seat_id;
        }
    }

    panic!();
}
