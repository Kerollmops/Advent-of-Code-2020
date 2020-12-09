use itertools::Itertools;

const INPUT: &str = include_str!("../../../inputs/day-9.txt");

fn main() {
    println!("part one answer is: {}", part_one(INPUT, 25));
}

fn part_one(input: &str, preamble: usize) -> i64 {
    let numbers: Vec<_> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let win = numbers.windows(preamble + 1).find(|window| {
        let (num, head) = window.split_last().unwrap();
        !head.iter().cartesian_product(head.iter()).any(|(a, b)| a != b && a + b == *num)
    });

    *win.unwrap().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_part_one() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        assert_eq!(part_one(input, 5), 127);
    }
}
