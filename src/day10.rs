use anyhow::{Error, Result};
use std::{
    // collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("input10.txt")?;
    let lines = BufReader::new(file)
        .lines()
        .map(|l| l.map_err(Error::new))
        .collect::<Result<Vec<String>>>()?;

    let mut reg = 1;
    let mut cycle = 1;
    let cycles = vec![20, 60, 100, 140, 180, 220];
    let mut sum_strengths = 0;

    for line in &lines {
        if let Some((_, n_str)) = line.split_once(' ') {
            let n = n_str.parse::<i32>()?;
            cycle += 2;
            reg += n;
            if cycles.contains(&cycle) {
                sum_strengths += cycle * reg;
            } else if cycles.contains(&(cycle - 1)) {
                sum_strengths += (cycle - 1) * (reg - n);
            }
        } else {
            cycle += 1;
            if cycles.contains(&cycle) {
                sum_strengths += cycle * reg;
            }
        }
    }

    println!("{}", sum_strengths);

    reg = 1;
    cycle = 0;
    let mut buf = vec![];

    for line in &lines {
        if let Some((_, n_str)) = line.split_once(' ') {
            buf.push(if (reg - cycle % 40).abs() <= 1 {
                '#'
            } else {
                '.'
            });
            buf.push(if (reg - (cycle + 1) % 40).abs() <= 1 {
                '#'
            } else {
                '.'
            });
            let n = n_str.parse::<i32>()?;
            cycle += 2;
            reg += n;
        } else {
            buf.push(if (reg - cycle % 40).abs() <= 1 {
                '#'
            } else {
                '.'
            });
            cycle += 1;
        }
    }

    for i in 0..6 {
        println!("{}", buf[i * 40..i * 40 + 40].iter().collect::<String>());
    }

    Ok(())
}
