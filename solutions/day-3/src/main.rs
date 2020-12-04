const INPUT: &str = include_str!("../../../inputs/day-3.txt");
const INPUT_BYTES: &[u8] = INPUT.as_bytes();

fn count_trees(width: usize, right: usize, left: usize) -> usize {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;

    loop {
        x = (x + right) % width;
        y = y + left;

        // don't forget the newline
        let pos = (y * (width + 1)) + x;

        match INPUT_BYTES.get(pos) {
            Some(c) => if *c == b'#' { trees += 1 },
            None => break,
        }
    }

    trees
}

fn main() -> anyhow::Result<()> {
    let width = INPUT.lines().next().map_or(0, str::len);

    let trees_3_1 = count_trees(width, 3, 1);
    println!("part one answer is {}", trees_3_1);

    let trees_1_1 = count_trees(width, 1, 1);
    let trees_5_1 = count_trees(width, 5, 1);
    let trees_7_1 = count_trees(width, 7, 1);
    let trees_1_2 = count_trees(width, 1, 2);
    let answer = trees_1_1 * trees_3_1 * trees_5_1 * trees_7_1 * trees_1_2;
    println!("part two answer is {}", answer);

    Ok(())
}
