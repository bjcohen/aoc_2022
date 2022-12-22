use anyhow::{anyhow, bail, Error, Result};
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, PartialEq)]
enum Tile {
    None,
    Open,
    Wall,
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
    Walk(u64),
}

fn parse<R: Read>(reader: R) -> Result<(Vec<Vec<Tile>>, Vec<Instruction>)> {
    let line_groups = BufReader::new(reader)
        .lines()
        .group_by(|r| r.is_ok() && r.as_ref().unwrap().is_empty());
    let mut line_groups_iter = line_groups.into_iter();
    let (_, board_lines) = line_groups_iter
        .next()
        .ok_or_else(|| anyhow!("no board line group"))?;
    let board = board_lines
        .map(|line| {
            line?
                .chars()
                .map(|c| {
                    Ok(match c {
                        ' ' => Tile::None,
                        '.' => Tile::Open,
                        '#' => Tile::Wall,
                        _ => bail!("unknown tile: {}", c),
                    })
                })
                .collect::<Result<Vec<Tile>>>()
        })
        .collect::<Result<Vec<Vec<Tile>>>>()?;
    line_groups_iter.next();
    let (_, mut instruction_lines) = line_groups_iter
        .next()
        .ok_or_else(|| anyhow!("no instruction line group"))?;
    let instruction_line = instruction_lines
        .next()
        .ok_or_else(|| anyhow!("couldn't find instruction line"))
        .and_then(|x| x.map_err(Error::new))?;
    let mut instructions = vec![];
    let mut curr = None;
    for c in instruction_line.chars() {
        match c {
            'L' => {
                if let Some(n) = curr {
                    instructions.push(Instruction::Walk(n));
                    curr = None;
                }
                instructions.push(Instruction::Left);
            }
            'R' => {
                if let Some(n) = curr {
                    instructions.push(Instruction::Walk(n));
                    curr = None;
                }
                instructions.push(Instruction::Right);
            }
            _ => {
                if let Some(n) = curr {
                    curr = Some(
                        10 * n
                            + u64::from(
                                c.to_digit(10)
                                    .ok_or_else(|| anyhow!("couldn't map to digit: {}", c))?,
                            ),
                    )
                } else {
                    curr = Some(u64::from(
                        c.to_digit(10)
                            .ok_or_else(|| anyhow!("couldn't map to digit: {}", c))?,
                    ))
                }
            }
        }
    }
    if let Some(n) = curr {
        instructions.push(Instruction::Walk(n));
    }
    Ok((board, instructions))
}

fn main() -> Result<()> {
    let file = File::open("input22.txt")?;

    let (board, directions) = parse(file)?;

    println!("{}", part1(&board, &directions)?);
    println!("{}", part2(&board, &directions)?);

    Ok(())
}

fn part1(board: &[Vec<Tile>], instructions: &[Instruction]) -> Result<i64> {
    let rows = board
        .iter()
        .map(|row| {
            Ok((
                row.iter()
                    .position(|s| *s != Tile::None)
                    .ok_or_else(|| anyhow!("couldn't find first open space in row: {:?}", row))?,
                row.iter()
                    .positions(|s| *s != Tile::None)
                    .last()
                    .ok_or_else(|| anyhow!("couldn't find last open space in row: {:?}", row))?,
            ))
        })
        .collect::<Result<Vec<(usize, usize)>>>()?;
    let cols: Vec<(usize, usize)> = (0..board
        .iter()
        .map(|row| row.len())
        .max()
        .ok_or_else(|| anyhow!("no max row len"))?)
        .map(|i| {
            let mut min = None;
            let mut max = 0;
            for (j, row) in board.iter().enumerate() {
                if i >= row.len() {
                    continue;
                }
                if row[i] != Tile::None {
                    if min.is_none() {
                        min = Some(j);
                    }
                    max = j;
                }
            }
            Ok((
                min.ok_or_else(|| anyhow!("didn't find a min value for row {}", i))?,
                max,
            ))
        })
        .collect::<Result<Vec<(usize, usize)>>>()?;
    let mut row = 0;
    let mut col = rows[0].0;
    let mut facing = '>';
    for instruction in instructions {
        match instruction {
            Instruction::Left => {
                facing = match facing {
                    '>' => '^',
                    'v' => '>',
                    '<' => 'v',
                    '^' => '<',
                    _ => bail!("unknown facing: {}", facing),
                };
            }
            Instruction::Right => {
                facing = match facing {
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    '^' => '>',
                    _ => bail!("unknown facing: {}", facing),
                };
            }
            Instruction::Walk(n) => match facing {
                '>' => {
                    for _i in 0..*n {
                        let mut col_ = col + 1;
                        if col_ > rows[row].1 {
                            col_ = rows[row].0;
                        }
                        if board[row][col_] == Tile::Wall {
                            break;
                        }
                        col = col_;
                    }
                }
                'v' => {
                    for _i in 0..*n {
                        let mut row_ = row + 1;
                        if row_ > cols[col].1 {
                            row_ = cols[col].0;
                        }
                        if board[row_][col] == Tile::Wall {
                            break;
                        }
                        row = row_;
                    }
                }
                '<' => {
                    for _i in 0..*n {
                        let col_ = if col == 0 || col - 1 < rows[row].0 {
                            rows[row].1
                        } else {
                            col - 1
                        };
                        if board[row][col_] == Tile::Wall {
                            break;
                        }
                        col = col_;
                    }
                }
                '^' => {
                    for _i in 0..*n {
                        let row_ = if row == 0 || row - 1 < cols[col].0 {
                            cols[col].1
                        } else {
                            row - 1
                        };
                        if board[row_][col] == Tile::Wall {
                            break;
                        }
                        row = row_;
                    }
                }
                _ => bail!("unknown facing: {}", facing),
            },
        }
    }
    Ok(1000 * (i64::try_from(row)? + 1)
        + 4 * (i64::try_from(col)? + 1)
        + match facing {
            '>' => 0,
            'v' => 1,
            '<' => 2,
            '^' => 3,
            _ => bail!("unknown facing: {}", facing),
        })
}

