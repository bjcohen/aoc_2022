use anyhow::{anyhow, bail, Error, Result};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn parse<R: Read>(reader: R) -> Result<Vec<(i64, i64, i64)>> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")?;
    BufReader::new(reader)
        .lines()
        .map(|l| {
            l.map_err(Error::new).and_then(|l| {
                let matches = re
                    .captures(&l)
                    .ok_or_else(|| anyhow!("re no match: {}", l))?;
                let sx = matches[1].parse()?;
                let sy = matches[2].parse()?;
                let bx = matches[3].parse::<i64>()?;
                let by = matches[4].parse::<i64>()?;
                Ok((sx, sy, i64::abs(sx - bx) + i64::abs(sy - by)))
            })
        })
        .collect::<Result<Vec<(i64, i64, i64)>>>()
}

fn main() -> Result<()> {
    let file = File::open("input15.txt")?;
    let coords = parse(file)?;

    let sum_merged_lengths = get_ranges(2000000, &coords)?
        .iter()
        .map(|(x1, x2)| x2 - x1)
        .sum::<i64>();

    println!("{}", sum_merged_lengths);

    const B: i64 = 4000000;

    for y in 0..B + 1 {
        if y % 1000000 == 0 {
            println!("y={}", y);
        }
        let ranges = get_ranges(y, &coords)?;
        let filtered_ranges = ranges
            .iter()
            .filter_map(|(x1, x2)| {
                if *x1 <= B || *x2 >= 0 {
                    Some((*x1, *x2))
                } else {
                    None
                }
            })
            .collect::<Vec<(i64, i64)>>();
        if filtered_ranges.len() == 2 {
            if let [(_, x12), (x21, _)] = filtered_ranges.as_slice() {
                if x21 - x12 != 2 {
                    bail!("unexpected range values: {}, {}", x12, x21);
                }
                println!("{}", (x12 + 1) * B + y);
            } else {
                bail!("unexpected vec length {}", filtered_ranges.len());
            }
        }
    }

    Ok(())
}

fn get_ranges(y: i64, coords: &Vec<(i64, i64, i64)>) -> Result<Vec<(i64, i64)>> {
    let mut ranges: Vec<(i64, i64)> = vec![];

    for (sx, sy, db) in coords {
        let dy = i64::abs(sy - y);
        if *db >= dy {
            let dx = db - dy;
            ranges.push((sx - dx, sx + dx));
        }
    }

    ranges.sort();

    let mut iter = ranges.iter();
    let mut merged_ranges: Vec<(i64, i64)> = vec![];

    let (mut c1, mut c2) = iter.next().ok_or_else(|| anyhow!("no ranges"))?;

    for (n1, n2) in iter {
        if *n1 <= c2 {
            c2 = i64::max(c2, *n2);
        } else {
            merged_ranges.push((c1, c2));
            c1 = *n1;
            c2 = *n2;
        }
    }

    merged_ranges.push((c1, c2));

    Ok(merged_ranges)
}
