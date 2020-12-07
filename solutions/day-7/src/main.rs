use std::collections::{HashMap, HashSet};
use std::mem;

use regex::Regex;

const INPUT: &str = include_str!("../../../inputs/day-7.txt");
const NO_OTHER: &str = "no other";
const SHINY_GOLD: &str = "shiny gold";

fn main() {
    let part_one_answer = transitive_num_bags(INPUT);
    println!("part one answer is: {}", part_one_answer);
}

fn transitive_num_bags(input: &str) -> usize {
    let regex = Regex::new(r#"(\w+\s\w+)\sbags?"#).unwrap();

    let mut bag_graph = HashMap::new();
    for line in input.lines() {
        let mut bags = regex.captures_iter(line);
        let wrapping_bag = bags.next().unwrap().get(1).unwrap().as_str();
        for wrapped_bag in bags {
            let wrapped_bag = wrapped_bag.get(1).unwrap().as_str();
            if wrapped_bag != NO_OTHER {
                bag_graph.entry(wrapped_bag).or_insert_with(HashSet::new).insert(wrapping_bag);
            }
        }
    }

    let mut bags = HashSet::new();
    let mut tmp = Vec::new();
    tmp.push(SHINY_GOLD);
    while !tmp.is_empty() {
        for bag in mem::take(&mut tmp).into_iter().flat_map(|b| bag_graph.get(b)).flatten() {
            bags.insert(bag);
            tmp.push(bag);
        }
    }

    bags.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(transitive_num_bags(input), 4);
    }
}
