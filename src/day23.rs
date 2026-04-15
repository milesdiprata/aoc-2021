use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use strum::Display;
use strum::EnumString;

const ROOMS: usize = 4;

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString)]
enum Amphipod {
    #[strum(serialize = "A")]
    Amber,
    #[strum(serialize = "B")]
    Bronze,
    #[strum(serialize = "C")]
    Copper,
    #[strum(serialize = "D")]
    Desert,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hallway {
    slots: [Option<Amphipod>; Self::LEN],
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Room<const N: usize> {
    kind: Amphipod,
    slots: [Option<Amphipod>; N],
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Burrow<const N: usize> {
    hallway: Hallway,
    rooms: [Room<N>; ROOMS],
}

impl<const N: usize> fmt::Display for Burrow<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell = |slot: Option<Amphipod>| slot.map_or_else(|| ".".to_string(), |a| a.to_string());

        writeln!(f, "#############")?;
        write!(f, "#")?;
        for &slot in &self.hallway.slots {
            write!(f, "{}", cell(slot))?;
        }
        writeln!(f, "#")?;

        for depth in 0..N {
            let (prefix, suffix) = if depth == 0 {
                ("###", "###")
            } else {
                ("  #", "#")
            };
            writeln!(
                f,
                "{prefix}{}#{}#{}#{}{suffix}",
                cell(self.rooms[0].slots[depth]),
                cell(self.rooms[1].slots[depth]),
                cell(self.rooms[2].slots[depth]),
                cell(self.rooms[3].slots[depth]),
            )?;
        }

        write!(f, "  #########")
    }
}

impl FromStr for Burrow<2> {
    type Err = Error;

    fn from_str(grid: &str) -> Result<Self> {
        let lines = grid.lines().map(str::as_bytes).collect::<Vec<_>>();

        let parse_at = |row: usize, col: usize| -> Result<Amphipod> {
            Ok((lines[row][col] as char).to_string().parse()?)
        };

        let room = |kind: Amphipod, col: usize| -> Result<Room<2>> {
            Ok(Room::new(
                kind,
                [Some(parse_at(2, col)?), Some(parse_at(3, col)?)],
            ))
        };

        Ok(Self {
            hallway: Hallway::new(),
            rooms: [
                room(Amphipod::Amber, 3)?,
                room(Amphipod::Bronze, 5)?,
                room(Amphipod::Copper, 7)?,
                room(Amphipod::Desert, 9)?,
            ],
        })
    }
}

impl Amphipod {
    const fn energy(self) -> u64 {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1_000,
        }
    }

    const fn dest_room(self) -> usize {
        match self {
            Self::Amber => 0,
            Self::Bronze => 1,
            Self::Copper => 2,
            Self::Desert => 3,
        }
    }

    const fn door(self) -> usize {
        2 + (2 * self.dest_room())
    }
}

impl Hallway {
    const LEN: usize = 11;
    const STOPS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

    const fn new() -> Self {
        Self {
            slots: [None; Self::LEN],
        }
    }

    fn occupants(&self) -> impl Iterator<Item = (usize, Amphipod)> {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, &slot)| slot.map(|amph| (i, amph)))
    }

    fn is_path_clear(&self, from: usize, to: usize) -> bool {
        if from == to {
            return true;
        }

        let (lo, hi) = if from < to {
            (from + 1, to)
        } else {
            (to, from - 1)
        };

        (lo..=hi).all(|i| self.slots[i].is_none())
    }
}

impl<const N: usize> Room<N> {
    const fn new(kind: Amphipod, slots: [Option<Amphipod>; N]) -> Self {
        Self { kind, slots }
    }

    const fn door(&self) -> usize {
        self.kind.door()
    }

    fn top(&self) -> Option<(usize, Amphipod)> {
        self.slots
            .iter()
            .enumerate()
            .find_map(|(i, &slot)| slot.map(|amph| (i, amph)))
    }

    fn dest(&self) -> Option<usize> {
        self.is_settled()
            .then(|| self.slots.iter().rposition(Option::is_none))
            .flatten()
    }

