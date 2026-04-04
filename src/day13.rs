use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

type Pos = aoc_2021::Pos<usize>;

#[derive(Clone, Copy, Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

struct Paper {
    dots: HashSet<Pos>,
}

impl std::fmt::Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.dots.iter().map(|&p| p.x()).min().unwrap_or_default();
        let x_max = self.dots.iter().map(|&p| p.x()).max().unwrap_or_default();
        let y_min = self.dots.iter().map(|&p| p.y()).min().unwrap_or_default();
        let y_max = self.dots.iter().map(|&p| p.y()).max().unwrap_or_default();

        for y in y_min..=y_max {
            if y > y_min {
                writeln!(f)?;
            }

            for x in x_min..=x_max {
                if self.dots.contains(&Pos::new(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }

        Ok(())
    }
}

impl FromStr for Fold {
    type Err = Error;

    fn from_str(fold: &str) -> Result<Self> {
        let line = fold
            .strip_prefix("fold along ")
            .ok_or_else(|| anyhow!("invalid instruction '{fold}'"))?;

        if let Some(x) = line.strip_prefix("x=") {
            Ok(Self::X(x.parse()?))
        } else if let Some(y) = line.strip_prefix("y=") {
            Ok(Self::Y(y.parse()?))
        } else {
            bail!("invalid line '{line}'")
        }
    }
}

impl FromStr for Paper {
    type Err = Error;

    fn from_str(dots: &str) -> Result<Self> {
        fn parse_coord(coord: &str) -> Result<Pos> {
            let (x, y) = coord
                .split_once(',')
                .ok_or_else(|| anyhow!("invalid coordinate '{coord}'"))?;

            Ok(Pos::new(x.parse()?, y.parse()?))
        }

        Ok(Self {
            dots: dots.lines().map(parse_coord).collect::<Result<_>>()?,
        })
    }
}

impl Paper {
    fn fold(&mut self, fold: Fold) {
        const fn fold_x(dot: Pos, x: usize) -> Pos {
            if dot.x() > x {
                Pos::new((2 * x) - dot.x(), dot.y())
            } else {
                dot
            }
        }

        const fn fold_y(dot: Pos, y: usize) -> Pos {
            if dot.y() > y {
                Pos::new(dot.x(), (2 * y) - dot.y())
            } else {
                dot
            }
        }

        match fold {
            Fold::X(x) => {
                self.dots = self.dots.iter().map(|&dot| fold_x(dot, x)).collect();
            }
            Fold::Y(y) => {
                self.dots = self.dots.iter().map(|&dot| fold_y(dot, y)).collect();
            }
        }
    }
}

fn parse() -> Result<(Paper, Vec<Fold>)> {
    let input = fs::read_to_string("in/day13.txt")?;
    let (dots, folds) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("invalid input"))?;

    let paper = Paper::from_str(dots)?;
    let folds = folds.lines().map(Fold::from_str).collect::<Result<_>>()?;

    Ok((paper, folds))
}

fn part1(paper: &mut Paper, folds: &[Fold]) -> usize {
    if let Some(&fold) = folds.first() {
        paper.fold(fold);
    }

    paper.dots.len()
}

fn part2(paper: &mut Paper, folds: &[Fold]) {
    for &fold in folds.iter().skip(1) {
        paper.fold(fold);
    }
}

fn main() -> Result<()> {
    let (mut paper, folds) = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&mut paper, &folds);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 710);
    };

    {
        let start = Instant::now();
        self::part2(&mut paper, &folds);
        let elapsed = start.elapsed();

        println!("Part 2 ({elapsed:?})");
        println!("{paper}");
    };

    Ok(())
}
