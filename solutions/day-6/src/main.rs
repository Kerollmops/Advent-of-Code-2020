use bitvec::bitarr;
use bitvec::prelude::*;

const INPUT: &str = include_str!("../../../inputs/day-6.txt");

fn main() -> anyhow::Result<()> {
    let part_one_answer = groups_count_true_answers(INPUT);
    println!("part one answer is {}", part_one_answer);

    Ok(())
}

fn answer_to_index(answer: char) -> usize {
    assert!(matches!(answer, 'a'..='z'), "invalid answer {}", answer);
    answer as usize - 'a' as usize
}

fn groups_count_true_answers(input: &str) -> usize {
    let mut count = 0;
    for group_lines in input.split("\n\n") {
        let mut answers_set = bitarr![0; 26];
        for answers in group_lines.lines() {
            for answer in answers.chars() {
                let index = answer_to_index(answer);
                answers_set.set(index, true);
            }
        }
        count += answers_set.count_ones();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(groups_count_true_answers(input), 11);
    }
}
