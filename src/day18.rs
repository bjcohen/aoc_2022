use anyhow::{anyhow, ensure, Error, Result};

use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn parse<R: Read>(reader: R) -> Result<Vec<(i64, i64, i64)>> {
    BufReader::new(reader)
        .lines()
        .map(|r| {
            r.map_err(Error::new).and_then(|l| {
                let ss = l.splitn(3, ',').collect::<Vec<&str>>();
                Ok((
                    ss[0].parse::<i64>()?,
                    ss[1].parse::<i64>()?,
                    ss[2].parse::<i64>()?,
                ))
            })
        })
        .collect()
}

fn main() -> Result<()> {
    let file = File::open("input18.txt")?;

    let cubes = parse(file)?;

    let surface_area = part1(&cubes)?;
    println!("{}", surface_area);

    let outside_area = part2(&cubes)?;
    println!("{}", outside_area);

    Ok(())
}

fn part1(cubes: &[(i64, i64, i64)]) -> Result<i64> {
    let set: HashSet<&(i64, i64, i64)> = cubes.iter().collect();

    let mut total_area = 0;
    for (x, y, z) in cubes {
        for (dx, dy, dz) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            if !set.contains(&(x + dx, y + dy, z + dz)) {
                total_area += 1;
            }
        }
    }
    Ok(total_area)
}

fn part2(cubes: &[(i64, i64, i64)]) -> Result<i64> {
    let cubes_set: HashSet<&(i64, i64, i64)> = cubes.iter().collect();
    let maxx = cubes
        .iter()
        .map(|c| c.0)
        .max()
        .ok_or_else(|| anyhow!("no max x"))?;
    let maxy = cubes
        .iter()
        .map(|c| c.1)
        .max()
        .ok_or_else(|| anyhow!("no max y"))?;
    let maxz = cubes
        .iter()
        .map(|c| c.2)
        .max()
        .ok_or_else(|| anyhow!("no max z"))?;
    let mut queue: VecDeque<(i64, i64, i64)> = VecDeque::from(vec![(0, 0, 0)]);
    let mut seen: HashSet<(i64, i64, i64)> = HashSet::new();
    while let Some((x, y, z)) = queue.pop_front() {
        if !seen.insert((x, y, z)) {
            continue;
        }
        for (dx, dy, dz) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            if x + dx >= 0
                && x + dx <= maxx
                && y + dy >= 0
                && y + dy <= maxy
                && z + dz >= 0
                && z + dz <= maxz
                && !cubes_set.contains(&(x + dx, y + dy, z + dz))
            {
                queue.push_back((x + dx, y + dy, z + dz));
            }
        }
    }
    let mut outside_area = 0;
    for (x, y, z) in cubes {
        for (dx, dy, dz) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            if seen.contains(&(x + dx, y + dy, z + dz)) {
                ensure!(
                    !cubes_set.contains(&(x + dx, y + dy, z + dz)),
                    "cubes and seen should be disjoint"
                );
                outside_area += 1;
            }
            if x + dx > maxx || x + dx < 0 {
                outside_area += 1;
            }
            if y + dy > maxy || y + dy < 0 {
                outside_area += 1;
            }
            if z + dz > maxz || z + dz < 0 {
                outside_area += 1;
            }
        }
    }
    Ok(outside_area)
}
