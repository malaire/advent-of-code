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
    assert_eq!(solve(INPUT_A), 10);
    assert_eq!(solve(INPUT_X), 488);

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

    is_black.iter().filter(|(_, b)| **b).count()
}
