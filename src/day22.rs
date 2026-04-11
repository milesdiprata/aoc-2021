use std::collections::HashSet;
use std::fs;
use std::ops::Range;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Debug)]
struct Step {
    state: bool,
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(step: &str) -> Result<Self> {
        fn parse_range(range: &str) -> Result<RangeInclusive<i64>> {
            let (start, end) = range
                .split_once("..")
                .ok_or_else(|| anyhow!("invalid range '{range}'"))?;

            Ok(start.parse()?..=end.parse()?)
        }

        let (state, step) = step
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid step '{step}'"))?;

        let state = match state {
            "on" => true,
            "off" => false,
            _ => bail!("invalid state '{state}'"),
        };

        let mut ranges = step.split(',');

        let x = ranges
            .next()
            .and_then(|range| range.strip_prefix("x="))
            .ok_or_else(|| anyhow!("missing x-range"))?;
        let y = ranges
            .next()
            .and_then(|range| range.strip_prefix("y="))
            .ok_or_else(|| anyhow!("missing y-range"))?;
        let z = ranges
            .next()
            .and_then(|range| range.strip_prefix("z="))
            .ok_or_else(|| anyhow!("missing z-range"))?;

        Ok(Self {
            state,
            x: parse_range(x)?,
            y: parse_range(y)?,
            z: parse_range(z)?,
        })
    }
}

fn part1(steps: &[Step]) -> usize {
    let mut cubes = HashSet::new();

    for step in steps {
        for x in (*step.x.start()).max(-50)..=(*step.x.end()).min(50) {
            for y in (*step.y.start()).max(-50)..=(*step.y.end()).min(50) {
                for z in (*step.z.start()).max(-50)..=(*step.z.end()).min(50) {
                    if step.state {
                        cubes.insert(Cube { x, y, z });
                    } else {
                        cubes.remove(&Cube { x, y, z });
                    }
                }
            }
        }
    }

    cubes.len()
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn part2(steps: &[Step]) -> usize {
    fn index_range(sorted: &[i64], lo: i64, hi: i64) -> Range<usize> {
        sorted.partition_point(|&v| v < lo)..sorted.partition_point(|&v| v < hi)
    }

    let xs = {
        let mut boundaries = steps
            .iter()
            .flat_map(|step| [*step.x.start(), *step.x.end() + 1])
            .collect::<Vec<_>>();
        boundaries.dedup();
        boundaries.sort_unstable();
        boundaries
    };
    let ys = {
        let mut boundaries = steps
            .iter()
            .flat_map(|step| [*step.y.start(), *step.y.end() + 1])
            .collect::<Vec<_>>();
        boundaries.dedup();
        boundaries.sort_unstable();
        boundaries
    };
    let zs = {
        let mut boundaries = steps
            .iter()
            .flat_map(|step| [*step.z.start(), *step.z.end() + 1])
            .collect::<Vec<_>>();
        boundaries.dedup();
        boundaries.sort_unstable();
        boundaries
    };

    let mut grid = vec![vec![vec![false; zs.len()]; ys.len()]; xs.len()];
    for step in steps {
        let xi = index_range(&xs, *step.x.start(), *step.x.end() + 1);
        let yi = index_range(&ys, *step.y.start(), *step.y.end() + 1);
        let zi = index_range(&zs, *step.z.start(), *step.z.end() + 1);

        for x in xi {
            for y in yi.clone() {
                for z in zi.clone() {
                    grid[x][y][z] = step.state;
                }
            }
        }
    }

    let mut total = 0;
    for x in 0..xs.len() - 1 {
        for y in 0..ys.len() - 1 {
            for z in 0..zs.len() - 1 {
                if grid[x][y][z] {
                    total += (xs[x + 1] - xs[x]) as usize
                        * (ys[y + 1] - ys[y]) as usize
                        * (zs[z + 1] - zs[z]) as usize;
                }
            }
        }
    }

    total
}

fn main() -> Result<()> {
    let steps = fs::read_to_string("in/day22.txt")?
        .lines()
        .map(Step::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&steps);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 611_378);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&steps);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 1_214_313_344_725_528);
    };

    Ok(())
}
