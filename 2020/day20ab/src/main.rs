use regex::Regex;
use std::collections::{HashMap, HashSet};

use malaire_aoc::array2d::{Array2D, Vec2D, View};

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

const TILE_SIZE: usize = 10;
const TILE_SIZE_WITHOUT_BORDERS: usize = TILE_SIZE - 2;

// ================================================================================
// BitVector

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct BitVector(u16);

impl BitVector {
    fn new<'a>(items: &mut impl Iterator<Item = &'a bool>) -> Self {
        let mut bitvector = 0;
        for item in items {
            bitvector *= 2;
            if *item {
                bitvector += 1
            }
        }
        BitVector(bitvector)
    }
}

// ================================================================================
// Orientation

struct Orientation<'a> {
    id: usize,
    view: View<'a, Vec2D<bool>>,
    top: BitVector,
    left: BitVector,
    bottom: BitVector,
    right: BitVector,
}

impl<'a> Orientation<'a> {
    fn new(id: usize, view_with_borders: View<'a, Vec2D<bool>>) -> Self {
        let top = BitVector::new(&mut view_with_borders.row(0));
        let left = BitVector::new(&mut view_with_borders.col(0));
        let bottom = BitVector::new(&mut view_with_borders.row(TILE_SIZE - 1));
        let right = BitVector::new(&mut view_with_borders.col(TILE_SIZE - 1));

        Orientation {
            id,
            view: view_with_borders.window(
                1,
                1,
                TILE_SIZE_WITHOUT_BORDERS,
                TILE_SIZE_WITHOUT_BORDERS,
            ),
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
// MAJOR FUNCTIONS

fn parse_input(input: &str) -> Vec<(usize, Vec2D<bool>)> {
    let re_id = Regex::new(r"^Tile (\d+):$").unwrap();

    let mut tiles_data = Vec::new();

    let mut id = None;
    let mut data = None;
    for line in input.lines() {
        if let Some(cap) = re_id.captures(line) {
            id = Some(cap[1].parse::<usize>().unwrap());
            data = Some(Vec::with_capacity(TILE_SIZE * TILE_SIZE));
        } else if line != "" {
            for ch in line.chars() {
                data.as_mut().unwrap().push(ch == '#');
            }
        } else {
            let array = Vec2D::from_row_major(data.take().unwrap(), TILE_SIZE, TILE_SIZE);
            tiles_data.push((id.unwrap(), array));
        }
    }
    let array: Vec2D<bool> = Vec2D::from_row_major(data.take().unwrap(), TILE_SIZE, TILE_SIZE);
    tiles_data.push((id.unwrap(), array));

    tiles_data
}

fn get_orientations<'a>(
    tiles_data: &'a Vec<(usize, Vec2D<bool>)>,
) -> (Vec<Orientation<'a>>, usize) {
    let mut orientations = Vec::new();
    let mut tile_count = 0;

    for (id, array) in tiles_data {
        let v0 = array.view();
        let v1 = v0.clone().cw(90);
        let v2 = v1.clone().cw(90);
        let v3 = v2.clone().cw(90);
        let v4 = v0.clone().transpose();
        let v5 = v4.clone().cw(90);
        let v6 = v5.clone().cw(90);
        let v7 = v6.clone().cw(90);

        orientations.push(Orientation::new(*id, v0));
        orientations.push(Orientation::new(*id, v1));
        orientations.push(Orientation::new(*id, v2));
        orientations.push(Orientation::new(*id, v3));
        orientations.push(Orientation::new(*id, v4));
        orientations.push(Orientation::new(*id, v5));
        orientations.push(Orientation::new(*id, v6));
        orientations.push(Orientation::new(*id, v7));

        tile_count += 1;
    }

    let mut size = 1;
    while size * size < tile_count {
        size += 1;
    }
    if tile_count != size * size {
        panic!("n*n tiles required");
    }

    (orientations, size)
}

fn find_arrangement<'a>(
    orientations: &'a Vec<Orientation<'a>>,
    size: usize,
) -> Vec2D<&'a Orientation> {
    // [(top, left)] = matching orientations
    let mut possible_orientations: HashMap<
        (Option<BitVector>, Option<BitVector>),
        Vec<&Orientation>,
    > = HashMap::new();
    for orientation in orientations.iter() {
        let top = orientation.top;
        let left = orientation.left;

        for key in vec![
            (None, None),
            (None, Some(left)),
            (Some(top), None),
            (Some(top), Some(left)),
        ] {
            possible_orientations
                .entry(key)
                .or_insert(Vec::new())
                .push(orientation);
        }
    }

