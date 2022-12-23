use anyhow::{bail, Result};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn parse<R: Read>(reader: R) -> Result<Vec<String>, std::io::Error> {
    BufReader::new(reader).lines().collect()
}

fn main() -> Result<()> {
    let file = File::open("input23.txt")?;

    let board = parse(file)?;

    println!("{}", solve(&board, 10)?.1);
    println!("{}", solve(&board, 0)?.0);

    Ok(())
}

fn solve(board: &[String], nrounds: usize) -> Result<(usize, i64)> {
    let mut elves: HashSet<(i64, i64)> = HashSet::new();
    for (i, row) in board.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '#' {
                elves.insert((i64::try_from(j)?, i64::try_from(i)?));
            }
        }
    }
    let directions = ['N', 'S', 'W', 'E'];
    let mut r = 0;
    loop {
        if nrounds > 0 && r >= nrounds {
            break;
        }
        let mut proposals: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
        for (x, y) in &elves {
            let num_adjacent = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .iter()
            .filter(|(dx, dy)| elves.contains(&(x + dx, y + dy)))
            .count();
            if num_adjacent == 0 {
                continue;
            }
            for i in 0..4 {
                let dir = directions[(r + i) % 4];
                let (checks, (px, py)) = match dir {
                    'N' => ([(-1, -1), (0, -1), (1, -1)], (0, -1)),
                    'S' => ([(-1, 1), (0, 1), (1, 1)], (0, 1)),
                    'W' => ([(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
                    'E' => ([(1, -1), (1, 0), (1, 1)], (1, 0)),
                    _ => bail!("unexpected dir: {}", dir),
                };
                if checks
                    .iter()
                    .filter(|(dx, dy)| elves.contains(&(x + dx, y + dy)))
                    .count()
                    == 0
                {
                    proposals
                        .entry((x + px, y + py))
                        .and_modify(|v| v.push((*x, *y)))
                        .or_insert_with(|| vec![(*x, *y)]);
                    break;
                }
            }
        }
        let mut n_moved = 0;
        for (proposal, original_positions) in proposals {
            if original_positions.len() == 1 {
                n_moved += 1;
                elves.remove(&original_positions[0]);
                elves.insert(proposal);
            }
        }
        if n_moved == 0 {
            break;
        }
        r += 1;
    }
    let (mut minx, mut miny, mut maxx, mut maxy) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
    for (x, y) in &elves {
        minx = *x.min(&minx);
        miny = *y.min(&miny);
        maxx = *x.max(&maxx);
        maxy = *y.max(&maxy);
    }
    let empty_squares = (maxx - minx + 1) * (maxy - miny + 1) - i64::try_from(elves.len())?;
    Ok((r + 1, empty_squares))
}
