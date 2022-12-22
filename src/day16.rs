use anyhow::{anyhow, ensure, Error, Result};
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn parse<R: Read>(reader: R) -> Result<Vec<(String, i64, Vec<String>)>> {
    let re =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)")?;
    BufReader::new(reader)
        .lines()
        .map(|l| {
            l.map_err(Error::new).and_then(|l| {
                let matches = re
                    .captures(&l)
                    .ok_or_else(|| anyhow!("re no match: {}", l))?;
                Ok((
                    matches[1].to_string(),
                    matches[2].parse()?,
                    matches[3].split(", ").map(String::from).collect(),
                ))
            })
        })
        .collect()
}

#[allow(clippy::type_complexity)]
fn flatten(
    valves: &[(String, i64, Vec<String>)],
) -> Result<Vec<(&String, i64, Vec<(&String, i64)>)>> {
    let mut zero_links: HashMap<&String, (&String, &String)> = HashMap::new();
    let mut result = vec![];
    for valve in valves {
        if valve.1 == 0 && valve.0 != "AA" {
            ensure!(
                valve.2.len() == 2,
                "valve len must equal 2, was {}",
                valve.2.len()
            );
            zero_links.insert(&valve.0, (&valve.2[0], &valve.2[1]));
        }
    }
    for valve in valves {
        if valve.1 != 0 || valve.0 == "AA" {
            result.push((
                &valve.0,
                valve.1,
                valve
                    .2
                    .iter()
                    .map(|mut d: &String| {
                        let mut n = 1;
                        let mut prev = &valve.0;
                        while let Some((d1, d2)) = zero_links.get(d) {
                            if *d1 != prev {
                                prev = d;
                                d = d1;
                            } else {
                                prev = d;
                                d = d2;
                            }
                            n += 1;
                        }
                        (d, n)
                    })
                    .collect(),
            ));
        }
    }
    Ok(result)
}

#[allow(clippy::type_complexity)]
fn solve<'a>(
    valves: &HashMap<&'a String, (&i64, &Vec<(&'a String, i64)>)>,
    curr: &'a String,
    time: i64,
    maxtime: i64,
    opened: &HashMap<&String, i64>,
    memo: &mut HashMap<(&'a String, i64, String), (i64, Vec<(&'a String, i64)>)>,
) -> Result<(i64, Vec<(&'a String, i64)>)> {
    let opened_str = opened.keys().sorted().join(",");
    if let Some(result) = memo.get(&(curr, time, opened_str.clone())) {
        return Ok(result.clone());
    }
    if time >= maxtime {
        return Ok((0, vec![]));
    }
    let mut max = 0;
    let mut maxvalves = vec![];
    let (flow, outs) = valves
        .get(curr)
        .ok_or_else(|| anyhow!("couldn't find loc {}", curr))?;
    for (out, cost) in *outs {
        let (s1, m1) = solve(valves, out, time + cost, maxtime, opened, memo)?;
        if s1 > max {
            max = s1;
            maxvalves = m1;
        }
    }
    if !opened.contains_key(curr) {
        let mut opened = opened.clone();
        opened.insert(curr, time);
        let (s1, m1) = solve(valves, curr, time + 1, maxtime, &opened, memo)?;
        let s2 = s1 + *flow * (maxtime - time - 1);
        if s2 > max {
            max = s2;
            maxvalves = m1;
            maxvalves.push((curr, time + 1));
        }
    }
    memo.insert((curr, time, opened_str), (max, maxvalves.clone()));
    Ok((max, maxvalves))
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn solve2<'a>(
    valves: &HashMap<&'a String, (i64, Vec<(&'a String, i64)>)>,
    curr1: &'a String,
    time1: i64,
    curr2: &'a String,
    time2: i64,
    maxtime: i64,
    opened: &HashMap<&String, i64>,
    memo: &mut HashMap<(&'a String, i64, &'a String, i64, String), i64>,
) -> Result<i64> {
    let opened_str = opened.keys().sorted().join(",");
    let memo_key = (curr1, time1, curr2, time2, opened_str);
    if let Some(result) = memo.get(&memo_key) {
        return Ok(*result);
    }
    // println!("{:?}", memo_key);
    let mut max = 0;
    if time1 < maxtime {
        let (flow1, outs1) = valves
            .get(curr1)
            .ok_or_else(|| anyhow!("couldn't find loc {}", curr1))?;
        for (out1, cost1) in outs1 {
            let s1 = solve2(
                valves,
                out1,
                time1 + cost1,
                curr2,
                time2,
                maxtime,
                opened,
                memo,
            )?;
            if s1 > max {
                max = s1;
            }
        }
        if !opened.contains_key(curr1) {
            let mut opened = opened.clone();
            opened.insert(curr1, time1);
            let s1 = solve2(
                valves,
                curr1,
                time1 + 1,
                curr2,
                time2,
                maxtime,
                &opened,
                memo,
            )?;
            let s2 = s1 + *flow1 * (maxtime - time1 - 1);
            if s2 > max {
                max = s2;
            }
        }
    }
    if time2 < maxtime {
        let (flow2, outs2) = valves
            .get(curr2)
            .ok_or_else(|| anyhow!("couldn't find loc {}", curr2))?;
        for (out2, cost2) in outs2 {
            let s1 = solve2(
                valves,
                curr1,
                time1,
                out2,
                time2 + cost2,
                maxtime,
                opened,
                memo,
            )?;
            if s1 > max {
                max = s1;
            }
        }
        if !opened.contains_key(curr2) {
            let mut opened = opened.clone();
            opened.insert(curr2, time2);
            let s1 = solve2(
                valves,
                curr1,
                time1,
                curr2,
                time2 + 1,
                maxtime,
                &opened,
                memo,
            )?;
            let s2 = s1 + *flow2 * (maxtime - time2 - 1);
            if s2 > max {
                max = s2;
            }
        }
    }
    memo.insert(memo_key, max);
    Ok(max)
}

