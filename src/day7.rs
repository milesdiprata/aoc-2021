use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CrabSubmarine {
    pos: u32,
}

impl FromStr for CrabSubmarine {
    type Err = Error;

    fn from_str(pos: &str) -> Result<Self> {
        Ok(Self { pos: pos.parse()? })
    }
}

fn part1(mut crabs: Vec<CrabSubmarine>) -> u32 {
    crabs.sort_unstable();

    let median = crabs[crabs.len() / 2].pos;

    crabs
        .into_iter()
        .map(|crab| crab.pos.abs_diff(median))
        .sum()
}

fn part2(crabs: &[CrabSubmarine]) -> u32 {
    let fuel = |pos| {
        crabs
            .iter()
            .map(|&crab| {
                let dist = crab.pos.abs_diff(pos);
                (dist * (dist + 1)) / 2
            })
            .sum::<u32>()
    };

    #[allow(clippy::cast_precision_loss)]
    let mean = crabs.iter().map(|crab| f64::from(crab.pos)).sum::<f64>() / crabs.len() as f64;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fuel(mean.floor() as u32).min(fuel(mean.ceil() as u32))
}

fn main() -> Result<()> {
    let crabs = fs::read_to_string("in/day7.txt")?
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(crabs.clone());
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 341_534);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&crabs);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 93_397_632);
    };

    Ok(())
}
