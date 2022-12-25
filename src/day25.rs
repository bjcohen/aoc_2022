use anyhow::{bail, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn parse<R: Read>(reader: R) -> Result<Vec<String>> {
    Ok(BufReader::new(reader)
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?)
}

fn main() -> Result<()> {
    let file = File::open("input25.txt")?;

    let numbers = parse(file)?;

    println!("{}", part1(&numbers)?);
    println!("{}", part2(&numbers)?);

    Ok(())
}

fn part1(numbers: &[String]) -> Result<String> {
    let mut sum = 0i64;
    for number in numbers {
        let mut val = 0;
        for c in number.chars() {
            val *= 5;
            val += match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => bail!("unknown digit: {}", c),
            };
        }
        sum += val;
    }
    let mut digits: Vec<i64> = vec![];
    let mut i = 0;
    loop {
        if 5i64.pow(i) / 2 >= sum {
            break;
        }
        i += 1;
    }
    i -= 1;
    loop {
        let div = 5i64.pow(i);
        let sign = sum.signum();
        let d = (sign * sum + div / 2) / div;
        digits.push(sign * d);
        sum -= sign * d * div;
        if i > 0 {
            i -= 1;
        } else {
            break;
        }
    }
    digits
        .iter()
        .map(|d| {
            Ok(match d {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => bail!("uexpected c"),
            })
        })
        .collect::<Result<String>>()
}

fn part2(_numbers: &[String]) -> Result<usize> {
    bail!("unimplemented")
}
