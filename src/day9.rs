use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use aoc_2021::Pos;

struct HeightMap {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl std::fmt::Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                writeln!(f)?;
            }

            for x in 0..self.width {
                write!(f, "{}", self.get(Pos::new(x, y)).ok_or(std::fmt::Error)?)?;
            }
        }

        Ok(())
    }
}

impl FromStr for HeightMap {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let height = map.lines().count();
        let width = map
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty height map"))?
            .len();

        let grid = map
            .lines()
            .flat_map(|row| row.chars())
            .map(|char| char.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| anyhow!("invalid height in map"))?
            .into_iter()
            .map(u8::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            grid,
            width,
            height,
        })
    }
}

impl HeightMap {
    fn iter(&self) -> impl Iterator<Item = Pos<usize>> {
        (0..self.width).flat_map(|x| (0..self.height).map(move |y| Pos::new(x, y)))
    }

    fn get(&self, pos: Pos<usize>) -> Option<u8> {
        if pos.x() < self.width && pos.y() < self.height {
            Some(self.grid[(self.width * pos.y()) + pos.x()])
        } else {
            None
        }
    }

    fn adj(&self, pos: Pos<usize>) -> impl Iterator<Item = Pos<usize>> {
        [pos.up(), pos.right(), pos.down(), pos.left()]
            .into_iter()
            .flatten()
            .filter(|&pos| self.get(pos).is_some())
    }

    fn is_low_point(&self, pos: Pos<usize>) -> bool {
        let Some(height) = self.get(pos) else {
            return false;
        };

        self.adj(pos)
            .all(|pos| self.get(pos).is_none_or(|adj| height < adj))
    }

    fn basin_len(&self, low_point: Pos<usize>) -> usize {
        let mut queue = VecDeque::from([low_point]);
        let mut visited = HashSet::from([low_point]);

        while let Some(pos) = queue.pop_front() {
            for adj in self.adj(pos) {
                if self.get(adj).is_some_and(|adj| adj < 9) && visited.insert(adj) {
                    queue.push_back(adj);
                }
            }
        }

        visited.len()
    }
}

fn part1(map: &HeightMap) -> u32 {
    map.iter()
        .filter_map(|pos| map.is_low_point(pos).then(|| map.get(pos)).flatten())
        .map(|height| u32::from(1 + height))
        .sum()
}

fn part2(map: &HeightMap) -> usize {
    let mut lens = map
        .iter()
        .filter(|&pos| map.is_low_point(pos))
        .map(|low_point| map.basin_len(low_point))
        .collect::<BinaryHeap<_>>();

    (0..3).filter_map(|_| lens.pop()).product()
}

fn main() -> Result<()> {
    let map = HeightMap::from_str(&fs::read_to_string("in/day9.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&map);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 502);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&map);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 1_330_560);
    };

    Ok(())
}
