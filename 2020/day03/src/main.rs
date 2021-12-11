use malaire_aoc::run;

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

static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 7);
    run(1, solve_1, INPUT_X, 272);

    run(0, solve_2, INPUT_A, 336);
    run(2, solve_2, INPUT_X, 3898725600);
}

fn solve_1(input: &str) -> usize {
    solve_generic(input, &[(3, 1)])
}

fn solve_2(input: &str) -> usize {
    solve_generic(input, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
}

fn solve_generic(input: &str, slopes: &[(usize, usize)]) -> usize {
    let mut multiplied_tree_counts = 1;

    for (right, down) in slopes {
        let mut tree_count = 0;
        let mut pos_x = 0;
        let mut pos_y = 0;

        for line in input.lines().skip(1) {
            pos_y += 1;

            if pos_y < *down {
                continue;
            } else {
                pos_y = 0;
                pos_x = (pos_x + right) % line.trim().len();
            }

            if line.as_bytes()[pos_x] == b'#' {
                tree_count += 1;
            }
        }

        multiplied_tree_counts *= tree_count;
    }

    multiplied_tree_counts
}
