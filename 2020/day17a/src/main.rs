use std::collections::HashSet;

static INPUT_A: &str = ".#.
..#
###";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 112);
    assert_eq!(solve(INPUT_X), 273);

    println!("{:?}", solve(INPUT_X));
}

fn solve(input: &str) -> usize {
    // x,y,z
    let mut active: HashSet<(i16, i16, i16)> = HashSet::new();

    let mut x = 0;
    for line in input.split("\n") {
        let mut y = 0;
        for chr in line.chars() {
            if chr == '#' {
                active.insert((x, y, 0));
            }
            y += 1;
        }
        x += 1;
    }

    for _ in 1..=6 {
        let mut min_x = 30000;
        let mut max_x = -30000;
        let mut min_y = 30000;
        let mut max_y = -30000;
        let mut min_z = 30000;
        let mut max_z = -30000;
        for (x, y, z) in &active {
            if *x < min_x {
                min_x = *x
            }
            if *x > max_x {
                max_x = *x
            }

            if *y < min_y {
                min_y = *y
            }
            if *y > max_y {
                max_y = *y
            }

            if *z < min_z {
                min_z = *z
            }
            if *z > max_z {
                max_z = *z
            }
        }

        let mut new_active: HashSet<(i16, i16, i16)> = HashSet::new();
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    // NOTE: includes itself
                    let mut nearby_active = 0;
                    for xx in (x - 1)..=(x + 1) {
                        for yy in (y - 1)..=(y + 1) {
                            for zz in (z - 1)..=(z + 1) {
                                if active.contains(&(xx, yy, zz)) {
                                    nearby_active += 1;
                                }
                            }
                        }
                    }
                    if active.contains(&(x, y, z)) {
                        if nearby_active == 3 || nearby_active == 4 {
                            new_active.insert((x, y, z));
                        }
                    } else if nearby_active == 3 {
                        new_active.insert((x, y, z));
                    }
                }
            }
        }
        active = new_active;
    }

    active.len()
}
