use std::collections::{HashMap, HashSet};
use std::mem;

use regex::Regex;

const INPUT: &str = include_str!("../../../inputs/day-7.txt");
const NO_OTHER: &str = "no other";
const SHINY_GOLD: &str = "shiny gold";

fn main() {
    let part_one_answer = part_one(INPUT);
    println!("part one answer is: {}", part_one_answer);

    let part_two_answer = part_two(INPUT);
    println!("part two answer is: {}", part_two_answer);
}

fn part_one(input: &str) -> usize {
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

fn part_two(input: &str) -> usize {
    let regex = Regex::new(r#"(\d+\s)?(\w+\s\w+)\sbags?"#).unwrap();

    let mut bag_graph = HashMap::new();
    for line in input.lines() {
        let mut bags = regex.captures_iter(line);
        let wrapper_bag_color = bags.next().unwrap().get(2).unwrap().as_str();
        for wrapped_bag in bags {
            let bag_color = wrapped_bag.get(2).unwrap().as_str();
            if bag_color != NO_OTHER {
                let bag_count = wrapped_bag.get(1).unwrap().as_str().trim().parse::<usize>().unwrap();
                bag_graph
                    .entry(wrapper_bag_color)
                    .or_insert_with(HashMap::new)
                    .insert(bag_color, bag_count);
            }
        }
    }

    fn recurse(bags: &HashMap<&str, HashMap<&str, usize>>, bag: &str) -> usize {
        bags.get(bag).map_or(0, |bs| bs.iter().map(|(b, c)| c + c * recurse(bags, b)).sum())
    }

    recurse(&bag_graph, SHINY_GOLD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_part_one() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(part_one(input), 4);
    }

    #[test]
    fn first_part_two() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(part_two(input), 32);
    }

    #[test]
    fn second_part_two() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(part_two(input), 126);
    }
}
