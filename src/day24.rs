use anyhow::{bail, Error, Result};
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader, Read},
};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Dir {
    L,
    R,
    U,
    D,
}

struct Limits {
    maxx: i64,
    maxy: i64,
}

#[derive(Hash, PartialEq, Eq, Clone, Ord, PartialOrd, Debug)]
struct State {
    x: i64,
    y: i64,
    time: usize,
}

fn parse<R: Read>(reader: R) -> Result<Vec<Vec<Option<Dir>>>> {
    BufReader::new(reader)
        .lines()
        .skip(1)
        .map(|r| {
            r.map_err(Error::new).and_then(|l| {
                l.chars()
                    .skip(1)
                    .take_while(|c| *c != '#')
                    .map(|c| {
                        Ok(match c {
                            '<' => Some(Dir::L),
                            '>' => Some(Dir::R),
                            '^' => Some(Dir::U),
                            'v' => Some(Dir::D),
                            '.' => None,
                            _ => bail!("unknown character: {}", c),
                        })
                    })
                    .collect()
            })
        })
        .take_while(|r: &Result<Vec<Option<Dir>>>| matches!(r, Ok(v) if !v.is_empty()))
        .collect()
}

fn main() -> Result<()> {
    let file = File::open("input24.txt")?;

    let board = parse(file)?;

    println!("{}", part1(&board)?);
    println!("{}", part2(&board)?);

    Ok(())
}

fn part1(board: &[Vec<Option<Dir>>]) -> Result<usize> {
    solve(
        board,
        State {
            x: 0,
            y: -1,
            time: 0,
        },
        i64::try_from(board[0].len())? - 1,
        i64::try_from(board.len())?,
    )
}

fn solve(board: &[Vec<Option<Dir>>], init: State, goalx: i64, goaly: i64) -> Result<usize> {
    let map: HashMap<(i64, i64), Dir> = board
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, dir)| dir.map(|d| (x, d)))
                .map(move |(x, d)| Ok(((i64::try_from(x)?, i64::try_from(y)?), d)))
        })
        .collect::<Result<HashMap<(i64, i64), Dir>>>()?;

    let limits = Limits {
        maxx: i64::try_from(board[0].len())?,
        maxy: i64::try_from(board.len())?,
    };

    let mut visited: HashSet<State> = HashSet::new();
    let mut queue: BinaryHeap<(i64, State)> = BinaryHeap::from([(0, init)]);

    while let Some((_weight, state)) = queue.pop() {
        if state.x == goalx && state.y == goaly {
            return Ok(state.time);
        }
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());
        let map_ = get_map(&map, state.time + 1, &limits)?;
        if state.x == 0 && state.y == -1 {
            if !map_.contains(&(0, 0)) {
                let weight = i64::abs(state.x - goalx)
                    + i64::abs(state.y + 1 - goaly)
                    + i64::try_from(state.time)?;
                queue.push((
                    -weight,
                    State {
                        x: state.x,
                        y: state.y + 1,
                        time: state.time + 1,
                    },
                ));
            }
            let weight =
                i64::abs(state.x - goalx) + i64::abs(state.y - goaly) + i64::try_from(state.time)?;
            queue.push((
                -weight,
                State {
                    x: state.x,
                    y: state.y,
                    time: state.time + 1,
                },
            ));
        } else if state.x == limits.maxx - 1 && state.y == limits.maxy {
            if !map_.contains(&(limits.maxx - 1, limits.maxy - 1)) {
                let weight = i64::abs(state.x - goalx)
                    + i64::abs(state.y - 1 - goaly)
                    + i64::try_from(state.time)?;
                queue.push((
                    -weight,
                    State {
                        x: state.x,
                        y: state.y - 1,
                        time: state.time + 1,
                    },
                ));
            }
            let weight =
                i64::abs(state.x - goalx) + i64::abs(state.y - goaly) + i64::try_from(state.time)?;
            queue.push((
                -weight,
                State {
                    x: state.x,
                    y: state.y,
                    time: state.time + 1,
                },
            ));
        } else {
            if state.x < limits.maxx - 1 && !map_.contains(&(state.x + 1, state.y)) {
                let weight = i64::abs(state.x + 1 - goalx)
                    + i64::abs(state.y - goaly)
                    + i64::try_from(state.time)?;
                queue.push((
                    -weight,
                    State {
                        x: state.x + 1,
                        y: state.y,
                        time: state.time + 1,
                    },
                ));
            }
            if (state.y < limits.maxy - 1 && !map_.contains(&(state.x, state.y + 1)))
                || (state.y == limits.maxy - 1 && state.x == limits.maxx - 1)
            {
                let weight = i64::abs(state.x - goalx)
                    + i64::abs(state.y + 1 - goaly)
                    + i64::try_from(state.time)?;
                queue.push((
                    -weight,
                    State {
                        x: state.x,
                        y: state.y + 1,
                        time: state.time + 1,
                    },
                ));
            }
            if !map_.contains(&(state.x, state.y)) {
                let weight = i64::abs(state.x - goalx)
                    + i64::abs(state.y - goaly)
                    + i64::try_from(state.time)?;
                queue.push((
                    -weight,
                    State {
                        x: state.x,
                        y: state.y,
                        time: state.time + 1,
                    },
                ));
            }
            if state.x > 0 && !map_.contains(&(state.x - 1, state.y)) {
                let weight = i64::abs(state.x - 1 - goalx)
                    + i64::abs(state.y - goaly)
                    + i64::try_from(state.time)?;
                queue.push((
                    -weight,
                    State {
                        x: state.x - 1,
                        y: state.y,
                        time: state.time + 1,
                    },
                ));
            }
            if (state.y > 0 && !map_.contains(&(state.x, state.y - 1)))
                || (state.x == 0 && state.y == 0)
            {
                let weight = i64::abs(state.x - goalx)
                    + i64::abs(state.y - 1 - goaly)
                    + i64::try_from(state.time)?;
                queue.push((
                    -weight,
                    State {
                        x: state.x,
                        y: state.y - 1,
                        time: state.time + 1,
                    },
                ));
            }
        }
    }
    bail!("no more candidates and didn't find the goal");
}

fn get_map(
    map: &HashMap<(i64, i64), Dir>,
    time: usize,
    limits: &Limits,
) -> Result<HashSet<(i64, i64)>> {
    map.iter()
        .map(|((x, y), dir)| {
            let d = i64::try_from(time)?;
            let c = match dir {
                Dir::L => ((x - d).rem_euclid(limits.maxx), *y),
                Dir::R => ((x + d) % limits.maxx, *y),
                Dir::U => (*x, (y - d).rem_euclid(limits.maxy)),
                Dir::D => (*x, (y + d) % limits.maxy),
            };
            Ok(c)
        })
        .collect()
}

fn part2(board: &[Vec<Option<Dir>>]) -> Result<usize> {
    let maxx = i64::try_from(board[0].len())?;
    let maxy = i64::try_from(board.len())?;
    let time = solve(
        board,
        State {
            x: 0,
            y: -1,
            time: 0,
        },
        maxx - 1,
        maxy,
    )?;
    let time = solve(
        board,
        State {
            x: maxx - 1,
            y: maxy,
            time,
        },
        0,
        -1,
    )?;
    solve(board, State { x: 0, y: -1, time }, maxx - 1, maxy)
}
