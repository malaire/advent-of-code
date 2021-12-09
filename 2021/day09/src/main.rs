use malaire_aoc::run;

static INPUT_A: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 15);
    run(1, solve_1, INPUT_X, 465);

    run(0, solve_2, INPUT_A, 1134);
    run(2, solve_2, INPUT_X, 1269555);
}

fn solve_1(input: &str) -> usize {
    let mut heightmap: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        heightmap.push(line.as_bytes().iter().map(|x| x - b'0').collect());
    }

    let max_row = heightmap.len() - 1;
    let max_col = heightmap[0].len() - 1;

    let mut risk_level_sum: usize = 0;
    for row in 0..=max_row {
        for col in 0..=max_col {
            let height = heightmap[row][col];

            let mut is_low_point = true;

            if row > 0 && height >= heightmap[row - 1][col] {
                is_low_point = false;
            }
            if col > 0 && height >= heightmap[row][col - 1] {
                is_low_point = false;
            }
            if row < max_row && height >= heightmap[row + 1][col] {
                is_low_point = false;
            }
            if col < max_col && height >= heightmap[row][col + 1] {
                is_low_point = false;
            }

            if is_low_point {
                risk_level_sum += height as usize + 1;
            }
        }
    }

    risk_level_sum
}

fn solve_2(input: &str) -> usize {
    let mut heightmap: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        heightmap.push(line.as_bytes().iter().map(|x| x - b'0').collect());
    }

    let max_row = heightmap.len() - 1;
    let max_col = heightmap[0].len() - 1;

    let mut basin_sizes = Vec::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; max_col + 1]; max_row + 1];

    for row in 0..=max_row {
        for col in 0..=max_col {
            if visited[row][col] || heightmap[row][col] == 9 {
                continue;
            }

            let mut basin_size = 0;
            let mut todo = vec![(row, col)];
            while let Some((r, c)) = todo.pop() {
                if visited[r][c] {
                    continue;
                }
                visited[r][c] = true;
                basin_size += 1;

                if r > 0 && heightmap[r - 1][c] != 9 {
                    todo.push((r - 1, c));
                }
                if c > 0 && heightmap[r][c - 1] != 9 {
                    todo.push((r, c - 1));
                }
                if r < max_row && heightmap[r + 1][c] != 9 {
                    todo.push((r + 1, c));
                }
                if c < max_col && heightmap[r][c + 1] != 9 {
                    todo.push((r, c + 1));
                }
            }

            basin_sizes.push(basin_size);
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}
