use std::fmt;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use strum::Display;
use strum::EnumString;

#[derive(Clone, Copy, Debug, Display, EnumString, Eq, Hash, PartialEq)]
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

const STOPS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Hallway([Option<Amphipod>; Self::LEN]);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Room {
    kind: Amphipod,
    slots: [Option<Amphipod>; Self::DEPTH],
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Burrow {
    hallway: Hallway,
    rooms: [Room; 4],
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell = |slot: Option<Amphipod>| slot.map_or_else(|| ".".to_string(), |a| a.to_string());

        writeln!(f, "#############")?;
        write!(f, "#")?;
        for &slot in &self.hallway.0 {
            write!(f, "{}", cell(slot))?;
        }
        writeln!(f, "#")?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            cell(self.rooms[0].slots[0]),
            cell(self.rooms[1].slots[0]),
            cell(self.rooms[2].slots[0]),
            cell(self.rooms[3].slots[0]),
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#",
            cell(self.rooms[0].slots[1]),
            cell(self.rooms[1].slots[1]),
            cell(self.rooms[2].slots[1]),
            cell(self.rooms[3].slots[1]),
        )?;
        write!(f, "  #########")
    }
}

impl FromStr for Burrow {
    type Err = Error;

    fn from_str(grid: &str) -> Result<Self> {
        let lines = grid.lines().map(str::as_bytes).collect::<Vec<_>>();

        let parse_at = |row: usize, col: usize| -> Result<Amphipod> {
            Ok((lines[row][col] as char).to_string().parse()?)
        };

        let room = |kind: Amphipod, col: usize| -> Result<Room> {
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

    const fn target_room(self) -> usize {
        match self {
            Self::Amber => 0,
            Self::Bronze => 1,
            Self::Copper => 2,
            Self::Desert => 3,
        }
    }

    const fn door(self) -> usize {
        2 + 2 * self.target_room()
    }
}

impl Hallway {
    const LEN: usize = 11;

    const fn new() -> Self {
        Self([None; Self::LEN])
    }

    const fn get(&self, i: usize) -> Option<Amphipod> {
        self.0[i]
    }

    const fn set(&mut self, i: usize, value: Option<Amphipod>) {
        self.0[i] = value;
    }

    fn occupants(&self) -> impl Iterator<Item = (usize, Amphipod)> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, slot)| slot.map(|a| (i, a)))
    }

    // True if every hallway cell strictly between `from` and `to`, plus
    // `to` itself, is empty. `from` is excluded so the caller can pass
    // their own current position without tripping on themselves.
    fn path_clear(&self, from: usize, to: usize) -> bool {
        if from == to {
            return true;
        }
        let (lo, hi) = if from < to {
            (from + 1, to)
        } else {
            (to, from - 1)
        };
        (lo..=hi).all(|p| self.0[p].is_none())
    }
}

impl Room {
    const DEPTH: usize = 2;

    const fn new(kind: Amphipod, slots: [Option<Amphipod>; Self::DEPTH]) -> Self {
        Self { kind, slots }
    }
}

impl Burrow {
    fn moves(&self) -> Vec<(Self, u64)> {
        let mut moves = Vec::new();

        // Hallway -> rooms
        // Rooms -> hallway

        moves
    }
}

fn part1() -> u64 {
    todo!()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let burrow = Burrow::from_str(&fs::read_to_string("in/day23.txt")?)?;

    println!("{burrow}");

    {
        let start = Instant::now();
        let part1 = self::part1();
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 0);
    };

    {
        let start = Instant::now();
        let part2 = self::part2();
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 0);
    };

    Ok(())
}
