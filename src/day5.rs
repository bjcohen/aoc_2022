use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("input5.txt")?;
    let mut lines = BufReader::new(file).lines();

    let mut stacks = vec![];
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let n = (line.len() - 3) / 4 + 1;
        if stacks.is_empty() {
            (0..n).for_each(|_| stacks.push(vec![]));
        }
        for (i, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let c = chunk.nth(1).ok_or(anyhow!("couldn't unwrap char"))?;
            if c != ' ' && !c.is_digit(10) {
                stacks[i].push(c);
            }
        }
    }

    stacks.iter_mut().for_each(|s| s.reverse());

    while let Some(Ok(line)) = lines.next() {
        let s: Vec<&str> = line.split(' ').collect();
        let (n, src, dst) = (
            s[1].parse::<usize>()?,
            s[3].parse::<usize>()?,
            s[5].parse::<usize>()?,
        );
        let i = stacks[src - 1].len() - n;
        let mut to_move = stacks[src - 1].split_off(i);
        to_move.reverse();
        stacks[dst - 1].extend_from_slice(&mut to_move);
    }

    println!(
        "{}",
        stacks
            .into_iter()
            .map(|s| s.last().copied().ok_or(anyhow!("empty vec")))
            .collect::<Result<String>>()?
    );

    let file = File::open("input5.txt")?;
    let mut lines = BufReader::new(file).lines();

    let mut stacks = vec![];
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let n = (line.len() - 3) / 4 + 1;
        if stacks.is_empty() {
            (0..n).for_each(|_| stacks.push(vec![]));
        }
        for (i, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let c = chunk.nth(1).ok_or(anyhow!("couldn't unwrap char"))?;
            if c != ' ' && !c.is_digit(10) {
                stacks[i].push(c);
            }
        }
    }

    stacks.iter_mut().for_each(|s| s.reverse());

    while let Some(Ok(line)) = lines.next() {
        let s: Vec<&str> = line.split(' ').collect();
        let (n, src, dst) = (
            s[1].parse::<usize>()?,
            s[3].parse::<usize>()?,
            s[5].parse::<usize>()?,
        );
        let i = stacks[src - 1].len() - n;
        let mut to_move = stacks[src - 1].split_off(i);
        stacks[dst - 1].extend_from_slice(&mut to_move);
    }

    println!(
        "{}",
        stacks
            .into_iter()
            .map(|s| s.last().copied().ok_or(anyhow!("empty vec")))
            .collect::<Result<String>>()?
    );
    
    Ok(())
}
