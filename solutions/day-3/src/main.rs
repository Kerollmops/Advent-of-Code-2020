const INPUT: &str = include_str!("../../../inputs/day-3.txt");

fn main() -> anyhow::Result<()> {
    let width = INPUT.lines().next().map_or(0, str::len);
    let input = INPUT.as_bytes();

    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;

    loop {
        x = (x + 3) % width;
        y = y + 1;

        // don't forget the newline
        let pos = (y * (width + 1)) + x;

        match input.get(pos) {
            Some(c) => if *c == b'#' { trees += 1 },
            None => break,
        }
    }

    println!("answer is {}", trees);

    Ok(())
}
