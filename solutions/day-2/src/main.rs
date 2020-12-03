use std::str::FromStr;
use anyhow::Context;

const INPUT: &str = include_str!("../../../inputs/day-2.txt");

fn main() -> anyhow::Result<()> {
    let mut buffer = [0; 4];
    let mut valid_passwords = 0;

    for line in INPUT.lines() {
        let mut iter = line.splitn(4, |c| matches!(c, '-' | ' ' | ':'));
        let left = iter.next().map(usize::from_str).context("invalid low bound")?.context("missing low bound")?;
        let right = iter.next().map(usize::from_str).context("invalid high bound")?.context("missing high bound")?;
        let letter = iter.next().context("missing letter")?;
        let password = iter.next().context("missing password")?.trim();

        let count = password.chars().filter(|c| c.encode_utf8(&mut buffer) == letter).count();
        if count >= left && count <= right {
            valid_passwords += 1;
        }
    }

    println!("answer is {}", valid_passwords);

    Ok(())
}
