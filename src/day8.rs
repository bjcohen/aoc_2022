use anyhow::{anyhow, Error, Result};
use itertools::izip;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::repeat,
};

fn main() -> Result<()> {
    let file = File::open("input8.txt")?;
    let lines = BufReader::new(file).lines();

    let grid: Vec<Vec<i32>> = lines
        .map(|l| {
            l.map_err(Error::new).and_then(|l| {
                l.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .ok_or_else(|| anyhow!("couldn't map digit to int"))
                            .and_then(|d| i32::try_from(d).map_err(Error::new))
                    })
                    .collect::<Result<Vec<i32>>>()
            })
        })
        .collect::<Result<Vec<Vec<i32>>>>()?;

    let h = grid.len();
    let w = grid[0].len();

    let mut top_top = repeat(-1).take(w).collect::<Vec<i32>>();
    let mut viz_top = repeat(repeat(false).take(w).collect::<Vec<bool>>())
        .take(h)
        .collect::<Vec<Vec<bool>>>();
    let mut top_bot = repeat(-1).take(w).collect::<Vec<i32>>();
    let mut viz_bot = repeat(repeat(false).take(w).collect::<Vec<bool>>())
        .take(h)
        .collect::<Vec<Vec<bool>>>();

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] > top_top[j] {
                top_top[j] = grid[i][j];
                viz_top[i][j] = true;
            }
            if grid[h - i - 1][j] > top_bot[j] {
                top_bot[j] = grid[h - i - 1][j];
                viz_bot[h - i - 1][j] = true;
            }
        }
    }

    let mut top_l = repeat(-1).take(h).collect::<Vec<i32>>();
    let mut viz_l = repeat(repeat(false).take(h).collect::<Vec<bool>>())
        .take(w)
        .collect::<Vec<Vec<bool>>>();
    let mut top_r = repeat(-1).take(h).collect::<Vec<i32>>();
    let mut viz_r = repeat(repeat(false).take(h).collect::<Vec<bool>>())
        .take(w)
        .collect::<Vec<Vec<bool>>>();

    for i in 0..w {
        for j in 0..h {
            if grid[j][i] > top_l[j] {
                top_l[j] = grid[j][i];
                viz_l[j][i] = true;
            }
            if grid[j][w - i - 1] > top_r[j] {
                top_r[j] = grid[j][w - i - 1];
                viz_r[j][w - i - 1] = true;
            }
        }
    }

    let visible_trees: usize = izip!(viz_top, viz_bot, viz_l, viz_r)
        .map(|(ts, bs, ls, rs)| {
            izip!(ts, bs, ls, rs)
                .filter(|(t, b, l, r)| *t || *b || *l || *r)
                .count()
        })
        .sum();

    println!("{}", visible_trees);

    let mut score_l = repeat(repeat(0).take(h).collect::<Vec<i32>>())
        .take(w)
        .collect::<Vec<Vec<i32>>>();
    let mut score_r = repeat(repeat(0).take(h).collect::<Vec<i32>>())
        .take(w)
        .collect::<Vec<Vec<i32>>>();

    for i in 0..h {
        let mut lstack: Vec<(i32, usize)> = vec![];
        let mut rstack: Vec<(i32, usize)> = vec![];
        for j in 0..w {
            let vl = grid[i][j];
            while !lstack.is_empty()
                && vl > lstack.last().ok_or_else(|| anyhow!("stack was empty"))?.0
            {
                lstack.pop();
            }
            if lstack.is_empty() {
                score_l[i][j] = i32::try_from(j)?;
            } else {
                score_l[i][j] =
                    i32::try_from(j - lstack.last().ok_or_else(|| anyhow!("stack was empty"))?.1)?;
            }
            lstack.push((vl, j));
            let vr = grid[i][w - j - 1];
            while !rstack.is_empty()
                && vr > rstack.last().ok_or_else(|| anyhow!("stack was empty"))?.0
            {
                rstack.pop();
            }
            if rstack.is_empty() {
                score_r[i][w - j - 1] = i32::try_from(j)?;
            } else {
                score_r[i][w - j - 1] = i32::try_from(
                    rstack.last().ok_or_else(|| anyhow!("stack was empty"))?.1 - (w - j - 1),
                )?;
            }
            rstack.push((vr, w - j - 1));
        }
    }

    let mut score_u = repeat(repeat(0).take(h).collect::<Vec<i32>>())
        .take(w)
        .collect::<Vec<Vec<i32>>>();
    let mut score_d = repeat(repeat(0).take(h).collect::<Vec<i32>>())
        .take(w)
        .collect::<Vec<Vec<i32>>>();

    for i in 0..w {
        let mut ustack: Vec<(i32, usize)> = vec![];
        let mut dstack: Vec<(i32, usize)> = vec![];
        for j in 0..h {
            let vu = grid[j][i];
            while !ustack.is_empty()
                && vu > ustack.last().ok_or_else(|| anyhow!("stack was empty"))?.0
            {
                ustack.pop();
            }
            if ustack.is_empty() {
                score_u[j][i] = i32::try_from(j)?;
            } else {
                score_u[j][i] =
                    i32::try_from(j - ustack.last().ok_or_else(|| anyhow!("stack was empty"))?.1)?;
            }
            ustack.push((vu, j));
            let vd = grid[h - j - 1][i];
            while !dstack.is_empty()
                && vd > dstack.last().ok_or_else(|| anyhow!("stack was empty"))?.0
            {
                dstack.pop();
            }
            if dstack.is_empty() {
                score_d[h - j - 1][i] = i32::try_from(j)?;
            } else {
                score_d[h - j - 1][i] = i32::try_from(
                    dstack.last().ok_or_else(|| anyhow!("stack was empty"))?.1 - (h - j - 1),
                )?;
            }
            dstack.push((vd, h - j - 1));
        }
    }

    let max_score = izip!(score_l, score_r, score_u, score_d)
        .map(|(ls, rs, us, ds)| {
            izip!(ls, rs, us, ds)
                .map(|(l, r, u, d)| l * r * u * d)
                .max()
                .expect("")
        })
        .max()
        .ok_or_else(|| anyhow!("no max"))?;

    println!("{}", max_score);

    Ok(())
}