    // possible orientations of each position, with index of currently selected one
    let mut arrangement: Vec2D<Option<(usize, &Vec<&Orientation>)>> =
        Vec2D::repeat_item(None, size, size);
    let mut used_tiles: HashSet<usize> = HashSet::new();
    let mut row = 0;
    let mut col = 0;

    loop {
        let backtrack;
        if let Some((mut selected, possible)) = arrangement[(row, col)] {
            let prev_selected = selected;
            while selected < possible.len() && used_tiles.contains(&possible[selected].id) {
                selected += 1;
            }
            used_tiles.remove(&possible[prev_selected].id);

            if selected < possible.len() {
                arrangement[(row, col)].as_mut().unwrap().0 = selected;
                used_tiles.insert(possible[selected].id);
                backtrack = false;
            } else {
                arrangement[(row, col)] = None;
                backtrack = true;
            }
        } else {
            let top = if row == 0 {
                None
            } else {
                let (selected, possible) = arrangement[(row - 1, col)].unwrap();
                Some(possible[selected].bottom)
            };

            let left = if col == 0 {
                None
            } else {
                let (selected, possible) = arrangement[(row, col - 1)].unwrap();
                Some(possible[selected].right)
            };

            match &possible_orientations.get(&(top, left)) {
                Some(possible) => {
                    let mut selected = 0;
                    while selected < possible.len() && used_tiles.contains(&possible[selected].id) {
                        selected += 1;
                    }
                    if selected < possible.len() {
                        arrangement[(row, col)] = Some((selected, possible));
                        used_tiles.insert(possible[selected].id);
                        backtrack = false;
                    } else {
                        backtrack = true;
                    }
                }
                None => backtrack = true,
            }
        }

        if backtrack {
            if col > 0 {
                col -= 1;
            } else if row > 0 {
                row -= 1;
                col = size - 1;
            } else {
                panic!("NO SOLUTION FOUND");
            }
        } else {
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

    Vec2D::from_row_major(
        arrangement.row_major().map(|a| {
            let (selected, possible) = a.unwrap();
            possible[selected]
        }),
        size,
        size,
    )
}

// ================================================================================
// SOLVE

fn solve_1(input: &str) -> usize {
    let tiles_data = parse_input(input);
    let (orientations, size) = get_orientations(&tiles_data);
    let arrangement = find_arrangement(&orientations, size);

    let a = arrangement[(0, 0)];
    let b = arrangement[(size - 1, 0)];
    let c = arrangement[(0, size - 1)];
    let d = arrangement[(size - 1, size - 1)];

    a.id * b.id * c.id * d.id
}

fn solve_2(input: &str) -> usize {
    let sea_monster = SeaMonster::new(SEA_MONSTER);

    let tiles_data = parse_input(input);
    let (orientations, size) = get_orientations(&tiles_data);
    let arrangement = find_arrangement(&orientations, size);

    let image_size = size * TILE_SIZE_WITHOUT_BORDERS;
    let mut image: Vec<bool> = Vec::with_capacity(image_size * image_size);
    for row in 0..image_size {
        for col in 0..image_size {
            image.push(
                arrangement[(
                    row / TILE_SIZE_WITHOUT_BORDERS,
                    col / TILE_SIZE_WITHOUT_BORDERS,
                )]
                    .view[(
                    row % TILE_SIZE_WITHOUT_BORDERS,
                    col % TILE_SIZE_WITHOUT_BORDERS,
                )],
            );
        }
    }
    let image = Vec2D::from_row_major(image, image_size, image_size);

    let v0 = image.view();
    let v1 = v0.clone().cw(90);
    let v2 = v1.clone().cw(90);
    let v3 = v2.clone().cw(90);
    let v4 = v0.clone().transpose();
    let v5 = v4.clone().cw(90);
    let v6 = v5.clone().cw(90);
    let v7 = v6.clone().cw(90);
    let images = vec![v0, v1, v2, v3, v4, v5, v6, v7];

    for image in images {
        let mut monster_count: usize = 0;
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
                    monster_count += 1;
                }
            }
        }

        if monster_count > 0 {
            return image.row_major().filter(|b| **b).count()
                - monster_count * sea_monster.locations.len();
        }
    }
    panic!("No monsters found");
}
