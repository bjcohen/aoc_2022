use anyhow::{anyhow, Result};
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("input12.txt")?;
    let grid = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;

    let start_pos = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.chars().position(|c| c == 'S').map(|j| (i, j)))
        .ok_or_else(|| anyhow!("couldn't find S"))?;

    let end_pos = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.chars().position(|c| c == 'E').map(|j| (i, j)))
        .ok_or_else(|| anyhow!("couldn't find E"))?;

    let heights = grid
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    'S' => 'a' as i32,
                    'E' => 'z' as i32,
                    _ => c as i32,
                })
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut dists = grid
        .iter()
        .map(|row| {
            (0..row.len())
                .map(|_| usize::max_value())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut queue: VecDeque<(usize, usize)> = VecDeque::from(vec![start_pos]);

    let height = grid.len();
    let width = grid[0].len();

    while let Some((r, c)) = queue.pop_front() {
        if (r, c) == start_pos {
            dists[r][c] = 0;
        } else if dists[r][c] != usize::max_value() {
            continue;
        }
        let mut coords_to_check: Vec<(usize, usize)> = vec![];
        if r > 0 {
            coords_to_check.push((r - 1, c));
        }
        if r < height - 1 {
            coords_to_check.push((r + 1, c));
        }
        if c > 0 {
            coords_to_check.push((r, c - 1));
        }
        if c < width - 1 {
            coords_to_check.push((r, c + 1));
        }
        let h = heights[r][c];
        for (r_, c_) in coords_to_check {
            let d_ = dists[r_][c_];
            if d_ == usize::max_value() {
                if heights[r_][c_] - h <= 1 {
                    queue.push_back((r_, c_));
                }
            } else if h - heights[r_][c_] <= 1 {
                let dcurr = dists[r][c];
                dists[r][c] = usize::min(dcurr, d_ + 1);
            }
        }
    }

    println!("{}", dists[end_pos.0][end_pos.1]);

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    let mut dists = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'a' || c == 'S' {
                        queue.push_back((i, j));
                        0
                    } else {
                        usize::max_value()
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    while let Some((r, c)) = queue.pop_front() {
        if dists[r][c] != usize::max_value() && dists[r][c] != 0 {
            continue;
        }
        let mut coords_to_check: Vec<(usize, usize)> = vec![];
        if r > 0 {
            coords_to_check.push((r - 1, c));
        }
        if r < height - 1 {
            coords_to_check.push((r + 1, c));
        }
        if c > 0 {
            coords_to_check.push((r, c - 1));
        }
        if c < width - 1 {
            coords_to_check.push((r, c + 1));
        }
        let h = heights[r][c];
        for (r_, c_) in coords_to_check {
            let d_ = dists[r_][c_];
            if d_ == usize::max_value() {
                if heights[r_][c_] - h <= 1 {
                    queue.push_back((r_, c_));
                }
            } else if h - heights[r_][c_] <= 1 {
                let dcurr = dists[r][c];
                dists[r][c] = usize::min(dcurr, d_ + 1);
            }
        }
    }

    println!("{}", dists[end_pos.0][end_pos.1]);

    Ok(())
}
