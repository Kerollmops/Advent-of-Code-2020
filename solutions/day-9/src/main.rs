use itertools::Itertools;

const INPUT: &str = include_str!("../../../inputs/day-9.txt");

fn main() {
    println!("part one answer is: {}", part_one(INPUT, 25));
    println!("part two answer is: {}", part_two(INPUT, 25));
}

fn parse_numbers(input: &str) -> Vec<i64> {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn part_one(input: &str, preamble: usize) -> i64 {
    let numbers = parse_numbers(input);
    part_one_inner(&numbers, preamble)
}

fn part_one_inner(numbers: &[i64], preamble: usize) -> i64 {
    *numbers.windows(preamble + 1).find(|window| {
        let (num, head) = window.split_last().unwrap();
        !head.iter().cartesian_product(head.iter()).any(|(a, b)| a != b && a + b == *num)
    }).unwrap().last().unwrap()
}

fn part_two(input: &str, preamble: usize) -> i64 {
    let numbers = parse_numbers(input);
    let number = part_one_inner(&numbers, preamble);
    for start in 0..numbers.len() {
        let mut count = 0;
        for (i, x) in numbers[start..].iter().enumerate() {
            count += x;
            if count == number {
                let (min, max) = numbers[start..start + i].iter().minmax().into_option().unwrap();
                return min + max;
            } else if count > number {
                break
            }
        }
    }

    panic!("count not find a range for this number")
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

    #[test]
    fn simple_part_two() {
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

        assert_eq!(part_two(input, 5), 62);
    }
}
