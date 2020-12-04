use maplit::hashset;

const INPUT: &str = include_str!("../../../inputs/day-4.txt");

fn main() -> anyhow::Result<()> {
    let mut valid_passport = 0;

    for input in INPUT.split("\n\n") {
        let mut mandatory_fields = hashset!{ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" }; // "cid"
        for entry in input.split_whitespace() {
            if let Some(field_name) = entry.split(':').next() {
                mandatory_fields.remove(field_name);
            }
        }
        if mandatory_fields.is_empty() {
            valid_passport += 1;
        }
    }

    println!("answer is {}", valid_passport);

    Ok(())
}