    fn is_settled(&self) -> bool {
        self.slots
            .iter()
            .all(|&slot| slot.is_none_or(|amph| amph == self.kind))
    }

    fn is_complete(&self) -> bool {
        self.slots.iter().all(|&slot| slot == Some(self.kind))
    }
}

impl<const N: usize> Burrow<N> {
    fn min_energy_to_complete(&self) -> u64 {
        let mut best = HashMap::new();
        let mut frontier = BinaryHeap::from([Reverse((0, self.clone()))]);

        while let Some(Reverse((cost, burrow))) = frontier.pop() {
            if burrow.is_complete() {
                return cost;
            }

            if best.get(&burrow).is_some_and(|&best| cost > best) {
                continue;
            }

            for (next, cost_next) in burrow.moves() {
                let cost_new = cost + cost_next;
                if best.get(&next).is_none_or(|&best| cost_new < best) {
                    best.insert(next.clone(), cost_new);
                    frontier.push(Reverse((cost_new, next)));
                }
            }
        }

        unreachable!("no path to goal")
    }

    fn is_complete(&self) -> bool {
        self.rooms.iter().all(Room::is_complete)
    }

    fn moves(&self) -> Vec<(Self, u64)> {
        let mut moves = Vec::new();

        // Hallway -> rooms
        for (pos, amph) in self.hallway.occupants() {
            let room = &self.rooms[amph.dest_room()];
            let door = amph.door();

            if !self.hallway.is_path_clear(pos, door) {
                continue;
            }

            let Some(depth) = room.dest() else {
                continue;
            };

            let steps = pos.abs_diff(door) + depth + 1;
            let cost = steps as u64 * amph.energy();

            let mut next = self.clone();
            next.hallway.slots[pos] = None;
            next.rooms[amph.dest_room()].slots[depth] = Some(amph);

            // Any such move is on an optimal path; if found, skip generating
            // room -> hallway moves entirely
            return vec![(next, cost)];
        }

        // Rooms -> hallway
        for (i, room) in self.rooms.iter().enumerate() {
            if room.is_settled() {
                continue;
            }

            let Some((depth, amph)) = room.top() else {
                continue;
            };

            for stop in Hallway::STOPS {
                if !self.hallway.is_path_clear(room.door(), stop) {
                    continue;
                }

                let steps = depth + 1 + room.door().abs_diff(stop);
                let cost = steps as u64 * amph.energy();

                let mut next = self.clone();
                next.rooms[i].slots[depth] = None;
                next.hallway.slots[stop] = Some(amph);

                moves.push((next, cost));
            }
        }

        moves
    }
}

impl Burrow<2> {
    fn unfold(self) -> Burrow<4> {
        const INSERT: [[Amphipod; ROOMS]; 2] = [
            [
                Amphipod::Desert,
                Amphipod::Copper,
                Amphipod::Bronze,
                Amphipod::Amber,
            ],
            [
                Amphipod::Desert,
                Amphipod::Bronze,
                Amphipod::Amber,
                Amphipod::Copper,
            ],
        ];

        Burrow {
            hallway: self.hallway,
            rooms: std::array::from_fn(|i| Room {
                kind: self.rooms[i].kind,
                slots: [
                    self.rooms[i].slots[0],
                    Some(INSERT[0][i]),
                    Some(INSERT[1][i]),
                    self.rooms[i].slots[1],
                ],
            }),
        }
    }
}

fn part1(burrow: &Burrow<2>) -> u64 {
    burrow.min_energy_to_complete()
}

fn part2(burrow: &Burrow<4>) -> u64 {
    burrow.min_energy_to_complete()
}

fn main() -> Result<()> {
    let burrow = Burrow::from_str(&fs::read_to_string("in/day23.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&burrow);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 14_467);
    };

    {
        let burrow = burrow.unfold();
        let start = Instant::now();
        let part2 = self::part2(&burrow);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 48_759);
    };

    Ok(())
}
