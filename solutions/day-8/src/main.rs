const INPUT: &str = include_str!("../../../inputs/day-8.txt");
const ACC_INSTR: &str = "acc";
const JMP_INSTR: &str = "jmp";
const NOP_INSTR: &str = "nop";

fn main() {
    println!("part on answer is: {}", part_one(INPUT));
}

fn part_one(input: &str) -> i64 {
    let mut instrs: Vec<_> = input.lines().map(|l| {
        let (name, number) = l.split_at(3);
        let number = number.trim().parse::<i64>().unwrap();
        (name, number, false)
    }).collect();

    let mut acc = 0;
    let mut index = 0i64;
    while !instrs[index as usize].2 {
        instrs[index as usize].2 = true;
        match instrs[index as usize] {
            (ACC_INSTR, num, _) => { acc += num; index += 1; },
            (JMP_INSTR, num, _) => index += num,
            (NOP_INSTR, _, _) => index += 1,
            (name, _, _) => panic!("Unexpected instruction {}", name),
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_part_one() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        assert_eq!(part_one(&input), 5);
    }
}
