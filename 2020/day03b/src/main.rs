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
    assert_eq!(solve(INPUT_A), 336);
    assert_eq!(solve(INPUT_B), 3898725600);

    println!("{:?}", solve(INPUT_B));
}

fn solve(input: &str) -> usize {
    let a = traverse(input, 1, 1);
    let b = traverse(input, 3, 1);
    let c = traverse(input, 5, 1);
    let d = traverse(input, 7, 1);
    let e = traverse(input, 1, 2);
    a * b * c * d * e
}

fn traverse(input: &str, right: usize, down: usize) -> usize {
    let mut trees: usize = 0;
    let mut pos_x: usize = 0;
    let mut pos_y: usize = 0;

    for line in input.lines().skip(1) {
        pos_y += 1;

        if pos_y < down {
            continue;
        } else {
            pos_y = 0;
            pos_x = (pos_x + right) % line.trim().len();
        }

        if line.as_bytes()[pos_x] == b'#' {
            trees += 1;
        }
    }

    trees
}
