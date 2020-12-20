use array2d::Array2D;
use regex::Regex;
use std::collections::HashSet;

static INPUT_A: &str = include_str!("input_a");
static INPUT_X: &str = include_str!("input");

fn main() {
    // PART 1

    assert_eq!(solve_1(INPUT_A), 20899048083289);
    assert_eq!(solve_1(INPUT_X), 32287787075651);

    println!("{:?}", solve_1(INPUT_X));

    // PART 2

    assert_eq!(solve_2(INPUT_A), 273);
    assert_eq!(solve_2(INPUT_X), 1939);

    println!("{:?}", solve_2(INPUT_X));
}

// ================================================================================
// Tile

const TILE_SIZE: usize = 10;

struct Tile {
    id: usize,
    orientations: Vec<Orientation>,
}

impl Tile {
    fn new(id: usize, data: &Vec<bool>) -> Self {
        let array = Array2D::from_row_major(&data, TILE_SIZE, TILE_SIZE);

        let a0 = array;
        let a1 = rotate_array_cw(&a0);
        let a2 = rotate_array_cw(&a1);
        let a3 = rotate_array_cw(&a2);
        let b0 = flip_array(&a0);
        let b1 = rotate_array_cw(&b0);
        let b2 = rotate_array_cw(&b1);
        let b3 = rotate_array_cw(&b2);

        let orientations = vec![
            Orientation::new(a0),
            Orientation::new(a1),
            Orientation::new(a2),
            Orientation::new(a3),
            Orientation::new(b0),
            Orientation::new(b1),
            Orientation::new(b2),
            Orientation::new(b3),
        ];

        Tile { id, orientations }
    }
}

// ================================================================================
// Orientation

struct Orientation {
    array: Array2D<bool>,
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
}

impl Orientation {
    fn new(array: Array2D<bool>) -> Self {
        fn to_bitvector<'a>(items: &mut impl Iterator<Item = &'a bool>) -> usize {
            let mut bitvector = 0;
            for item in items {
                bitvector *= 2;
                bitvector += *item as usize;
            }
            bitvector
        }

        let top = to_bitvector(&mut array.row_iter(0));
        let left = to_bitvector(&mut array.column_iter(0));
        let bottom = to_bitvector(&mut array.row_iter(TILE_SIZE - 1));
        let right = to_bitvector(&mut array.column_iter(TILE_SIZE - 1));

        Orientation {
            array,
            top,
            left,
            bottom,
            right,
        }
    }
}

// ================================================================================
// SeaMonster

static SEA_MONSTER: &str = "                  #
#    ##    ##    ###
 #  #  #  #  #  #";

struct SeaMonster {
    locations: Vec<(usize, usize)>,
    rows: usize,
    cols: usize,
}

impl SeaMonster {
    fn new(data: &str) -> Self {
        let mut locations = Vec::new();
        let mut max_row = 0;
        let mut max_col = 0;

        let mut row = 0;
        for line in data.lines() {
            let mut col = 0;
            for ch in line.chars() {
                if ch == '#' {
                    locations.push((row, col));
                }
                col += 1;
                if col > max_col {
                    max_col = col;
                }
            }
            row += 1;
            if row > max_row {
                max_row = row;
            }
        }

        SeaMonster {
            locations,
            rows: max_row + 1,
            cols: max_col + 1,
        }
    }
}

// ================================================================================
// ARRAY HELPERS

fn rotate_array_cw<T: Copy>(array: &Array2D<T>) -> Array2D<T> {
    let mut data = Vec::with_capacity(array.num_elements());
    for row in 0..array.num_columns() {
        for col in 0..array.num_rows() {
            data.push(array[(array.num_rows() - 1 - col, row)]);
        }
    }
    Array2D::from_row_major(&data, array.num_columns(), array.num_rows())
}

// row/col flipped
fn flip_array<T: Copy>(array: &Array2D<T>) -> Array2D<T> {
    Array2D::from_iter_column_major(
        array.elements_row_major_iter().cloned(),
        array.num_columns(),
        array.num_rows(),
    )
}

// ================================================================================
// MAJOR FUNCTIONS

fn parse_input(input: &str) -> (Vec<Tile>, usize) {
    let re_id = Regex::new(r"^Tile (\d+):$").unwrap();

    let mut tiles: Vec<Tile> = Vec::new();

    let mut id = None;
    let mut data = Vec::with_capacity(TILE_SIZE * TILE_SIZE);
    for line in input.lines() {
        if let Some(cap) = re_id.captures(line) {
            id = Some(cap[1].parse::<usize>().unwrap());
            data.clear();
        } else if line != "" {
            for ch in line.chars() {
                data.push(ch == '#');
            }
        } else {
            tiles.push(Tile::new(id.unwrap(), &data));
        }
    }
    tiles.push(Tile::new(id.unwrap(), &data));

    let mut size = 1;
    while size * size < tiles.len() {
        size += 1;
    }
    if tiles.len() != size * size {
        panic!("n*n tiles required");
    }

    (tiles, size)
}

