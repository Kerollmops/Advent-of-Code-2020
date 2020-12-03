use std::str::FromStr;

const INPUT: &str = include_str!("../../../inputs/day-1.txt");

fn main() -> anyhow::Result<()> {
    let mut expenses = INPUT.lines().map(u32::from_str).collect::<Result<Vec<_>, _>>()?;
    expenses.sort_unstable();

    let part_one_answer = (0..expenses.len()).find_map(|i| {
        expenses[i..].split_first().and_then(|(left, tail)| {
            tail.iter().find_map(|&right| {
                if left + right == 2020 { Some(left * right) } else { None }
            })
        })
    });

    match part_one_answer {
        Some(answer) => println!("part one answer is {}", answer),
        None => println!("no part one answer found"),
    }

    let part_two_answer = (0..expenses.len()).find_map(|i| {
        expenses[i..].split_first().and_then(|(first, tail)| {
            (0..tail.len()).find_map(|i| {
                tail[i..].split_first().and_then(|(second, tail)| {
                    tail.iter().find_map(|&third| if first + second + third == 2020 {
                        Some(first * second * third)
                    } else {
                        None
                    })
                })
            })
        })
    });

    match part_two_answer {
        Some(answer) => println!("part two answer is {}", answer),
        None => println!("no part two answer found"),
    }

    Ok(())
}
