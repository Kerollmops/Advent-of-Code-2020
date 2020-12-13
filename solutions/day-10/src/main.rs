use std::collections::BTreeSet;

const INPUT: &str = include_str!("../../../inputs/day-10.txt");

fn main() {
    println!("part one answer is: {}", part_one(INPUT));
}

fn extract_numbers(input: &str) -> BTreeSet<u64> {
    input.split_whitespace().map(|l| l.parse().unwrap()).collect()
}

fn part_one(input: &str) -> u64 {
    let mut adapters_set = extract_numbers(input);

    /// Returns the count of 1-jolt and 3-jolts differences or None
    /// if no adapters fits and the adapters set isn't empty.
    fn assemble_adapters(previous: u64, adapters: &mut BTreeSet<u64>) -> Option<(u64, u64)> {
        // don't forget the device built-in adapter.
        if adapters.is_empty() { return Some((0, 1)) }

        for adapter in previous + 1..=previous + 3 {
            if adapters.remove(&adapter) {
                match assemble_adapters(adapter, adapters) {
                    Some((mut ones, mut threes)) => {
                        if adapter == previous + 1 { ones += 1 }
                        if adapter == previous + 3 { threes += 1 }
                        return Some((ones, threes));
                    },
                    None => {
                        adapters.insert(adapter);
                    },
                }
            }
        }

        None
    }

    let (ones, threes) = assemble_adapters(0, &mut adapters_set).unwrap();
    ones * threes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_part_one() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";

        assert_eq!(part_one(input), 7*5);
    }

    #[test]
    fn simple_part_one_2() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        assert_eq!(part_one(input), 22*10);
    }
}