// <usize, usize> = tile position in tiles ; its orientation
fn find_arrangement(tiles: &Vec<Tile>, size: usize) -> Array2D<(usize, usize)> {
    const ORIENTATION_COUNT: usize = 8;

    fn next_t_o(t: usize, o: usize) -> (usize, usize) {
        if o < ORIENTATION_COUNT - 1 {
            (t, o + 1)
        } else {
            (t + 1, 0)
        }
    }

    let mut arrangement: Array2D<Option<(usize, usize)>> = Array2D::filled_with(None, size, size);
    let mut used_tiles: HashSet<usize> = HashSet::new();
    let mut row = 0;
    let mut col = 0;

    loop {
        let top = if row == 0 {
            None
        } else {
            let (t, o) = arrangement[(row - 1, col)].unwrap();
            Some(tiles[t].orientations[o].bottom)
        };

        let left = if col == 0 {
            None
        } else {
            let (t, o) = arrangement[(row, col - 1)].unwrap();
            Some(tiles[t].orientations[o].right)
        };

        // FIND FITTING TILE

        let (mut t, mut o) = match arrangement[(row, col)] {
            None => (0, 0),
            Some((tt, oo)) => next_t_o(tt, oo),
        };
        let mut backtrack = t == size * size;
        while !backtrack {
            let mut ok = true;
            if used_tiles.contains(&t) {
                ok = false;
            } else {
                let orientation = &tiles[t].orientations[o];
                if let Some(top) = top {
                    if top != orientation.top {
                        ok = false;
                    }
                }
                if let Some(left) = left {
                    if left != orientation.left {
                        ok = false;
                    }
                }
            }

            if ok {
                break;
            } else {
                let (tt, oo) = next_t_o(t, o);
                t = tt;
                o = oo;

                if t == size * size {
                    backtrack = true;
                }
            }
        }

        if let Some((t, _)) = arrangement[(row, col)] {
            used_tiles.remove(&t);
        }

        if backtrack {
            arrangement[(row, col)] = None;
            if col > 0 {
                col -= 1;
            } else if row > 0 {
                row -= 1;
                col = size - 1;
            } else {
                panic!("NO SOLUTION FOUND");
            }
        } else {
            arrangement[(row, col)] = Some((t, o));
            used_tiles.insert(t);

            if col < size - 1 {
                col += 1;
            } else if row < size - 1 {
                row += 1;
                col = 0;
            } else {
                break;
            }
        }
    }

    Array2D::from_iter_row_major(
        arrangement.elements_row_major_iter().map(|x| x.unwrap()),
        size,
        size,
    )
}

// ================================================================================
// SOLVE

fn solve_1(input: &str) -> usize {
    let (tiles, size) = parse_input(input);
    let arrangement = find_arrangement(&tiles, size);

    let a = arrangement[(0, 0)];
    let b = arrangement[(size - 1, 0)];
    let c = arrangement[(0, size - 1)];
    let d = arrangement[(size - 1, size - 1)];

    tiles[a.0].id * tiles[b.0].id * tiles[c.0].id * tiles[d.0].id
}

fn solve_2(input: &str) -> usize {
    let sea_monster = SeaMonster::new(SEA_MONSTER);

    let (tiles, size) = parse_input(input);
    let arrangement = find_arrangement(&tiles, size);

    let image_size = size * (TILE_SIZE - 2);
    let mut image: Vec<bool> = Vec::with_capacity(image_size * image_size);
    for row in 0..image_size {
        for col in 0..image_size {
            let row_major = row / (TILE_SIZE - 2);
            let col_major = col / (TILE_SIZE - 2);
            let row_minor = row % (TILE_SIZE - 2) + 1;
            let col_minor = col % (TILE_SIZE - 2) + 1;

            let (t, o) = arrangement[(row_major, col_major)];
            image.push(tiles[t].orientations[o].array[(row_minor, col_minor)]);
        }
    }
    let image = Array2D::from_row_major(&image, image_size, image_size);

    let a0 = image;
    let a1 = rotate_array_cw(&a0);
    let a2 = rotate_array_cw(&a1);
    let a3 = rotate_array_cw(&a2);
    let b0 = flip_array(&a0);
    let b1 = rotate_array_cw(&b0);
    let b2 = rotate_array_cw(&b1);
    let b3 = rotate_array_cw(&b2);

    let images = vec![a0, a1, a2, a3, b0, b1, b2, b3];

    for image in images {
        let mut sea_monster_count: usize = 0;
        for row in 0..=(image_size - sea_monster.rows) {
            for col in 0..=(image_size - sea_monster.cols) {
                let mut monster_found = true;
                for (row_delta, col_delta) in &sea_monster.locations {
                    if !image[(row + row_delta, col + col_delta)] {
                        monster_found = false;
                        break;
                    }
                }
                if monster_found {
                    sea_monster_count += 1;
                }
            }
        }

        if sea_monster_count > 0 {
            let mut roughness: usize = 0;
            for item in image.elements_row_major_iter() {
                roughness += *item as usize;
            }
            return roughness - sea_monster_count * sea_monster.locations.len();
        }
    }

    panic!();
}
