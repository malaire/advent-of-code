use regex::Regex;

static INPUT_A: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 2);
    assert_eq!(solve(INPUT_B), 607);

    println!("{:?}", solve(INPUT_B));
}

fn solve(input: &str) -> usize {
    // 1-3 b: cdefg
    let re = Regex::new(r"(?m)^(\d+)-(\d+) (.): (.+)$").unwrap();

    let mut valid_count: usize = 0;

    for x in re.captures_iter(input) {
        let min: usize = x[1].parse().unwrap();
        let max: usize = x[2].parse().unwrap();
        let needle: &str = &x[3];
        let haystack: &str = &x[4];

        let match_count: usize = haystack.matches(needle).count();

        if match_count >= min && match_count <= max {
            valid_count += 1;
        }
    }

    valid_count
}
