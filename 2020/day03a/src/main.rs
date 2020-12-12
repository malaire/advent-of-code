static INPUT_A: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 7);
    assert_eq!(solve(INPUT_B), 272);

    println!("{:?}", solve(INPUT_B));
}

fn solve(input: &str) -> usize {
    let mut trees: usize = 0;
    let mut pos: usize = 0;

    for line in input.lines().skip(1) {
        pos = (pos + 3) % line.trim().len();

        if line.as_bytes()[pos] == b'#' {
            trees += 1;
        }
    }

    trees
}
