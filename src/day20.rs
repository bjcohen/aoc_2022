use anyhow::{anyhow, Error, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read}, cmp::Ordering,
};

fn parse<R: Read>(reader: R) -> Result<Vec<i64>> {
    BufReader::new(reader)
        .lines()
        .map(|r| {
            r.map_err(Error::new)
                .and_then(|l| l.parse().map_err(Error::new))
        })
        .collect()
}

fn main() -> Result<()> {
    let file = File::open("input20.txt")?;

    let numbers = parse(file)?;

    println!("{}", part1(&numbers)?);
    println!("{}", part2(&numbers)?);

    Ok(())
}

fn mix(numbers: &mut Vec<(i64, usize)>, n: usize) -> Result<()> {
    let len = numbers.len();
    let orig = numbers.clone();
    for _i in 0..n {
        for (number, i) in &orig {
            let curri = numbers
                .iter()
                .position(|(_n, j)| j == i)
                .ok_or_else(|| anyhow!("couldn't find original index {}", i))?;
            let newi: usize = (i64::try_from(curri)? + number)
                .checked_rem_euclid(i64::try_from(len)? - 1)
                .ok_or_else(|| anyhow!("error finding remainder"))?
                .try_into()?;
            let tmp = numbers[curri];
            match newi.cmp(&curri) {
                Ordering::Greater => numbers.copy_within(curri+1..newi+1, curri),
                Ordering::Less => numbers.copy_within(newi..curri, newi+1),
                Ordering::Equal => (),
            }
            numbers[newi] = tmp;
        }
    }
    Ok(())
}

fn part1(numbers: &[i64]) -> Result<i64> {
    let mut numbers: Vec<(i64, usize)> = numbers.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    mix(&mut numbers, 1)?;
    let izero = numbers
        .iter()
        .position(|(n, _)| *n == 0)
        .ok_or_else(|| anyhow!("couldn't find 0"))?;
    let len = numbers.len();
    Ok(numbers[(izero + 1000) % len].0
        + numbers[(izero + 2000) % len].0
        + numbers[(izero + 3000) % len].0)
}

fn part2(numbers: &[i64]) -> Result<i64> {
    let key = 811589153;
    let mut numbers: Vec<(i64, usize)> = numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| (n * key, i))
        .collect();
    mix(&mut numbers, 10)?;
    let izero = numbers
        .iter()
        .position(|(n, _)| *n == 0)
        .ok_or_else(|| anyhow!("couldn't find 0"))?;
    let len = numbers.len();
    Ok(numbers[(izero + 1000) % len].0
        + numbers[(izero + 2000) % len].0
        + numbers[(izero + 3000) % len].0)
}
