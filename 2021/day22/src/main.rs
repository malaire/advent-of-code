use std::collections::HashSet;

use regex::Regex;

use malaire_aoc::run;

static INPUT_A: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

static INPUT_B: &str = include_str!("input_b");
static INPUT_C: &str = include_str!("input_c");
static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 39);
    run(0, solve_1, INPUT_B, 590784);
    run(0, solve_1, INPUT_C, 474140);
    run(1, solve_1, INPUT_X, 652209);

    run(0, solve_2, INPUT_A, 39);
    run(0, solve_2, INPUT_B, 39769202357779);
    run(0, solve_2, INPUT_C, 2758514936282235);

    // This takes 17s on my computer.
    let start = std::time::Instant::now();
    run(2, solve_2, INPUT_X, 1217808640648260);
    println!("Elapsed {} s", start.elapsed().as_secs());
}

fn solve_1(input: &str) -> usize {
    let re = Regex::new(
        r"(?m)^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$",
    )
    .unwrap();

    let mut on = HashSet::new();

    for cap in re.captures_iter(input) {
        let state = &cap[1];

        let mut min_x: i64 = cap[2].parse().unwrap();
        let mut max_x: i64 = cap[3].parse().unwrap();
        let mut min_y: i64 = cap[4].parse().unwrap();
        let mut max_y: i64 = cap[5].parse().unwrap();
        let mut min_z: i64 = cap[6].parse().unwrap();
        let mut max_z: i64 = cap[7].parse().unwrap();

        min_x = std::cmp::max(min_x, -50);
        min_y = std::cmp::max(min_y, -50);
        min_z = std::cmp::max(min_z, -50);

        max_x = std::cmp::min(max_x, 50);
        max_y = std::cmp::min(max_y, 50);
        max_z = std::cmp::min(max_z, 50);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    if state == "on" {
                        on.insert((x, y, z));
                    } else {
                        on.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    on.len()
}

struct Step {
    turn_on: bool,
    min: (i64, i64, i64), // inclusive
    max: (i64, i64, i64), // exclusive
}

fn solve_2(input: &str) -> i64 {
    let re = Regex::new(
        r"(?m)^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$",
    )
    .unwrap();

    let mut area_min = (i64::MAX, i64::MAX, i64::MAX); // inclusive
    let mut area_max = (i64::MIN, i64::MIN, i64::MIN); // exclusive

    let mut steps = Vec::new();

    for cap in re.captures_iter(input) {
        let state = &cap[1];

        let min_x = cap[2].parse::<i64>().unwrap();
        let max_x = cap[3].parse::<i64>().unwrap() + 1;
        let min_y = cap[4].parse::<i64>().unwrap();
        let max_y = cap[5].parse::<i64>().unwrap() + 1;
        let min_z = cap[6].parse::<i64>().unwrap();
        let max_z = cap[7].parse::<i64>().unwrap() + 1;

        area_min.0 = std::cmp::min(area_min.0, min_x);
        area_min.1 = std::cmp::min(area_min.1, min_y);
        area_min.2 = std::cmp::min(area_min.2, min_z);

        area_max.0 = std::cmp::max(area_max.0, max_x);
        area_max.1 = std::cmp::max(area_max.1, max_y);
        area_max.2 = std::cmp::max(area_max.2, max_z);

        steps.push(Step {
            turn_on: state == "on",
            min: (min_x, min_y, min_z),
            max: (max_x, max_y, max_z),
        });
    }

    steps.reverse();

    count(area_min, area_max, &steps)
}

// min values inclusive, max values exclusive
fn count(min: (i64, i64, i64), max: (i64, i64, i64), steps: &[Step]) -> i64 {
    let mut split_x = [(false, 0i64); 3];
    let mut split_y = [(false, 0i64); 3];
    let mut split_z = [(false, 0i64); 3];

    let mut intersects_with_turn_on_step = false;
    for step in steps {
        if step.turn_on
            && (step.min.0 < max.0 && step.max.0 > min.0)
            && (step.min.1 < max.1 && step.max.1 > min.1)
            && (step.min.2 < max.2 && step.max.2 > min.2)
        {
            intersects_with_turn_on_step = true;
            break;
        }
    }

    if !intersects_with_turn_on_step {
        return 0;
    }

    for step_index in 0..steps.len() {
        let step = &steps[step_index];

        let n_x = split_area_1d(min.0, max.0, step.min.0, step.max.0, &mut split_x);
        let n_y = split_area_1d(min.1, max.1, step.min.1, step.max.1, &mut split_y);
        let n_z = split_area_1d(min.2, max.2, step.min.2, step.max.2, &mut split_z);

        if n_x + n_y + n_z == 3 {
            if split_x[0].0 && split_y[0].0 && split_z[0].0 {
                if step.turn_on {
                    return (max.0 - min.0) * (max.1 - min.1) * (max.2 - min.2);
                } else {
                    return 0;
                }
            } else {
                continue;
            }
        } else {
            let mut sum = 0;

            let mut min_x = min.0;
            for (in_x, max_x) in split_x[..n_x].iter().copied() {
                let mut min_y = min.1;
                for (in_y, max_y) in split_y[..n_y].iter().copied() {
                    let mut min_z = min.2;
                    for (in_z, max_z) in split_z[..n_z].iter().copied() {
                        if in_x && in_y && in_z {
                            if step.turn_on {
                                sum += (max_x - min_x) * (max_y - min_y) * (max_z - min_z);
                            }
                        } else {
                            sum += count(
                                (min_x, min_y, min_z),
                                (max_x, max_y, max_z),
                                &steps[step_index + 1..],
                            );
                        }
                        min_z = max_z;
                    }
                    min_y = max_y;
                }
                min_x = max_x;
            }
            return sum;
        }
    }

    return 0;
}

#[inline]
fn split_area_1d(
    area_min: i64,
    area_max: i64,
    block_min: i64,
    block_max: i64,
    // (split_is_in_block, max)
    splits: &mut [(bool, i64); 3],
) -> usize {
    if area_max <= block_min || block_max <= area_min {
        // NO INTERSECTION

        splits[0] = (false, area_max);
        return 1;
    } else if block_min <= area_min && area_max <= block_max {
        // AREA INSIDE OR EQUAL

        splits[0] = (true, area_max);
        return 1;
    } else if area_min < block_min && block_max < area_max {
        // BLOCK INSIDE

        splits[0] = (false, block_min);
        splits[1] = (true, block_max);
        splits[2] = (false, area_max);
        return 3;
    } else if block_min <= area_min && block_max < area_max {
        // OVERLAP AT AREA MIN

        splits[0] = (true, block_max);
        splits[1] = (false, area_max);
        return 2;
    } else if area_min < block_min && area_max <= block_max {
        // OVERLAP AT AREA MAX

        splits[0] = (false, block_min);
        splits[1] = (true, area_max);
        return 2;
    } else {
        panic!();
    }
}
