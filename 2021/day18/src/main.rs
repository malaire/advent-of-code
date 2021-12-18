use malaire_aoc::run;

static INPUT_A: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 4140);
    run(1, solve_1, INPUT_X, 4132);

    run(0, solve_2, INPUT_A, 3993);
    run(2, solve_2, INPUT_X, 4685);
}

// (value, depth)
type RegularNumber = (usize, usize);
type Pair = Vec<RegularNumber>;

fn solve_1(input: &str) -> usize {
    let pairs: Vec<Pair> = input.lines().map(|p| parse_pair(p)).collect();
    magnitude(sum(pairs))
}

fn solve_2(input: &str) -> usize {
    let pairs: Vec<Pair> = input.lines().map(|p| parse_pair(p)).collect();

    let mut max_magnitude = 0;

    for a in 0..pairs.len() {
        for b in 0..pairs.len() {
            if a != b {
                max_magnitude = std::cmp::max(
                    max_magnitude,
                    magnitude(sum(vec![pairs[a].clone(), pairs[b].clone()])),
                )
            }
        }
    }

    max_magnitude
}

// ======================================================================
// UTIL

fn parse_pair(pair: &str) -> Pair {
    let mut result = Vec::new();
    let mut depth = 0;
    for byte in pair.as_bytes() {
        match byte {
            b'[' => depth += 1,
            b']' => depth -= 1,
            b'0'..=b'9' => result.push(((byte - b'0') as usize, depth)),
            b',' => {}
            _ => panic!(),
        }
    }
    result
}

fn sum(pairs: Vec<Pair>) -> Pair {
    let mut sum = pairs[0].clone();
    for pair in &pairs[1..] {
        // ADDITION

        let mut temp = Vec::new();
        for (value, depth) in &sum {
            temp.push((*value, depth + 1));
        }
        for (value, depth) in pair {
            temp.push((*value, depth + 1));
        }
        sum = temp;

        // REDUCTION

        'reduce: loop {
            // EXPLODE

            for pos in 0..sum.len() - 1 {
                if sum[pos].1 >= 5 && sum[pos].1 == sum[pos + 1].1 {
                    if pos > 0 {
                        sum[pos - 1].0 += sum[pos].0;
                    }

                    if pos < sum.len() - 2 {
                        sum[pos + 2].0 += sum[pos + 1].0;
                    }

                    sum.remove(pos);
                    sum[pos].0 = 0;
                    sum[pos].1 -= 1;

                    continue 'reduce;
                }
            }

            // SPLIT

            for pos in 0..sum.len() {
                if sum[pos].0 > 9 {
                    let value = sum[pos].0;
                    let depth = sum[pos].1;

                    sum[pos] = ((value + 1) / 2, depth + 1);
                    sum.insert(pos, (value / 2, depth + 1));

                    continue 'reduce;
                }
            }

            break;
        }
    }

    sum
}

fn magnitude(mut pair: Pair) -> usize {
    while pair.len() > 1 {
        for pos in 0..pair.len() - 1 {
            if pair[pos].1 == pair[pos + 1].1 {
                let left = pair[pos].0;
                let right = pair[pos + 1].0;

                pair.remove(pos);

                pair[pos].0 = 3 * left + 2 * right;
                pair[pos].1 -= 1;

                break;
            }
        }
    }
    pair[0].0
}
