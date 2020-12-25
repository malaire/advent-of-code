static INPUT_A: (usize, usize) = (5764801, 17807724);
static INPUT_X: (usize, usize) = (3418282, 8719412);

fn main() {
    assert_eq!(solve(INPUT_A), 14897079);
    assert_eq!(solve(INPUT_X), 9620012);

    println!("{:?}", solve(INPUT_X));
}

fn solve(input: (usize, usize)) -> usize {
    let (public_a, public_b) = input;

    let mut value = 1;
    let mut loops = 0;

    while value != public_a && value != public_b {
        value = (value * 7) % 20201227;
        loops += 1;
    }

    let loop_size = loops;
    let the_other_public_key = if value == public_a {
        public_b
    } else {
        public_a
    };

    value = 1;
    for _ in 0..loop_size {
        value = (value * the_other_public_key) % 20201227;
    }
    value
}
