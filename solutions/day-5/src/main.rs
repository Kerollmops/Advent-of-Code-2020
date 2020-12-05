use std::str::Chars;

const INPUT: &str = include_str!("../../../inputs/day-5.txt");

fn main() -> anyhow::Result<()> {
    let mut seat_ids: Vec<_> = INPUT.lines().map(|line| {
        let (row, column) = seat_row_column(line);
        row * 8 + column
    }).collect();

    seat_ids.sort_unstable();

    let part_one_answer = seat_ids.last().unwrap();
    println!("part one answer is {}", part_one_answer);

    let part_two_answer = seat_ids.windows(2).find(|ws| ws[0] + 1 != ws[1]).unwrap()[0] + 1;
    println!("part two answer is {}", part_two_answer);

    Ok(())
}

fn seat_row_column(input: &str) -> (u32, u32) {
    let (row, column) = (&input[0..7], &input[7..7+3]);
    let row = binary_search(row.chars(), 0, 127);
    let column = binary_search(column.chars(), 0, 7);
    (row, column)
}

fn binary_search(moves: Chars, mut low: u32, mut high: u32) -> u32 {
    for goto in moves {
        match goto {
            'F' | 'L' => high = low + (high - low) / 2,
            'B' | 'R' => low = low + (high - low + 1) / 2,
            _ => panic!("Unexpected character {}", goto),
        }
    }
    assert_eq!(high, low);
    low
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
