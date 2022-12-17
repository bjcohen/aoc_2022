use anyhow::{ensure, Result};
use std::{fs::File, io::Read};

#[derive(Debug)]
enum Dir {
    L,
    R,
}

fn parse<R: Read>(mut reader: R) -> Result<Vec<Dir>> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf
        .iter()
        .filter_map(|i| match *i as char {
            '<' => Some(Dir::L),
            '>' => Some(Dir::R),
            _ => None,
        })
        .collect())
}

fn main() -> Result<()> {
    let file = File::open("input17.txt")?;
    let gusts = parse(file)?;

    println!("{}", sim(&gusts, 2022)?);

    println!("{}", sim(&gusts, 1_000_000_000_000)?);

    Ok(())
}

fn _print(board: &Vec<[bool; 7]>, mut topn: usize) {
    if topn == 0 || topn > board.len() {
        topn = board.len();
    }
    for row in board.iter().rev().take(topn) {
        println!(
            "|{}|",
            row.iter()
                .map(|&x| if x { '#' } else { '.' })
                .collect::<String>()
        );
    }
    if topn == board.len() {
        println!("+-------+");
    }
}

fn is_blocked(
    board: &Vec<[bool; 7]>,
    shape: &(Vec<(usize, usize)>, usize),
    x: usize,
    y: usize,
) -> bool {
    for (dx, dy) in &shape.0 {
        if x + dx > 6 {
            return true;
        }
        if y + dy < board.len() && board[y + dy][x + dx] {
            return true;
        }
    }
    false
}

fn sim(gusts: &Vec<Dir>, n: usize) -> Result<usize> {
    let (len, cycle) = sim_(gusts, 0, 0, n)?;
    if let Some(Cycle {
        tshape,
        clen,
        cshape,
        ishape,
        igust,
    }) = cycle
    {
        let ncycles = (n - tshape) / cshape;
        let rshape = n - ncycles * cshape - tshape;
        let (lenr, cycler) = sim_(gusts, ishape, igust, rshape)?;
        ensure!(cycler.is_none(), "found trailing cycle");
        return Ok(len + ncycles * clen + lenr);
    }
    Ok(len)
}

struct Cycle {
    tshape: usize,
    clen: usize,
    cshape: usize,
    ishape: usize,
    igust: usize,
}

fn sim_(gusts: &Vec<Dir>, ishape: usize, igust: usize, n: usize) -> Result<(usize, Option<Cycle>)> {
    let shapes: Vec<(Vec<(usize, usize)>, usize)> = vec![
        (vec![(0, 0), (1, 0), (2, 0), (3, 0)], 1),
        (vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)], 3),
        (vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], 3),
        (vec![(0, 0), (0, 1), (0, 2), (0, 3)], 4),
        (vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2),
    ];

    let mut board: Vec<[bool; 7]> = Vec::new();

    let mut igust = igust;
    let mut last: Option<(usize, usize, usize, usize)> = None;

    for (tshape, (ishape, shape)) in shapes
        .iter()
        .enumerate()
        .cycle()
        .enumerate()
        .skip(ishape)
        .take(n)
    {
        let mut x = 2;
        let mut y = board.len() + 3;

        loop {
            let gust = &gusts[igust];
            igust = (igust + 1) % gusts.len();

            let x_ = match gust {
                Dir::L => {
                    if x > 0 {
                        x - 1
                    } else {
                        0
                    }
                }
                Dir::R => x + 1,
            };

            if !is_blocked(&board, shape, x_, y) {
                x = x_;
            }
            if y > 0 && !is_blocked(&board, shape, x, y - 1) {
                y -= 1;
            } else {
                let maxy = y + shape.1 - 1;
                if maxy >= board.len() {
                    board.append(&mut (board.len()..maxy + 1).map(|_| [false; 7]).collect());
                }
                for (dx, dy) in &shape.0 {
                    ensure!(
                        !board[y + dy][x + dx],
                        "setting board pos that was already set: {}, {}",
                        x + dx,
                        y + dy,
                    );
                    board[y + dy][x + dx] = true;
                }
                break;
            }
        }

        if board[board.len() - 1].iter().all(|x| *x) {
            if let Some((tshapel, lenl, ishapel, igustl)) = last {
                if ishapel == ishape && igustl == igust {
                    return Ok((
                        board.len(),
                        Some(Cycle {
                            tshape: tshape + 1,
                            clen: board.len() - lenl,
                            cshape: tshape - tshapel,
                            ishape: ishape + 1,
                            igust,
                        }),
                    ));
                }
            } else {
                last = Some((tshape, board.len(), ishape, igust));
            }
        }
    }
    Ok((board.len(), None))
}
