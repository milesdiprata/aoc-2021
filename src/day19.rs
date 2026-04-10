use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Coord>,
}

impl FromStr for Coord {
    type Err = Error;

    fn from_str(coord: &str) -> Result<Self> {
        let [x, y, z] = coord
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|coords: Vec<_>| anyhow!("found {} coords, expected three", coords.len()))?;

        Ok(Self { x, y, z })
    }
}

impl FromStr for Scanner {
    type Err = Error;

    fn from_str(scanner: &str) -> Result<Self> {
        Ok(Self {
            beacons: scanner
                .lines()
                .skip(1)
                .map(str::parse)
                .collect::<Result<_>>()?,
        })
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Coord {
    const fn rotations(self) -> [Self; 24] {
        let Self { x, y, z } = self;
        [
            // Facing +x
            Self { x, y, z },
            Self { x, y: -z, z: y },
            Self { x, y: -y, z: -z },
            Self { x, y: z, z: -y },
            // Facing -x
            Self { x: -x, y: -y, z },
            Self { x: -x, y: z, z: y },
            Self { x: -x, y, z: -z },
            Self {
                x: -x,
                y: -z,
                z: -y,
            },
            // Facing +y
            Self { x: y, y: z, z: x },
            Self { x: y, y: -x, z },
            Self { x: y, y: -z, z: -x },
            Self { x: y, y: x, z: -z },
            // Facing -y
            Self { x: -y, y: -z, z: x },
            Self { x: -y, y: x, z },
            Self { x: -y, y: z, z: -x },
            Self {
                x: -y,
                y: -x,
                z: -z,
            },
            // Facing +z
            Self { x: z, y: x, z: y },
            Self { x: z, y: -y, z: x },
            Self { x: z, y: -x, z: -y },
            Self { x: z, y, z: -x },
            // Facing -z
            Self { x: -z, y: -x, z: y },
            Self { x: -z, y, z: x },
            Self { x: -z, y: x, z: -y },
            Self {
                x: -z,
                y: -y,
                z: -x,
            },
        ]
    }

    const fn manhattan(self, other: Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Scanner {
    fn rotation(&self, rotation_idx: usize) -> Vec<Coord> {
        self.beacons
            .iter()
            .map(|beacon| beacon.rotations()[rotation_idx])
            .collect()
    }
}

fn solve(scanners: &[Scanner]) -> (usize, i64) {
    fn find_match(known: &HashSet<Coord>, rotated_sets: &[Vec<Coord>]) -> Option<(Coord, usize)> {
        for (rot, rotated) in rotated_sets.iter().enumerate() {
            let mut offsets = HashMap::new();

            for &known in known {
                for &rotated in rotated {
                    *offsets.entry(known - rotated).or_insert(0_usize) += 1;
                }
            }

            if let Some((&offset, _)) = offsets.iter().find(|&(_, &count)| count >= 12) {
                return Some((offset, rot));
            }
        }

        None
    }

    let rotated = scanners
        .iter()
        .map(|scanner| (0..24).map(|rot| scanner.rotation(rot)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut known = scanners[0].beacons.iter().copied().collect::<HashSet<_>>();
    let mut positions = vec![Coord { x: 0, y: 0, z: 0 }];
    let mut remaining = (1..scanners.len()).collect::<Vec<_>>();

    while !remaining.is_empty() {
        let remaining_idx = remaining
            .iter()
            .position(|&scanner_idx| {
                if let Some((offset, rotation_idx)) = find_match(&known, &rotated[scanner_idx]) {
                    known.extend(
                        rotated[scanner_idx][rotation_idx]
                            .iter()
                            .map(|&b| b + offset),
                    );
                    positions.push(offset);
                    true
                } else {
                    false
                }
            })
            .expect("no scanner matched");

        remaining.remove(remaining_idx);
    }

    let part1 = known.len();
    let part2 = positions
        .iter()
        .flat_map(|&a| positions.iter().map(move |&b| a.manhattan(b)))
        .max()
        .unwrap_or_default();

    (part1, part2)
}

fn main() -> Result<()> {
    let scanners = fs::read_to_string("in/day19.txt")?
        .split("\n\n")
        .map(Scanner::from_str)
        .collect::<Result<Vec<_>>>()?;

    let start = Instant::now();
    let (part1, part2) = self::solve(&scanners);
    let elapsed = start.elapsed();

    println!("Part 1: {part1} ({elapsed:?})");
    println!("Part 2: {part2} ({elapsed:?})");

    assert_eq!(part1, 451);
    assert_eq!(part2, 13_184);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn all_rotations() {
        let coord = Coord { x: 1, y: 2, z: 3 };
        let rotations = HashSet::from(coord.rotations());
        assert_eq!(rotations.len(), 24);
    }
}
