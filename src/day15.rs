use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

type Pos = aoc_2021::Pos<usize>;

struct Cave {
    risk: Vec<u8>,
    width: usize,
    height: usize,
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(grid: &str) -> Result<Self> {
        let height = grid.lines().count();
        let width = grid
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty cave"))?
            .len();

        #[allow(clippy::cast_possible_truncation)]
        let risk = grid
            .lines()
            .flat_map(|row| row.chars())
            .map(|risk| risk.to_digit(10).map(|risk| risk as u8))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| anyhow!("invalid risk level in cave"))?;

        Ok(Self {
            risk,
            width,
            height,
        })
    }
}

impl Cave {
    fn extend(&self, times: usize) -> Self {
        let width = self.width * times;
        let height = self.height * times;
        let mut risk = vec![0; width * height];

        for times_y in 0..times {
            for times_x in 0..times {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let x_extended = (times_x * self.width) + x;
                        let y_extended = (times_y * self.height) + y;

                        let base = usize::from(self.get(Pos::new(x, y)).unwrap());
                        #[allow(clippy::cast_possible_truncation)]
                        let new = (((base - 1 + times_y + times_x) % 9) + 1) as u8;

                        risk[(width * y_extended) + x_extended] = new;
                    }
                }
            }
        }

        Self {
            risk,
            width,
            height,
        }
    }

    const fn idx(&self, pos: Pos) -> Option<usize> {
        if pos.x() < self.width && pos.y() < self.height {
            Some((self.width * pos.y()) + pos.x())
        } else {
            None
        }
    }

    fn get(&self, pos: Pos) -> Option<u8> {
        Some(self.risk[self.idx(pos)?])
    }

    fn adj(&self, pos: Pos) -> impl Iterator<Item = (Pos, u8)> {
        [pos.up(), pos.right(), pos.down(), pos.left()]
            .into_iter()
            .flatten()
            .filter_map(|pos| self.get(pos).map(|risk| (pos, risk)))
    }

    fn min_risk(&self) -> u64 {
        let start = Pos::new(0, 0);
        let end = Pos::new(self.width - 1, self.height - 1);

        let mut frontier = BinaryHeap::new();
        let mut cost = vec![u64::MAX; self.width * self.height];

        frontier.push(Reverse((0_u64, start)));
        cost[self.idx(start).unwrap()] = 0;

        while let Some(Reverse((c, pos))) = frontier.pop() {
            if c > cost[self.idx(pos).unwrap()] {
                continue;
            }

            if pos == end {
                return cost[self.idx(end).unwrap()];
            }

            for (adj, risk) in self.adj(pos) {
                let cost_adj = cost[self.idx(pos).unwrap()] + u64::from(risk);
                if cost_adj < cost[self.idx(adj).unwrap()] {
                    cost[self.idx(adj).unwrap()] = cost_adj;
                    frontier.push(Reverse((cost_adj, adj)));
                }
            }
        }

        unreachable!()
    }
}

fn part1(cave: &Cave) -> u64 {
    cave.min_risk()
}

fn part2(cave: &Cave) -> u64 {
    cave.extend(5).min_risk()
}

fn main() -> Result<()> {
    let cave = Cave::from_str(&fs::read_to_string("in/day15.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&cave);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 755);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&cave);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 3_016);
    };

    Ok(())
}
