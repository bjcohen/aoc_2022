use anyhow::{anyhow, bail, ensure, Error, Result};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug)]
enum Op {
    Add(String, String),
    Const(i64),
    Div(String, String),
    Mul(String, String),
    Sub(String, String),
}

fn parse<R: Read>(reader: R) -> Result<HashMap<String, Op>> {
    let re1 = Regex::new(r"(\w+): (\-?\d+)")?;
    let re2 = Regex::new(r"(\w+): (\w+) ([\+\-\*/]) (\w+)")?;
    BufReader::new(reader)
        .lines()
        .map(|r| {
            r.map_err(Error::new).and_then(|l| {
                if let Some(cs) = re1.captures(&l) {
                    Ok((cs[1].to_string(), Op::Const(cs[2].parse()?)))
                } else if let Some(cs) = re2.captures(&l) {
                    Ok((
                        cs[1].to_string(),
                        match &cs[3] {
                            "+" => Op::Add(cs[2].to_string(), cs[4].to_string()),
                            "-" => Op::Sub(cs[2].to_string(), cs[4].to_string()),
                            "*" => Op::Mul(cs[2].to_string(), cs[4].to_string()),
                            "/" => Op::Div(cs[2].to_string(), cs[4].to_string()),
                            _ => bail!("unmatched operator: {}", &cs[3]),
                        },
                    ))
                } else {
                    bail!("couldn't match either regex: {}", l)
                }
            })
        })
        .collect()
}

fn main() -> Result<()> {
    let file = File::open("input21.txt")?;

    let monkeys = parse(file)?;

    println!("{}", part1(&monkeys)?);
    println!("{}", part2(&monkeys)?);

    Ok(())
}

fn solve1(monkeys: &HashMap<String, Op>, monkey: &str) -> Result<i64> {
    Ok(
        match monkeys
            .get(monkey)
            .ok_or_else(|| anyhow!("unknown monkey: {}", monkey))?
        {
            Op::Add(m1, m2) => solve1(monkeys, m1)? + solve1(monkeys, m2)?,
            Op::Sub(m1, m2) => solve1(monkeys, m1)? - solve1(monkeys, m2)?,
            Op::Mul(m1, m2) => solve1(monkeys, m1)? * solve1(monkeys, m2)?,
            Op::Div(m1, m2) => solve1(monkeys, m1)? / solve1(monkeys, m2)?,
            Op::Const(c) => *c,
        },
    )
}

fn part1(monkeys: &HashMap<String, Op>) -> Result<i64> {
    solve1(monkeys, "root")
}

fn find_humn<'a>(
    monkeys: &'a HashMap<String, Op>,
    monkey: &'a str,
) -> Result<Option<Vec<&'a str>>> {
    if monkey == "humn" {
        Ok(Some(vec!["humn"]))
    } else {
        let v = match monkeys
            .get(monkey)
            .ok_or_else(|| anyhow!("unknown monkey: {}", monkey))?
        {
            Op::Add(m1, m2) => match (find_humn(monkeys, m1)?, find_humn(monkeys, m2)?) {
                (None, None) => None,
                (Some(v), None) => Some(v),
                (None, Some(v)) => Some(v),
                (Some(_), Some(_)) => bail!("found two humans?"),
            },
            Op::Const(_) => None,
            Op::Div(m1, m2) => match (find_humn(monkeys, m1)?, find_humn(monkeys, m2)?) {
                (None, None) => None,
                (Some(v), None) => Some(v),
                (None, Some(v)) => Some(v),
                (Some(_), Some(_)) => bail!("found two humans?"),
            },
            Op::Mul(m1, m2) => match (find_humn(monkeys, m1)?, find_humn(monkeys, m2)?) {
                (None, None) => None,
                (Some(v), None) => Some(v),
                (None, Some(v)) => Some(v),
                (Some(_), Some(_)) => bail!("found two humans?"),
            },
            Op::Sub(m1, m2) => match (find_humn(monkeys, m1)?, find_humn(monkeys, m2)?) {
                (None, None) => None,
                (Some(v), None) => Some(v),
                (None, Some(v)) => Some(v),
                (Some(_), Some(_)) => bail!("found two humans?"),
            },
        };
        Ok(v.map(|v| {
            let mut v = v.clone();
            v.push(monkey);
            v
        }))
    }
}

