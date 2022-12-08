use anyhow::Result;
use itertools::Itertools;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let mut buf = read_to_string("input6.txt")?;
    buf.pop();

    let vec = buf.chars().collect::<Vec<char>>();
    for (i, win) in vec.windows(4).enumerate() {
        if win.into_iter().unique().count() == 4 {
            println!("{}", i + 4);
            break;
        }
    }
    for (i, win) in vec.windows(14).enumerate() {
        if win.into_iter().unique().count() == 14 {
            println!("{}", i + 14);
            break;
        }
    }

    Ok(())
}
