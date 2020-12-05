const INPUT: &str = include_str!("../../../inputs/day-5.txt");

fn main() -> anyhow::Result<()> {
    let mut seat_ids: Vec<_> = INPUT.lines().map(|line| {
        let (row, column) = seat_row_column(line);
        row * 8 + column
    }).collect();

    seat_ids.sort_unstable();

    let part_one_answer = seat_ids.last().unwrap();
    println!("part one anwser is {}", part_one_answer);

    let part_two_answer = seat_ids.windows(2).find(|ws| ws[0] + 1 != ws[1]).unwrap()[0] + 1;
    println!("part two anwser is {}", part_two_answer);

    Ok(())
}

fn seat_row_column(input: &str) -> (u32, u32) {
    let (row, column) = (&input[0..7], &input[7..7+3]);
    let row = binary_search(row.chars().map(Goto::from), 0, 127);
    let column = binary_search(column.chars().map(Goto::from), 0, 7);
    (row, column)
}

fn binary_search(moves: impl Iterator<Item=Goto>, mut low: u32, mut high: u32) -> u32 {
    for goto in moves {
        match goto {
            Goto::Low => high = low + (high - low) / 2,
            Goto::High => low = low + (high - low + 1) / 2,
        }
    }
    assert_eq!(high, low);
    low
}

#[derive(Debug)]
enum Goto { Low, High }

impl From<char> for Goto {
    fn from(value: char) -> Goto {
        match value {
            'F' | 'L' => Goto::Low,
            'B' | 'R' => Goto::High,
            _otherwise => panic!("Unexpected character {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seats() {
        let first  = ("FBFBBFFRLR", 44,  5, 357);
        let second = ("BFFFBBFRRR", 70,  7, 567);
        let third  = ("FFFBBBFRRR", 14,  7, 119);
        let fourth = ("BBFFBBFRLL", 102, 4, 820);

        for (input, row, column, seat_id) in vec![first, second, third, fourth] {
            let (r, c) = seat_row_column(input);
            let id = r * 8 + c;

            assert_eq!(r, row);
            assert_eq!(c, column);
            assert_eq!(id, seat_id);
        }
    }
}
