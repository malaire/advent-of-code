use regex::Regex;
use std::collections::HashMap;

static INPUT_A: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 2208);
    assert_eq!(solve(INPUT_X), 4118);

    println!("{:?}", solve(INPUT_X));
}

fn solve(input: &str) -> usize {
    let re = Regex::new(r"se|sw|ne|nw|e|w").unwrap();

    let mut is_black: HashMap<(i32, i32), bool> = HashMap::new();

    for line in input.lines() {
        let mut row = 0;
        let mut col = 0;
        for cap in re.captures_iter(line) {
            match &cap[0] {
                "e" => col += 1,
                "w" => col -= 1,
                "ne" => row -= 1,
                "sw" => row += 1,
                "nw" => {
                    row -= 1;
                    col -= 1;
                }
                "se" => {
                    row += 1;
                    col += 1;
                }
                _ => panic!(""),
            }
        }
        is_black.insert((row, col), is_black.get(&(row, col)).map_or(true, |b| !*b));
    }

    for _ in 1..=100 {
        let mut row_min = 1000;
        let mut row_max = -1000;
        let mut col_min = 1000;
        let mut col_max = -1000;

        for (row, col) in is_black.keys().cloned() {
            if row < row_min {
                row_min = row
            }
            if row > row_max {
                row_max = row
            }
            if col < col_min {
                col_min = col
            }
            if col > col_max {
                col_max = col
            }
        }

        let mut is_black_new: HashMap<(i32, i32), bool> = HashMap::new();
        for row in (row_min - 1)..=(row_max + 1) {
            for col in (col_min - 1)..=(col_max + 1) {
                let mut adjacent_black = 0;
                adjacent_black += is_black.get(&(row, col + 1)).map_or(0, |b| *b as u32);
                adjacent_black += is_black.get(&(row, col - 1)).map_or(0, |b| *b as u32);
                adjacent_black += is_black.get(&(row + 1, col)).map_or(0, |b| *b as u32);
                adjacent_black += is_black.get(&(row - 1, col)).map_or(0, |b| *b as u32);
                adjacent_black += is_black.get(&(row + 1, col + 1)).map_or(0, |b| *b as u32);
                adjacent_black += is_black.get(&(row - 1, col - 1)).map_or(0, |b| *b as u32);

                let self_is_black = is_black.get(&(row, col)).map_or(false, |b| *b);

                if self_is_black {
                    if !(adjacent_black == 0 || adjacent_black > 2) {
                        is_black_new.insert((row, col), true);
                    }
                } else if adjacent_black == 2 {
                    is_black_new.insert((row, col), true);
                }
            }
        }
        is_black = is_black_new;
    }

    is_black.iter().filter(|(_, b)| **b).count()
}
