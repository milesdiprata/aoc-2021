use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use aoc_2021::Pos;

#[derive(Debug)]
struct Line {
    start: Pos<i32>,
    end: Pos<i32>,
    delta: Pos<i32>,
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        fn parse_coord(coord: &str) -> Result<Pos<i32>> {
            let (x, y) = coord
                .split_once(',')
                .ok_or_else(|| anyhow!("invalid coordinate '{coord}'"))?;
            Ok(Pos::new(x.parse()?, y.parse()?))
        }

        let (start, end) = line
            .split_once(" -> ")
            .ok_or_else(|| anyhow!("invalid line '{line}'"))?;

        let start = parse_coord(start)?;
        let end = parse_coord(end)?;

        Ok(Self {
            start,
            end,
            delta: Pos::new(
                (end.x() - start.x()).signum(),
                (end.y() - start.y()).signum(),
            ),
        })
    }
}

impl Line {
    const fn is_straight(&self) -> bool {
        self.start.x() == self.end.x() || self.start.y() == self.end.y()
    }

    const fn is_diagonal(&self) -> bool {
        self.delta.x().abs() == 1 && self.delta.y().abs() == 1
    }
}

fn part1(lines: &[Line]) -> usize {
    let mut coverage = HashMap::new();

    for line in lines.iter().filter(|&line| line.is_straight()) {
        let mut coord = line.start;

        loop {
            *coverage.entry(coord).or_insert(0_usize) += 1;

            if coord == line.end {
                break;
            }

            coord += line.delta;
        }
    }

    coverage.into_values().filter(|&count| count >= 2).count()
}

fn part2(lines: &[Line]) -> usize {
    let mut coverage = HashMap::new();

    for line in lines
        .iter()
        .filter(|&line| line.is_straight() || line.is_diagonal())
    {
        let mut coord = line.start;

        loop {
            *coverage.entry(coord).or_insert(0_usize) += 1;

            if coord == line.end {
                break;
            }

            coord += line.delta;
        }
    }

    coverage.into_values().filter(|&count| count >= 2).count()
}

fn main() -> Result<()> {
    let lines = fs::read_to_string("in/day5.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&lines);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 8_622);
    }

    {
        let start = Instant::now();
        let part2 = self::part2(&lines);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 22_037);
    }

    Ok(())
}