fn main() -> Result<()> {
    let file = File::open("input16.txt")?;
    let valves = parse(file)?;
    let flattened = flatten(&valves)?;

    let flattened_map = flattened
        .iter()
        .map(|(cur, flow, out)| (*cur, (flow, out)))
        .collect::<HashMap<&String, (&i64, &Vec<(&String, i64)>)>>();

    let aa = "AA".to_string();

    let result = solve(
        &flattened_map,
        &aa,
        0,
        30,
        &HashMap::new(),
        &mut HashMap::new(),
    )?;

    let mut all_pairs: HashMap<(&String, &String), i64> = HashMap::new();

    for (l1, (_flow, outs)) in &flattened_map {
        for (out, dist) in *outs {
            all_pairs.insert((l1, out), *dist);
        }
    }

    for l1 in flattened_map.keys() {
        for l2 in flattened_map.keys() {
            for l3 in flattened_map.keys() {
                let l1l2 = all_pairs.get(&(l1, l2));
                let l2l3 = all_pairs.get(&(l2, l3));
                if let Some(d12) = l1l2 {
                    if let Some(d23) = l2l3 {
                        let l1l3 = all_pairs.get(&(l1, l3));
                        if let Some(d13) = l1l3 {
                            all_pairs.insert((l1, l3), i64::min(*d13, d12 + d23));
                        } else {
                            all_pairs.insert((l1, l3), d12 + d23);
                        }
                    }
                }
            }
        }
    }

    #[allow(clippy::type_complexity)]
    let mut all_dists: HashMap<&String, (i64, Vec<(&String, i64)>)> = HashMap::new();
    for ((v1, v2), dist) in all_pairs {
        if let Some((flow, outs)) = all_dists.get(v1) {
            let mut outs_ = vec![];
            outs.iter().for_each(|&o| outs_.push(o));
            outs_.push((v2, dist));
            all_dists.insert(v1, (*flow, outs_));
        } else {
            let (flow, _outs) = flattened_map
                .get(v1)
                .ok_or_else(|| anyhow!("missing {} in flattened_map", v1))?;
            let outs = vec![(v2, dist)];
            all_dists.insert(v1, (**flow, outs));
        }
    }

    println!("{} {:?}", result.0, result.1);

    let result2 = solve2(
        &all_dists,
        &aa,
        0,
        &aa,
        0,
        26,
        &HashMap::new(),
        &mut HashMap::new(),
    )?;

    println!("{}", result2);

    Ok(())
}
