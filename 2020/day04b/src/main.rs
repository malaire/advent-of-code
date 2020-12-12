use regex::Regex;
use std::collections::HashMap;

static INPUT_A: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

static INPUT_B: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

static INPUT_C: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

static INPUT_D: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 2);
    assert_eq!(solve(INPUT_B), 0);
    assert_eq!(solve(INPUT_C), 4);
    assert_eq!(solve(INPUT_D), 167);

    println!("{:?}", solve(INPUT_D));
}

fn solve(input: &str) -> usize {
    let mut valid = 0;

    let field_re = Regex::new(r"(\S+)?:(\S+)").unwrap();
    let four_digits_re = Regex::new(r"^(\d{4})$").unwrap();
    let hgt_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecl_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_re = Regex::new(r"^(\d{9})$").unwrap();

    for passport_str in input.split("\n\n") {
        let mut passport = HashMap::new();

        for field in field_re.captures_iter(passport_str) {
            let name = &field[1];
            let content = &field[2];

            let field_is_valid = match name {
                "byr" => {
                    let value = content.parse::<usize>().unwrap();
                    four_digits_re.is_match(content) && value >= 1920 && value <= 2002
                }
                "iyr" => {
                    let value = content.parse::<usize>().unwrap();
                    four_digits_re.is_match(content) && value >= 2010 && value <= 2020
                }
                "eyr" => {
                    let value = content.parse::<usize>().unwrap();
                    four_digits_re.is_match(content) && value >= 2020 && value <= 2030
                }
                "hgt" => {
                    if let Some(cap) = hgt_re.captures(content) {
                        let value = cap[1].parse::<usize>().unwrap();
                        match &cap[2] {
                            "cm" => value >= 150 && value <= 193,
                            "in" => value >= 59 && value <= 76,
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                "hcl" => hcl_re.is_match(content),
                "ecl" => ecl_re.is_match(content),
                "pid" => pid_re.is_match(content),
                "cid" => true,
                _ => false,
            };

            if field_is_valid {
                passport.insert(name.to_owned(), content.to_owned());
            }
        }

        let mut keys = passport.keys().cloned().collect::<Vec<String>>();
        keys.sort();
        let keys_str = keys.join(" ");

        if keys_str == "byr cid ecl eyr hcl hgt iyr pid"
            || keys_str == "byr ecl eyr hcl hgt iyr pid"
        {
            valid += 1;
        }
    }

    valid
}
