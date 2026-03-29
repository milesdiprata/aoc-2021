use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use strum::EnumString;

#[derive(Clone, Copy, Debug, EnumString)]
enum Dir {
    #[strum(serialize = "forward")]
    Forward,
    #[strum(serialize = "down")]
    Down,
    #[strum(serialize = "up")]
    Up,
}

#[derive(Clone, Copy, Debug)]
struct Cmd {
    dir: Dir,
    units: u8,
}

#[derive(Default, Debug)]
struct Submarine {
    pos: u32,
    depth: u32,
    aim: u32,
}

impl FromStr for Cmd {
    type Err = Error;

    fn from_str(command: &str) -> Result<Self> {
        let (dir, units) = command
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid command '{command}'"))?;

        Ok(Self {
            dir: dir.parse()?,
            units: units.parse()?,
        })
    }
}

impl Submarine {
    fn new() -> Self {
        Self::default()
    }

    fn navigate(&mut self, cmd: Cmd) {
        let units = u32::from(cmd.units);
        match cmd.dir {
            Dir::Forward => self.pos += units,
            Dir::Down => self.depth += units,
            Dir::Up => self.depth = self.depth.saturating_sub(units),
        }
    }

    fn navigate2(&mut self, cmd: Cmd) {
        let units = u32::from(cmd.units);
        match cmd.dir {
            Dir::Forward => {
                self.pos += units;
                self.depth += self.aim * units;
            }
            Dir::Down => self.aim += units,
            Dir::Up => self.aim -= units,
        }
    }
}

fn part1(cmds: &[Cmd]) -> u32 {
    let mut sub = Submarine::new();

    for &cmd in cmds {
        sub.navigate(cmd);
    }

    sub.pos * sub.depth
}

fn part2(cmds: &[Cmd]) -> u32 {
    let mut sub = Submarine::new();

    for &cmd in cmds {
        sub.navigate2(cmd);
    }

    sub.pos * sub.depth
}

fn main() -> Result<()> {
    let cmds = fs::read_to_string("in/day2.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&cmds);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_692_075);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&cmds);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 1_749_524_700);
    };

    Ok(())
}
