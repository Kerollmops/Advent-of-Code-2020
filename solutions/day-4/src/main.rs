use maplit::hashset;

const INPUT: &str = include_str!("../../../inputs/day-4.txt");

fn main() -> anyhow::Result<()> {
    let mut part_one_valid_passport = 0;

    for input in INPUT.split("\n\n") {
        let mut mandatory_fields = hashset!{ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" }; // "cid"
        for entry in input.split_whitespace() {
            if let Some(field_name) = entry.split(':').next() {
                mandatory_fields.remove(field_name);
            }
        }
        if mandatory_fields.is_empty() {
            part_one_valid_passport += 1;
        }
    }

    println!("part one answer is {}", part_one_valid_passport);

    let part_two_valid_passport = day_4_part_two(INPUT)?;
    println!("part two answer is {}", part_two_valid_passport);

    Ok(())
}

fn bounded_input(input: &str, low: u32, high: u32) -> bool {
    match input.parse::<u32>() {
        Ok(value) => value >= low && value <= high,
        Err(_e) => false,
    }
}

fn four_digit_bounded(input: &str, low: u32, high: u32) -> bool {
    if !(input.len() == 4 && input.chars().all(char::is_numeric)) {
        return false
    }
    bounded_input(input, low, high)
}

fn day_4_part_two(input: &str) -> anyhow::Result<usize> {
    let mut part_two_valid_passport = 0;

    for input in input.split("\n\n") {
        let mut mandatory_fields = hashset!{ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" }; // "cid"
        for entry in input.split_whitespace() {
            let mut parts = entry.splitn(2, ':');
            match parts.next().zip(parts.next()) {
                Some((name @ "byr", value)) if four_digit_bounded(value, 1920, 2002) => {
                    mandatory_fields.remove(name);
                },
                Some((name @ "iyr", value)) if four_digit_bounded(value, 2010, 2020) => {
                    mandatory_fields.remove(name);
                },
                Some((name @ "eyr", value)) if four_digit_bounded(value, 2020, 2030) => {
                    mandatory_fields.remove(name);
                },
                Some((name @ "hgt", value)) => {
                    let valid_cm = value.strip_suffix("cm").map_or(false, |v| bounded_input(v, 150, 193));
                    let valid_in = value.strip_suffix("in").map_or(false, |v| bounded_input(v, 59, 76));
                    if valid_cm || valid_in {
                        mandatory_fields.remove(name);
                    }
                },
                Some((name @ "hcl", value)) => {
                    let valid_hcl = value
                        .strip_prefix("#")
                        .map_or(false, |v| {
                            v.len() == 6 && v.chars().all(|c| matches!(c, 'a'..='f' | '0'..='9'))
                        });
                    if valid_hcl {
                        mandatory_fields.remove(name);
                    }
                },
                Some((name @ "ecl", value)) => {
                    if matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") {
                        mandatory_fields.remove(name);
                    }
                },
                Some((name @ "pid", value)) => {
                    if value.len() == 9 && value.chars().all(char::is_numeric) {
                        mandatory_fields.remove(name);
                    }
                },
                // Some((name @ "cid", value)) => (),
                _otherwise => (),
            }
        }
        if mandatory_fields.is_empty() {
            part_two_valid_passport += 1;
        }
    }

    Ok(part_two_valid_passport)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_valid_passports() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(day_4_part_two(input).unwrap(), 4);
    }

    #[test]
    fn part_two_invalid_passports() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        assert_eq!(day_4_part_two(input).unwrap(), 0);
    }
}
