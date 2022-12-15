use anyhow::{anyhow, bail, Error, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse() -> Result<Vec<Vec<(usize, usize)>>> {
    let file = File::open("input14.txt")?;
    BufReader::new(file)
        .lines()
        .map(|l| {
            l.map_err(Error::new).and_then(|l| {
                let tokens = l.split(' ');
                let mut v = vec![];
                for (i, token) in tokens.enumerate() {
                    if i % 2 == 0 {
                        let str_pair = token
                            .split_once(',')
                            .ok_or_else(|| anyhow!("couldn't split pair token"))?;

                        v.push((str_pair.0.parse::<usize>()?, str_pair.1.parse::<usize>()?));
                    }
                }
                Ok(v)
            })
        })
        .collect::<Result<Vec<Vec<(usize, usize)>>>>()
}

fn main() -> Result<()> {
    let paths = parse()?;

    let xmax = paths
        .iter()
        .map(|path| path.iter().map(|coord| coord.0).max())
        .max()
        .ok_or_else(|| anyhow!("no outer max"))?
        .ok_or_else(|| anyhow!("no inner max"))?;

    let xmin = paths
        .iter()
        .map(|path| path.iter().map(|coord| coord.0).min())
        .min()
        .ok_or_else(|| anyhow!("no outer min"))?
        .ok_or_else(|| anyhow!("no inner min"))?;

    let ymax = paths
        .iter()
        .map(|path| path.iter().map(|coord| coord.1).max())
        .max()
        .ok_or_else(|| anyhow!("no outer max"))?
        .ok_or_else(|| anyhow!("no inner max"))?;

    let mut grid = (0..ymax + 1)
        .map(|_| (xmin..xmax + 1).map(|_| '.').collect())
        .collect::<Vec<Vec<char>>>();

    for path in &paths {
        let mut windows = path.windows(2);
        while let Some([(x1, y1), (x2, y2)]) = windows.next() {
            if x1 == x2 {
                for row in grid
                    .iter_mut()
                    .take(usize::max(*y1, *y2) + 1)
                    .skip(usize::min(*y1, *y2))
                {
                    row[*x1 - xmin] = '#';
                }
            } else if y1 == y2 {
                for cell in grid[*y1]
                    .iter_mut()
                    .take(usize::max(*x1, *x2) + 1 - xmin)
                    .skip(usize::min(*x1, *x2) - xmin)
                {
                    *cell = '#';
                }
            } else {
                bail!("xs or ys must be equal")
            }
        }
    }

    let mut n_grains = None;

    'outer: for i in 0.. {
        let mut x = 500;
        let mut y = 0;
        loop {
            if y == ymax {
                n_grains = Some(i);
                break 'outer;
            } else if grid[y + 1][x - xmin] == '.' {
                y += 1;
            } else if x == xmin {
                n_grains = Some(i);
                break 'outer;
            } else if grid[y + 1][x - 1 - xmin] == '.' {
                x -= 1;
                y += 1;
            } else if x == xmax {
                n_grains = Some(i);
                break 'outer;
            } else if grid[y + 1][x + 1 - xmin] == '.' {
                x += 1;
                y += 1;
            } else {
                grid[y][x - xmin] = 'o';
                break;
            }
        }
    }

    println!("{}", n_grains.ok_or_else(|| anyhow!("n_grains not set"))?);

    let ymax = ymax + 2;
    let xmin = 500 - ymax;
    let xmax = 500 + ymax;

    let mut grid = (0..ymax + 1)
        .map(|_| (xmin..xmax + 1).map(|_| '.').collect())
        .collect::<Vec<Vec<char>>>();

    for path in paths {
        let mut windows = path.windows(2);
        while let Some([(x1, y1), (x2, y2)]) = windows.next() {
            if x1 == x2 {
                for row in grid
                    .iter_mut()
                    .take(usize::max(*y1, *y2) + 1)
                    .skip(usize::min(*y1, *y2))
                {
                    row[*x1 - xmin] = '#';
                }
            } else if y1 == y2 {
                for cell in grid[*y1]
                    .iter_mut()
                    .take(usize::max(*x1, *x2) + 1 - xmin)
                    .skip(usize::min(*x1, *x2) - xmin)
                {
                    *cell = '#';
                }
            } else {
                bail!("xs or ys must be equal")
            }
        }
    }

    for i in xmin..xmax + 1 {
        grid[ymax][i - xmin] = '#';
    }

    let mut n_grains = None;

    'outer: for i in 0.. {
        let mut x = 500;
        let mut y = 0;
        loop {
            if grid[y][x - xmin] == 'o' {
                n_grains = Some(i);
                break 'outer;
            }
            if grid[y + 1][x - xmin] == '.' {
                y += 1;
            } else if grid[y + 1][x - 1 - xmin] == '.' {
                x -= 1;
                y += 1;
            } else if grid[y + 1][x + 1 - xmin] == '.' {
                x += 1;
                y += 1;
            } else {
                grid[y][x - xmin] = 'o';
                break;
            }
        }
    }

    println!("{}", n_grains.ok_or_else(|| anyhow!("n_grains not set"))?);

    Ok(())
}
