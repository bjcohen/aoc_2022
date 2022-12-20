use anyhow::{anyhow, Error, Result};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug)]
struct Blueprint {
    ore: i64,
    clay: i64,
    obsidian: (i64, i64),
    geode: (i64, i64),
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct State {
    time: i64,
    ore: i64,
    clay: i64,
    obsidian: i64,
    // geode: i64,
    ore_robots: i64,
    clay_robots: i64,
    obsidian_robots: i64,
    geode_robots: i64,
}

struct MaxResourceNeeds {
    ore: i64,
    clay: i64,
    obsidian: i64,
}

struct CouldHaveProduced {
    ore_robot: bool,
    clay_robot: bool,
    obsidian_robot: bool,
}

impl CouldHaveProduced {
    fn none() -> Self {
        CouldHaveProduced {
            ore_robot: false,
            clay_robot: false,
            obsidian_robot: false,
        }
    }
}

fn parse<R: Read>(reader: R) -> Result<Vec<Blueprint>> {
    let re = Regex::new(
        r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.",
    )?;
    BufReader::new(reader)
        .lines()
        .map(|r| {
            r.map_err(Error::new).and_then(|l| {
                let cs = re
                    .captures(&l)
                    .ok_or_else(|| anyhow!("line didn't match: {}", l))?;
                Ok(Blueprint {
                    ore: cs[1].parse()?,
                    clay: cs[2].parse()?,
                    obsidian: (cs[3].parse()?, cs[4].parse()?),
                    geode: (cs[5].parse()?, cs[6].parse()?),
                })
            })
        })
        .collect()
}

fn main() -> Result<()> {
    let file = File::open("input19.txt")?;

    let blueprints = parse(file)?;

    println!("{}", part1(&blueprints)?);
    println!("{}", part2(&blueprints)?);

    Ok(())
}

fn part1(blueprints: &[Blueprint]) -> Result<i64> {
    blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| {
            solve(
                b,
                State {
                    time: 0,
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    // geode: 0,
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                },
                &mut HashMap::new(),
                24,
                &get_max_resource_needs(b)?,
                CouldHaveProduced::none(),
            )
            .and_then(|q| Ok((TryInto::<i64>::try_into(i)? + 1) * q))
        })
        .sum()
}

fn part2(blueprints: &[Blueprint]) -> Result<i64> {
    blueprints
        .iter()
        .take(3)
        .map(|b| {
            solve(
                b,
                State {
                    time: 0,
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                },
                &mut HashMap::new(),
                32,
                &get_max_resource_needs(b)?,
                CouldHaveProduced::none(),
            )
        })
        .product()
}

fn solve(
    blueprint: &Blueprint,
    state: State,
    memo: &mut HashMap<State, i64>,
    max_time: i64,
    max_resource_needs: &MaxResourceNeeds,
    could_have_produced: CouldHaveProduced,
) -> Result<i64> {
    if let Some(max) = memo.get(&state) {
        return Ok(*max);
    }
    if state.time == max_time {
        return Ok(0);
    }

    let mut max = -1;

    if state.ore >= blueprint.geode.0 && state.obsidian >= blueprint.geode.1 {
        max = i64::max(
            max,
            solve(
                blueprint,
                State {
                    time: state.time + 1,
                    ore: state.ore - blueprint.geode.0 + state.ore_robots,
                    clay: state.clay + state.clay_robots,
                    obsidian: state.obsidian - blueprint.geode.1 + state.obsidian_robots,
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots + 1,
                },
                memo,
                max_time,
                max_resource_needs,
                CouldHaveProduced::none(),
            )?,
        );
    } else {
        let mut could_produce_obsidian_robot = false;
        let mut could_produce_clay_robot = false;
        let mut could_produce_ore_robot = false;

        if state.ore >= blueprint.obsidian.0
            && state.clay >= blueprint.obsidian.1
            && state.obsidian_robots < max_resource_needs.obsidian
        {
            could_produce_obsidian_robot = true;
            if !could_have_produced.obsidian_robot {
                max = i64::max(
                    max,
                    solve(
                        blueprint,
                        State {
                            time: state.time + 1,
                            ore: state.ore - blueprint.obsidian.0 + state.ore_robots,
                            clay: state.clay - blueprint.obsidian.1 + state.clay_robots,
                            obsidian: state.obsidian + state.obsidian_robots,
                            ore_robots: state.ore_robots,
                            clay_robots: state.clay_robots,
                            obsidian_robots: state.obsidian_robots + 1,
                            geode_robots: state.geode_robots,
                        },
                        memo,
                        max_time,
                        max_resource_needs,
                        CouldHaveProduced::none(),
                    )?,
                );
            }
        }
        if state.ore >= blueprint.clay && state.clay_robots < max_resource_needs.clay {
            could_produce_clay_robot = true;
            if !could_have_produced.clay_robot {
                max = i64::max(
                    max,
                    solve(
                        blueprint,
                        State {
                            time: state.time + 1,
                            ore: state.ore - blueprint.clay + state.ore_robots,
                            clay: state.clay + state.clay_robots,
                            obsidian: state.obsidian + state.obsidian_robots,
                            ore_robots: state.ore_robots,
                            clay_robots: state.clay_robots + 1,
                            obsidian_robots: state.obsidian_robots,
                            geode_robots: state.geode_robots,
                        },
                        memo,
                        max_time,
                        max_resource_needs,
                        CouldHaveProduced::none(),
                    )?,
                );
            }
        }
        if state.ore >= blueprint.ore && state.ore_robots < max_resource_needs.ore {
            could_produce_ore_robot = true;
            if !could_have_produced.ore_robot {
                max = i64::max(
                    max,
                    solve(
                        blueprint,
                        State {
                            time: state.time + 1,
                            ore: state.ore - blueprint.ore + state.ore_robots,
                            clay: state.clay + state.clay_robots,
                            obsidian: state.obsidian + state.obsidian_robots,
                            ore_robots: state.ore_robots + 1,
                            clay_robots: state.clay_robots,
                            obsidian_robots: state.obsidian_robots,
                            geode_robots: state.geode_robots,
                        },
                        memo,
                        max_time,
                        max_resource_needs,
                        CouldHaveProduced::none(),
                    )?,
                );
            }
        }
        max = i64::max(
            max,
            solve(
                blueprint,
                State {
                    time: state.time + 1,
                    ore: state.ore + state.ore_robots,
                    clay: state.clay + state.clay_robots,
                    obsidian: state.obsidian + state.obsidian_robots,
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots,
                },
                memo,
                max_time,
                max_resource_needs,
                CouldHaveProduced {
                    ore_robot: could_produce_ore_robot,
                    clay_robot: could_produce_clay_robot,
                    obsidian_robot: could_produce_obsidian_robot,
                },
            )?,
        );
    }

    let geode_robots = state.geode_robots;
    memo.insert(state, max + geode_robots);
    Ok(max + geode_robots)
}

fn get_max_resource_needs(blueprint: &Blueprint) -> Result<MaxResourceNeeds> {
    Ok(MaxResourceNeeds {
        ore: [
            blueprint.ore,
            blueprint.clay,
            blueprint.obsidian.0,
            blueprint.obsidian.0,
        ]
        .iter()
        .max()
        .ok_or_else(|| anyhow!("no max ore"))?
        .to_owned(),
        clay: blueprint.obsidian.1,
        obsidian: blueprint.geode.1,
    })
}
