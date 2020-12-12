static INPUT_A: &str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

static INPUT_B: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 6);
    assert_eq!(solve(INPUT_B), 3143);

    println!("{:?}", solve(INPUT_B));
}

fn solve(input: &str) -> usize {
    let mut sum = 0;
    for group in input.split("\n\n") {
        let mut yes_counts = vec![0usize; 26];
        let mut people_count = 0;

        for line in group.lines() {
            people_count += 1;
            for chr in line.chars() {
                if chr >= 'a' && chr <= 'z' {
                    yes_counts[chr as usize - ('a' as usize)] += 1;
                }
            }
        }

        let mut all_yes_count = 0;
        for yes_count in yes_counts {
            if yes_count == people_count {
                all_yes_count += 1;
            }
        }

        sum += all_yes_count;
    }
    sum
}