fn solve2(
    monkeys: &HashMap<String, Op>,
    monkey: &str,
    path: &mut Vec<&str>,
    target: i64,
) -> Result<Option<i64>> {
    let path_monkey = path
        .pop()
        .ok_or_else(|| anyhow!("path ran out of monkeys while on monkey {}", monkey))?;
    ensure!(
        monkey == path_monkey,
        "current monkey must be the next monkey on the path, but was {} != {}",
        monkey,
        path_monkey
    );
    if monkey == "humn" {
        return Ok(Some(target));
    }
    let path_last = path
        .last()
        .ok_or_else(|| anyhow!("no last element in the path"))?;
    Ok(
        match monkeys
            .get(monkey)
            .ok_or_else(|| anyhow!("unknown monkey: {}", monkey))?
        {
            Op::Add(m1, m2) => {
                if path_last == m1 {
                    let v = solve1(monkeys, m2)?;
                    let r = solve2(monkeys, m1, path, target - v)?;
                    ensure!(r.is_some(), "r must be a target");
                    r
                } else if path_last == m2 {
                    let v = solve1(monkeys, m1)?;
                    let r = solve2(monkeys, m2, path, target - v)?;
                    ensure!(r.is_some(), "r must be a target");
                    r
                } else {
                    bail!("expected the last monkey in the path to be a child of the current node");
                }
            }
            Op::Sub(m1, m2) => {
                if path_last == m1 {
                    let v = solve1(monkeys, m2)?;
                    let r = solve2(monkeys, m1, path, target + v)?;
                    ensure!(r.is_some(), "r must be a target");
                    r
                } else if path_last == m2 {
                    let v = solve1(monkeys, m1)?;
                    let r = solve2(monkeys, m2, path, v - target)?;
                    ensure!(r.is_some(), "r must be a target");
                    r
                } else {
                    bail!("expected the last monkey in the path to be a child of the current node");
                }
            }
            Op::Mul(m1, m2) => {
                if path_last == m1 {
                    let v = solve1(monkeys, m2)?;
                    let r = solve2(monkeys, m1, path, target / v)?;
                    ensure!(r.is_some(), "r must be a target");
                    r
                } else if path_last == m2 {
                    let v = solve1(monkeys, m1)?;
                    let r = solve2(monkeys, m2, path, target / v)?;
                    ensure!(r.is_some(), "r must be a target");
                    r
                } else {
                    bail!("expected the last monkey in the path to be a child of the current node");
                }
            }
            Op::Div(m1, m2) => {
                if path_last == m1 {
                    let v = solve1(monkeys, m2)?;
                    let r = solve2(monkeys, m1, path, target * v)?;
                    ensure!(r.is_some(), "r must be a target");
                    r
                } else if path_last == m2 {
                    let v = solve1(monkeys, m1)?;
                    let r = solve2(monkeys, m2, path, v / target)?;
                    ensure!(r.is_some(), "r must be a target");
                    r
                } else {
                    bail!("expected the last monkey in the path to be a child of the current node");
                }
            }
            Op::Const(_) => {
                bail!("tried to find a target in a const node")
            }
        },
    )
}

fn part2(monkeys: &HashMap<String, Op>) -> Result<i64> {
    let (m1, m2) = match monkeys.get("root") {
        Some(Op::Add(m1, m2)) => (m1, m2),
        Some(Op::Sub(m1, m2)) => (m1, m2),
        Some(Op::Mul(m1, m2)) => (m1, m2),
        Some(Op::Div(m1, m2)) => (m1, m2),
        _ => bail!("unexpected root: {:?}", monkeys.get("root")),
    };
    let mut r1 = find_humn(monkeys, m1)?;
    let mut r2 = find_humn(monkeys, m2)?;
    match (&mut r1, &mut r2) {
        (Some(path), None) => {
            let target = solve1(monkeys, m2)?;
            if let Some(t) = solve2(monkeys, m1, path, target)? {
                Ok(t)
            } else {
                bail!("unexpected val when computing top level target")
            }
        }
        (None, Some(path)) => {
            let target = solve1(monkeys, m1)?;
            if let Some(t) = solve2(monkeys, m2, path, target)? {
                Ok(t)
            } else {
                bail!("unexpected val when computing top level target")
            }
        }
        _ => bail!("unexpected results from solve: {:?}, {:?}", r1, r2),
    }
}