fn part2(board: &[Vec<Tile>], instructions: &[Instruction]) -> Result<i64> {
    let mut row = 0;
    let mut col = board[0]
        .iter()
        .position(|t| *t != Tile::None)
        .ok_or_else(|| anyhow!("couldn't find starting col"))?;
    let mut facing = '>';
    for instruction in instructions {
        match instruction {
            Instruction::Left => {
                facing = match facing {
                    '>' => '^',
                    'v' => '>',
                    '<' => 'v',
                    '^' => '<',
                    _ => bail!("unknown facing: {}", facing),
                };
            }
            Instruction::Right => {
                facing = match facing {
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    '^' => '>',
                    _ => bail!("unknown facing: {}", facing),
                };
            }
            Instruction::Walk(n) => {
                for _i in 0..*n {
                    let (row_, col_, facing_) = match facing {
                        '>' => {
                            if row < 50 && col == 149 {
                                (149 - row, 99, '<')
                            } else if (50..100).contains(&row) && col == 99 {
                                (49, row + 50, '^')
                            } else if (100..150).contains(&row) && col == 99 {
                                (149 - row, 149, '<')
                            } else if 150 <= row && col == 49 {
                                (149, row - 100, '^')
                            } else {
                                (row, col + 1, facing)
                            }
                        }
                        '<' => {
                            if row < 50 && col == 50 {
                                (149 - row, 0, '>')
                            } else if (50..100).contains(&row) && col == 50 {
                                (100, row - 50, 'v')
                            } else if (100..150).contains(&row) && col == 0 {
                                (149 - row, 50, '>')
                            } else if 150 <= row && col == 0 {
                                (0, row - 100, 'v')
                            } else {
                                (row, col - 1, facing)
                            }
                        }
                        'v' => {
                            if col < 50 && row == 199 {
                                (0, col + 100, 'v')
                            } else if (50..100).contains(&col) && row == 149 {
                                (col + 100, 49, '<')
                            } else if 100 <= col && row == 49 {
                                (col - 50, 99, '<')
                            } else {
                                (row + 1, col, facing)
                            }
                        }
                        '^' => {
                            if col < 50 && row == 100 {
                                (50 + col, 50, '>')
                            } else if (50..100).contains(&col) && row == 0 {
                                (100 + col, 0, '>')
                            } else if 100 <= col && row == 0 {
                                (199, col - 100, '^')
                            } else {
                                (row - 1, col, facing)
                            }
                        }
                        _ => bail!("unknown facing: {}", facing),
                    };
                    if row_ >= board.len()
                        || col_ >= board[row_].len()
                        || board[row_][col_] == Tile::None
                    {
                        bail!(
                            "attempting to move from {},{},{} to empty {},{},{}",
                            row,
                            col,
                            facing,
                            row_,
                            col_,
                            facing_
                        );
                    }
                    if board[row_][col_] == Tile::Wall {
                        break;
                    }
                    row = row_;
                    col = col_;
                    facing = facing_;
                }
            }
        }
    }
    Ok(1000 * (i64::try_from(row)? + 1)
        + 4 * (i64::try_from(col)? + 1)
        + match facing {
            '>' => 0,
            'v' => 1,
            '<' => 2,
            '^' => 3,
            _ => bail!("unknown facing: {}", facing),
        })
}
