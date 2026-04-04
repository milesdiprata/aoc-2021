use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

type Pos = aoc_2021::Pos<usize>;

struct EnergyLevels {
    grid: [u8; Self::LEN * Self::LEN],
}

impl FromStr for EnergyLevels {
    type Err = Error;

    #[allow(clippy::cast_possible_truncation)]
    fn from_str(grid: &str) -> Result<Self> {
        Ok(Self {
            grid: grid
                .lines()
                .flat_map(|row| row.chars())
                .map(|energy| energy.to_digit(10).map(|energy| energy as u8))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| anyhow!("invalid energy level in grid"))?
                .try_into()
                .map_err(|_| anyhow!("expected 10x10 grid"))?,
        })
    }
}

impl EnergyLevels {
    const LEN: usize = 10;

    fn iter() -> impl Iterator<Item = Pos> {
        (0..Self::LEN).flat_map(|x| (0..Self::LEN).map(move |y| Pos::new(x, y)))
    }

    fn adj(pos: Pos) -> impl Iterator<Item = Pos> {
        [
            pos.up(),
            pos.up().and_then(Pos::right),
            pos.right(),
            pos.right().and_then(Pos::down),
            pos.down(),
            pos.down().and_then(Pos::left),
            pos.left(),
            pos.left().and_then(Pos::up),
        ]
        .into_iter()
        .flatten()
        .filter(|pos| pos.x() < Self::LEN && pos.y() < Self::LEN)
    }

    const fn get(&self, pos: Pos) -> Option<u8> {
        if pos.x() < Self::LEN && pos.y() < Self::LEN {
            Some(self.grid[(Self::LEN * pos.y()) + pos.x()])
        } else {
            None
        }
    }

    const fn get_mut(&mut self, pos: Pos) -> Option<&mut u8> {
        if pos.x() < Self::LEN && pos.y() < Self::LEN {
            Some(&mut self.grid[(Self::LEN * pos.y()) + pos.x()])
        } else {
            None
        }
    }

    fn step(&mut self) -> usize {
        for energy in &mut self.grid {
            *energy += 1;
        }

        loop {
            let mut any_flashed = false;

            for pos in Self::iter() {
                if self.get(pos).is_some_and(|energy| energy > 9) {
                    *self.get_mut(pos).unwrap() = 0;
                    any_flashed = true;

                    for adj in Self::adj(pos) {
                        if let Some(energy) = self.get_mut(adj)
                            && *energy > 0
                        {
                            *energy += 1;
                        }
                    }
                }
            }

            if !any_flashed {
                break;
            }
        }

        self.grid.into_iter().filter(|&energy| energy == 0).count()
    }
}

fn part1(energies: &mut EnergyLevels) -> usize {
    (0..100).map(|_| energies.step()).sum()
}

fn part2(energies: &mut EnergyLevels) -> usize {
    for step in 100.. {
        if energies.step() == energies.grid.len() {
            return step + 1;
        }
    }

    unreachable!()
}

fn main() -> Result<()> {
    let mut energies = EnergyLevels::from_str(&fs::read_to_string("in/day11.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&mut energies);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_601);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&mut energies);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 368);
    };

    Ok(())
}
