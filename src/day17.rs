use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
struct Target {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

#[derive(Debug, Default)]
struct Probe {
    x: i64,
    y: i64,
    vel_x: i64,
    vel_y: i64,
}

impl FromStr for Target {
    type Err = Error;

    fn from_str(target: &str) -> Result<Self> {
        fn parse_range(range: &str) -> Result<RangeInclusive<i64>> {
            let (lhs, rhs) = range
                .split_once("..")
                .ok_or_else(|| anyhow!("invalid range '{range}'"))?;

            let lhs = lhs.parse::<i64>()?;
            let rhs = rhs.parse::<i64>()?;

            Ok(lhs..=rhs)
        }

        let err = || anyhow!("invalid target '{target}'");
        let target = target.strip_prefix("target area: ").ok_or_else(err)?;
        let (x, y) = target.split_once(", ").ok_or_else(err)?;
        let x = x.strip_prefix("x=").ok_or_else(err)?;
        let y = y.strip_prefix("y=").ok_or_else(err)?;

        Ok(Self {
            x: parse_range(x)?,
            y: parse_range(y)?,
        })
    }
}

impl Probe {
    const fn new(vel_x: i64, vel_y: i64) -> Self {
        Self {
            x: 0,
            y: 0,
            vel_x,
            vel_y,
        }
    }

    fn simulate(&mut self, target: &Target) -> bool {
        while !self.is_past_target(target) {
            if self.is_in_target(target) {
                return true;
            }

            self.step();
        }

        false
    }

    fn is_in_target(&self, target: &Target) -> bool {
        target.x.contains(&self.x) && target.y.contains(&self.y)
    }

    const fn is_past_target(&self, target: &Target) -> bool {
        self.x > *target.x.end() || self.y < *target.y.start()
    }

    const fn step(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;

        if self.vel_x > 0 {
            self.vel_x -= 1;
        } else if self.vel_x < 0 {
            self.vel_x += 1;
        }

        self.vel_y -= 1;
    }
}

fn part1(target: &Target) -> i64 {
    // The probe's y trajectory is symmetric: when it returns to y=0 on the way
    // down its velocity is -(vel_y + 1). To still land in the target on the
    // next step, that must be >= target.y.start(), giving a maximum initial
    // vel_y of -target.y.start() - 1. The peak height is the triangular number
    // vel_y * (vel_y + 1) / 2.
    let vel_y = -target.y.start() - 1;
    (vel_y * (vel_y + 1)) / 2
}

fn part2(target: &Target) -> usize {
    let mut count = 0;

    for vel_x in 1..=*target.x.end() {
        for vel_y in *target.y.start()..-*target.y.start() {
            if Probe::new(vel_x, vel_y).simulate(target) {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<()> {
    let target = Target::from_str(&fs::read_to_string("in/day17.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&target);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 4_950);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&target);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 1_477);
    };

    Ok(())
}
