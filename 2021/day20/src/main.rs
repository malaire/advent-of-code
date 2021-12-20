use std::collections::HashSet;

use malaire_aoc::{read_byte_array, run};

static INPUT_A: &str = include_str!("input_a");
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 35);
    run(1, solve_1, INPUT_X, 5391);

    run(0, solve_2, INPUT_A, 3351);
    run(2, solve_2, INPUT_X, 16383);
}

fn solve_1(input: &str) -> usize {
    solve_generic(input, 2)
}

fn solve_2(input: &str) -> usize {
    solve_generic(input, 50)
}

fn solve_generic(input: &str, steps: usize) -> usize {
    let sections: Vec<_> = input.split("\n\n").collect();

    let image_enhancement_algorithm = sections[0].as_bytes();

    let mut image = HashSet::new();
    for (index, &pixel) in read_byte_array(sections[1]).indexed_iter() {
        if pixel == b'#' {
            image.insert((index.0 as i64, index.1 as i64));
        }
    }

    let mut outside_pixels_are_light = false;

    for _step in 1..=steps {
        let mut new_image = HashSet::new();

        let min_row = *image.iter().map(|(row, _)| row).min().unwrap();
        let max_row = *image.iter().map(|(row, _)| row).max().unwrap();

        let min_col = *image.iter().map(|(_, col)| col).min().unwrap();
        let max_col = *image.iter().map(|(_, col)| col).max().unwrap();

        for row in min_row - 1..=max_row + 1 {
            for col in min_col - 1..=max_col + 1 {
                let mut value = 0;

                for (row_delta, col_delta) in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 0),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    value <<= 1;
                    if row + row_delta < min_row
                        || row + row_delta > max_row
                        || col + col_delta < min_col
                        || col + col_delta > max_col
                    {
                        value += outside_pixels_are_light as usize;
                    } else if image.contains(&(row + row_delta, col + col_delta)) {
                        value += 1;
                    }
                }

                if image_enhancement_algorithm[value] == b'#' {
                    new_image.insert((row, col));
                }
            }
        }

        image = new_image;

        let outside_value = if outside_pixels_are_light { 511 } else { 0 };
        outside_pixels_are_light = image_enhancement_algorithm[outside_value] == b'#';
    }

    image.len()
}
