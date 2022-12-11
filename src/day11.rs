use anyhow::{anyhow, bail, Result};
use regex::Regex;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

fn main() -> Result<()> {
    let file = File::open("input11.txt")?;
    let mut lines = BufReader::new(file).lines();

    let re_monkey = Regex::new(r"Monkey (\d+):")?;
    let re_items = Regex::new(r"  Starting items: (.+)")?;
    let re_operation = Regex::new(r"  Operation: new = old (.) (.+)")?;
    let re_test = Regex::new(r"  Test: divisible by (\d+)")?;
    let re_iftrue = Regex::new(r"    If true: throw to monkey (\d+)")?;
    let re_iffalse = Regex::new(r"    If false: throw to monkey (\d+)")?;

    let mut monkeys = vec![];
    let mut items = vec![];
    let mut operations = vec![];
    let mut tests = vec![];
    let mut iftrues = vec![];
    let mut iffalses = vec![];

    loop {
        let num = re_monkey
            .captures(&lines.next().ok_or_else(|| anyhow!("no line"))??)
            .ok_or_else(|| anyhow!("no captures"))?[1]
            .parse::<usize>()?;
        monkeys.push(num);
        let itemlist = re_items
            .captures(&lines.next().ok_or_else(|| anyhow!("no line"))??)
            .ok_or_else(|| anyhow!("no captures"))?[1]
            .split(", ")
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<i32>, ParseIntError>>()?;
        items.push(VecDeque::from(itemlist));
        let operation_line = lines.next().ok_or_else(|| anyhow!("no line"))??;
        let operation_captures = re_operation
            .captures(&operation_line)
            .ok_or_else(|| anyhow!("no captures"))?;
        let operation = (
            operation_captures[1].to_string(),
            operation_captures[2].to_string(),
        );
        operations.push(operation);
        let test = re_test
            .captures(&lines.next().ok_or_else(|| anyhow!("no line"))??)
            .ok_or_else(|| anyhow!("no captures"))?[1]
            .parse::<i32>()?;
        tests.push(test);
        let iftrue = re_iftrue
            .captures(&lines.next().ok_or_else(|| anyhow!("no line"))??)
            .ok_or_else(|| anyhow!("no captures"))?[1]
            .parse::<usize>()?;
        iftrues.push(iftrue);
        let iffalse = re_iffalse
            .captures(&lines.next().ok_or_else(|| anyhow!("no line"))??)
            .ok_or_else(|| anyhow!("no captures"))?[1]
            .parse::<usize>()?;
        iffalses.push(iffalse);
        if lines.next().is_none() {
            break;
        }
    }

    let mut inspections = (0..monkeys.len()).map(|_| 0).collect::<Vec<i32>>();

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            while let Some(mut i) = items[m].pop_front() {
                inspections[m] += 1;
                let operand2 = if operations[m].1 == "old" {
                    i
                } else {
                    operations[m].1.parse::<i32>()?
                };
                i = match operations[m].0.as_str() {
                    "*" => i * operand2,
                    "+" => i + operand2,
                    _ => bail!("unhandled operation: {}", operations[m].0),
                };
                i /= 3;
                let to_monkey = if i % tests[m] == 0 {
                    iftrues[m]
                } else {
                    iffalses[m]
                };
                items[to_monkey].push_back(i);
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    println!("{:?}", inspections.iter().take(2).product::<i32>());

    let file = File::open("input11.txt")?;
    let mut lines = BufReader::new(file).lines();

    let mut monkeys = vec![];
    let mut items = vec![];
    let mut operations = vec![];
    let mut tests = vec![];
    let mut iftrues = vec![];
    let mut iffalses = vec![];

    loop {
        let num = re_monkey
            .captures(&lines.next().ok_or_else(|| anyhow!("no line"))??)
            .ok_or_else(|| anyhow!("no captures"))?[1]
            .parse::<usize>()?;
        monkeys.push(num);
        let itemlist = re_items
            .captures(&lines.next().ok_or_else(|| anyhow!("no line"))??)
            .ok_or_else(|| anyhow!("no captures"))?[1]
            .split(", ")
            .map(|n| n.parse::<i64>())
            .collect::<Result<Vec<i64>, ParseIntError>>()?;
        items.push(VecDeque::from(itemlist));
        let operation_line = lines.next().ok_or_else(|| anyhow!("no line"))??;
        let operation_captures = re_operation
            .captures(&operation_line)
            .ok_or_else(|| anyhow!("no captures"))?;
        let operation = (
            operation_captures[1].to_string(),
            operation_captures[2].to_string(),
        );
        operations.push(operation);
        let test = re_test
            .captures(&lines.next().ok_or_else(|| anyhow!("no line"))??)
            .ok_or_else(|| anyhow!("no captures"))?[1]
            .parse::<i64>()?;
        tests.push(test);
        let iftrue = re_iftrue
            .captures(&lines.next().ok_or_else(|| anyhow!("no line"))??)
            .ok_or_else(|| anyhow!("no captures"))?[1]
            .parse::<usize>()?;
        iftrues.push(iftrue);
        let iffalse = re_iffalse
            .captures(&lines.next().ok_or_else(|| anyhow!("no line"))??)
            .ok_or_else(|| anyhow!("no captures"))?[1]
            .parse::<usize>()?;
        iffalses.push(iffalse);
        if lines.next().is_none() {
            break;
        }
    }

    let mut inspections = (0..monkeys.len()).map(|_| 0).collect::<Vec<i64>>();
    let modulus = tests.iter().product::<i64>();

    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            while let Some(mut i) = items[m].pop_front() {
                inspections[m] += 1;
                let operand2 = if operations[m].1 == "old" {
                    i
                } else {
                    operations[m].1.parse::<i64>()?
                };
                i = match operations[m].0.as_str() {
                    "*" => i * operand2,
                    "+" => i + operand2,
                    _ => bail!("unhandled operation: {}", operations[m].0),
                };
                i %= modulus;
                let to_monkey = if i % tests[m] == 0 {
                    iftrues[m]
                } else {
                    iffalses[m]
                };
                items[to_monkey].push_back(i);
            }
        }
    }

    inspections.sort();
    inspections.reverse();
    println!("{:?}", inspections.iter().take(2).product::<i64>());

    Ok(())
}
