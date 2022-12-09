use anyhow::{anyhow, Error, Result};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("input9.txt")?;
    let lines = BufReader::new(file)
        .lines()
        .map(|l| l.map_err(Error::new))
        .collect::<Result<Vec<String>>>()?;

    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut t_locs: HashSet<(i32, i32)> = HashSet::new();
    t_locs.insert(t);

    for line in &lines {
        let (op, n_str) = line
            .split_once(' ')
            .ok_or_else(|| anyhow!("couldn't split"))?;
        let n = n_str.parse::<i32>()?;
        let d = match op {
            "R" => Ok((1, 0)),
            "L" => Ok((-1, 0)),
            "U" => Ok((0, 1)),
            "D" => Ok((0, -1)),
            _ => Err(anyhow!("unmatched up")),
        }?;
        for _ in 0..n {
            let (hx, hy) = h;
            let (tx, ty) = t;
            let (dx, dy) = d;
            h = (hx + dx, hy + dy);
            if (hx + dx - tx).abs() > 1 || (hy + dy - ty).abs() > 1 {
                t = (hx, hy);
            }
            t_locs.insert(t);
        }
    }

    println!("{}", t_locs.len());

    let mut knots: Vec<(i32, i32)> = (0..10).map(|_| (0, 0)).collect();
    let mut t_locs: HashSet<(i32, i32)> = HashSet::new();
    t_locs.insert(knots[9]);

    for line in lines {
        let (op, n_str) = line
            .split_once(' ')
            .ok_or_else(|| anyhow!("couldn't split"))?;
        let n = n_str.parse::<i32>()?;
        let d = match op {
            "R" => Ok((1, 0)),
            "L" => Ok((-1, 0)),
            "U" => Ok((0, 1)),
            "D" => Ok((0, -1)),
            _ => Err(anyhow!("unmatched up")),
        }?;
        for _ in 0..n {
            let mut di = d;
            for i in 0..10 {
                let (dx, dy) = di;
                let (hx, hy) = knots[i];
                if i < 9 {
                    let (tx, ty) = knots[i + 1];
                    let c = (hx + dx - tx, hy + dy - ty);
                    let (dx_, dy_) = match c {
                        (2, 0) => (1, 0),
                        (2, 1) => (1, 1),
                        (2, 2) => (1, 1),
                        (2, -1) => (1, -1),
                        (2, -2) => (1, -1),
                        (-2, 0) => (-1, 0),
                        (-2, 1) => (-1, 1),
                        (-2, 2) => (-1, 1),
                        (-2, -1) => (-1, -1),
                        (-2, -2) => (-1, -1),
                        (0, 2) => (0, 1),
                        (1, 2) => (1, 1),
                        (-1, 2) => (-1, 1),
                        (0, -2) => (0, -1),
                        (1, -2) => (1, -1),
                        (-1, -2) => (-1, -1),
                        _ => (0, 0),
                    };
                    di = (dx_, dy_);
                }
                knots[i] = (hx + dx, hy + dy);
            }
            t_locs.insert(knots[9]);
        }
    }

    println!("{}", t_locs.len());

    Ok(())
}
