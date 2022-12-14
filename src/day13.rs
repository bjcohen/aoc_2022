use anyhow::{anyhow, bail, Error, Result};
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Eq)]
enum List {
    Num(u32),
    List(Vec<List>),
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (List::Num(i1), List::Num(i2)) => i1.cmp(i2),
            (List::List(_), List::Num(i2)) => self.cmp(&List::List(vec![List::Num(*i2)])),
            (List::Num(i1), List::List(_)) => List::List(vec![List::Num(*i1)]).cmp(other),
            (List::List(l1), List::List(l2)) => {
                for i in 0..usize::min(l1.len(), l2.len()) {
                    let item_order = l1[i].cmp(&l2[i]);
                    if item_order == Ordering::Less {
                        return Ordering::Less;
                    } else if item_order == Ordering::Greater {
                        return Ordering::Greater;
                    }
                }
                l1.len().cmp(&l2.len())
            }
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (List::Num(i1), List::Num(i2)) => i1 == i2,
            (List::Num(_i1), List::List(_l2)) => false,
            (List::List(_l1), List::Num(_i2)) => false,
            (List::List(l1), List::List(l2)) => l1 == l2,
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &mut dyn Iterator<Item = char>) -> Result<List> {
    let mut n = None;
    let mut v = vec![];
    while let Some(c) = input.next() {
        if let Some(i) = c.to_digit(10) {
            if let Some(List::Num(j)) = n {
                n = Some(List::Num(10 * j + i));
            } else if n.is_none() {
                n = Some(List::Num(i));
            } else {
                bail!("expected n to be a num, but was {:?}", n);
            }
        } else if c == ',' {
            v.push(n.ok_or_else(|| anyhow!("expected a value"))?);
            n = None;
        } else if c == ']' {
            if let Some(n_) = n {
                v.push(n_);
            }
            return Ok(List::List(v));
        } else if c == '[' {
            n = Some(parse(input)?);
        } else {
            bail!("unhandled value of c: {}", c);
        }
    }
    bail!("no more characters")
}

fn main() -> Result<()> {
    let file = File::open("input13.txt")?;
    let mut lines = BufReader::new(file).lines();

    let mut sum_indices = 0;

    for i in 1.. {
        let v1 = lines.next().ok_or_else(|| anyhow!("no v1"))??;
        let v2 = lines.next().ok_or_else(|| anyhow!("no v2"))??;

        let mut chars1 = v1.chars();
        if let Some(c) = chars1.next() {
            if c != '[' {
                bail!("no open bracked in pair 1")
            }
        }

        let mut chars2 = v2.chars();
        if let Some(c) = chars2.next() {
            if c != '[' {
                bail!("no open bracked in pair 2")
            }
        }

        let parsed1 = parse(&mut chars1)?;
        let parsed2 = parse(&mut chars2)?;

        if parsed1.cmp(&parsed2) == Ordering::Less {
            sum_indices += i;
        }

        if lines.next().is_none() {
            break;
        }
    }

    println!("{}", sum_indices);

    let file = File::open("input13.txt")?;
    let mut lists = BufReader::new(file)
        .lines()
        .filter(|l| !l.as_ref().unwrap().is_empty())
        .map(|l| {
            l.map_err(Error::new)
                .and_then(|l| parse(&mut l.chars().skip(1)))
        })
        .collect::<Result<Vec<List>>>()?;

    lists.push(List::List(vec![List::List(vec![List::Num(2)])]));
    lists.push(List::List(vec![List::List(vec![List::Num(6)])]));

    lists.sort();

    let i1 = lists
        .iter()
        .position(|l| *l == List::List(vec![List::List(vec![List::Num(2)])]))
        .ok_or_else(|| anyhow!("couldn't find divider 1"))?;
    let i2 = lists
        .iter()
        .position(|l| *l == List::List(vec![List::List(vec![List::Num(6)])]))
        .ok_or_else(|| anyhow!("couldn't find divider 2"))?;

    println!("{}", (i1 + 1) * (i2 + 1));

    Ok(())
}
