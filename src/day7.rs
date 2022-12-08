use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("input7.txt")?;
    let mut lines = BufReader::new(file).lines();

    let mut stack = vec![];
    let mut sizes: HashMap<String, u32> = HashMap::new();

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("$ cd /") {
            stack.clear();
        } else if line.starts_with("$ cd ..") {
            stack.pop();
        } else if line.starts_with("$ cd") {
            let (_, dir) = line.split_at(5);
            stack.push(dir.to_string());
        } else if !line.starts_with("$ ls") && !line.starts_with("dir") {
            let (size_str, filename) = line
                .split_once(' ')
                .ok_or_else(|| anyhow!("couldn't split"))?;
            let size = size_str.parse::<u32>()?;
            for i in 0..stack.len() + 1 {
                let mut k = stack.iter().take(i).join("/");
                k.push('/');
                sizes.entry(k).and_modify(|s| *s += size).or_insert(size);
            }
            let mut k = stack.iter().join("/");
            k.push('/');
            k.push_str(filename);
            sizes.entry(k).and_modify(|s| *s += size).or_insert(size);
        }
    }

    println!(
        "{}",
        sizes
            .iter()
            .filter(|(path, size)| path.ends_with('/') && **size <= 100000)
            .map(|(_path, size)| size)
            .sum::<u32>()
    );

    let space_needed = 30000000
        - (70000000
            - sizes
                .get("/")
                .ok_or_else(|| anyhow!("couldn't find root dir"))?);

    println!(
        "{}",
        sizes
            .iter()
            .filter(|(path, size)| path.ends_with('/') && **size > space_needed)
            .map(|(_path, size)| size)
            .min()
            .ok_or_else(|| anyhow!("couldn't find a min size"))?
    );

    Ok(())
}
