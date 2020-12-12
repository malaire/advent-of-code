static INPUT_A: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 37);
    assert_eq!(solve(INPUT_B), 2438);

    println!("{:?}", solve(INPUT_B));
}

#[derive(Copy, Clone, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}
use Seat::*;

fn get(row: i64, col: i64, rows: i64, cols: i64, grid: &mut Vec<Seat>) -> Seat {
    if row < 0 || col < 0 || row >= rows || col >= cols {
        Floor
    } else {
        grid[(row * cols + col) as usize]
    }
}

fn solve(input: &str) -> usize {
    let mut grid = Vec::new();
    let mut rows: i64 = 0;
    let cols: i64;
    for line in input.lines() {
        rows += 1;
        for seat in line.chars() {
            grid.push(match seat {
                '.' => Floor,
                'L' => Empty,
                _ => panic!(),
            });
        }
    }
    cols = grid.len() as i64 / rows;

    let mut changed = true;
    while changed {
        let mut new_grid = Vec::new();
        changed = false;
        for row in 0..rows {
            for col in 0..cols {
                let mut adjacent = 0;
                if get(row + 1, col + 1, rows, cols, &mut grid) == Occupied {
                    adjacent += 1
                }
                if get(row + 1, col, rows, cols, &mut grid) == Occupied {
                    adjacent += 1
                }
                if get(row + 1, col - 1, rows, cols, &mut grid) == Occupied {
                    adjacent += 1
                }
                if get(row, col + 1, rows, cols, &mut grid) == Occupied {
                    adjacent += 1
                }
                if get(row, col - 1, rows, cols, &mut grid) == Occupied {
                    adjacent += 1
                }
                if get(row - 1, col + 1, rows, cols, &mut grid) == Occupied {
                    adjacent += 1
                }
                if get(row - 1, col, rows, cols, &mut grid) == Occupied {
                    adjacent += 1
                }
                if get(row - 1, col - 1, rows, cols, &mut grid) == Occupied {
                    adjacent += 1
                }

                let seat = get(row, col, rows, cols, &mut grid);
                let new_seat = match seat {
                    Empty if adjacent == 0 => Occupied,
                    Occupied if adjacent >= 4 => Empty,
                    other => other,
                };
                if new_seat != seat {
                    changed = true
                }
                new_grid.push(new_seat)
            }
        }
        grid = new_grid;
    }

    let mut occupied = 0;
    for &seat in &grid {
        if seat == Occupied {
            occupied += 1
        }
    }

    occupied
}
