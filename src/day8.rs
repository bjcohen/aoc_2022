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

    let mut max_score = 0;

    for i in 0..w {
        for j in 0..h {
            let l = (0..i)
                .take_while(|k| grid[i - k - 1][j] < grid[i][j])
                .count();
            let r = (0..w - i - 1)
                .take_while(|k| grid[i + k + 1][j] < grid[i][j])
                .count();
            let u = (0..j)
                .take_while(|k| grid[i][j - k - 1] < grid[i][j])
                .count();
            let d = (0..h - j - 1)
                .take_while(|k| grid[i][j + k + 1] < grid[i][j])
                .count();
            let score = if l < i { l + 1 } else { l }
                * if r < w - i - 1 { r + 1 } else { r }
                * if u < j { u + 1 } else { u }
                * if d < h - j - 1 { d + 1 } else { d };
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("{}", max_score);

    Ok(())
}
