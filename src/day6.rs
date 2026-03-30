use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Lanternfish {
    timer: u32,
}

impl FromStr for Lanternfish {
    type Err = Error;

    fn from_str(fish: &str) -> Result<Self> {
        Ok(Self {
            timer: fish.parse()?,
        })
    }
}

impl Lanternfish {
    const fn new() -> Self {
        Self { timer: 8 }
    }

    fn tick(mut self) -> Vec<Self> {
        if self.timer == 0 {
            self.timer = 6;
            vec![self, Self::new()]
        } else {
            self.timer = self.timer.saturating_sub(1);
            vec![self]
        }
    }
}

fn simulate(fish: &[Lanternfish], days: usize) -> usize {
    let mut counts = HashMap::new();
    for &fish in fish {
        *counts.entry(fish).or_insert(0_usize) += 1;
    }

    for _ in 0..days {
        let mut next = HashMap::new();
        for (fish, count) in counts {
            for fish in fish.tick() {
                *next.entry(fish).or_insert(0_usize) += count;
            }
        }

        counts = next;
    }

    counts.into_values().sum()
}

fn main() -> Result<()> {
    let fish = fs::read_to_string("in/day6.txt")?
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::simulate(&fish, 80);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 362_666);
    };

    {
        let start = Instant::now();
        let part2 = self::simulate(&fish, 256);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 1_640_526_601_595);
    };

    Ok(())
}
