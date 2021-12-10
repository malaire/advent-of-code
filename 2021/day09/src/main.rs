use malaire_aoc::{read_byte_array, run, Tuple2Ext};

static INPUT_A: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 15);
    run(1, solve_1, INPUT_X, 465);

    run(0, solve_2, INPUT_A, 1134);
    run(2, solve_2, INPUT_X, 1269555);
}

fn solve_1(input: &str) -> usize {
    let heightmap = read_byte_array(input).map(|x| x - b'0');

    let mut risk_level_sum = 0;
    for (index, &height) in heightmap.indexed_iter() {
        if index
            .neighbours_orthogonal_iter(heightmap.dim())
            .all(|x| height < heightmap[x])
        {
            risk_level_sum += height as usize + 1;
        }
    }
    risk_level_sum
}

fn solve_2(input: &str) -> usize {
    let heightmap = read_byte_array(input).map(|x| x - b'0');

    let mut basin_sizes = Vec::new();
    let mut visited = heightmap.map(|&x| x == 9);

    for (start_index, _) in heightmap.indexed_iter() {
        if !visited[start_index] {
            let mut basin_size = 0;
            let mut todo = vec![start_index];
            while let Some(index) = todo.pop() {
                if !visited[index] {
                    visited[index] = true;
                    basin_size += 1;
                    todo.extend_from_slice(&index.neighbours_orthogonal(heightmap.dim()));
                }
            }
            basin_sizes.push(basin_size);
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes[0..3].iter().product()
}
