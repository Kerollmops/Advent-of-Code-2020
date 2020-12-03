use std::str::FromStr;

const INPUT: &str = include_str!("../../../inputs/day-1.txt");

fn main() -> anyhow::Result<()> {
    let mut expenses = INPUT.lines().map(u32::from_str).collect::<Result<Vec<_>, _>>()?;
    expenses.sort_unstable();

    let answer = (0..expenses.len()).find_map(|i| {
        expenses[i..].split_first().and_then(|(left, tail)| {
            tail.iter().find_map(|&right| {
                if left + right == 2020 { Some(left * right) } else { None }
            })
        })
    });

    match answer {
        Some(answer) => println!("answer is {}", answer),
        None => println!("no answer found"),
    }

    Ok(())
}
