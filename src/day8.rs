use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use strum::EnumString;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumString)]
enum Segment {
    #[strum(serialize = "a")]
    A,
    #[strum(serialize = "b")]
    B,
    #[strum(serialize = "c")]
    C,
    #[strum(serialize = "d")]
    D,
    #[strum(serialize = "e")]
    E,
    #[strum(serialize = "f")]
    F,
    #[strum(serialize = "g")]
    G,
}

#[derive(Debug)]
struct Entry {
    patterns: [BTreeSet<Segment>; 10],
    outputs: [BTreeSet<Segment>; 4],
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(entry: &str) -> Result<Self> {
        fn parse_segments(segments: &str) -> Result<BTreeSet<Segment>> {
            segments
                .chars()
                .map(|segment| segment.to_string().parse().map_err(Error::from))
                .collect()
        }

        let (patterns_str, outputs_str) = entry
            .split_once(" | ")
            .ok_or_else(|| anyhow!("missing ' | ' separator"))?;

        let patterns = patterns_str
            .split_whitespace()
            .map(parse_segments)
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| anyhow!("expected exactly 10 patterns"))?;

        let outputs = outputs_str
            .split_whitespace()
            .map(parse_segments)
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| anyhow!("expected exactly 4 outputs"))?;

        Ok(Self { patterns, outputs })
    }
}

impl Entry {
    fn decode(&self) -> u64 {
        let digit_1 = self
            .patterns
            .iter()
            .find(|&pattern| pattern.len() == 2)
            .unwrap();
        let digit_7 = self
            .patterns
            .iter()
            .find(|&pattern| pattern.len() == 3)
            .unwrap();
        let digit_4 = self
            .patterns
            .iter()
            .find(|&pattern| pattern.len() == 4)
            .unwrap();
        let digit_8 = self
            .patterns
            .iter()
            .find(|&pattern| pattern.len() == 7)
            .unwrap();

        let digit_3 = self
            .patterns
            .iter()
            .find(|&pattern| pattern.len() == 5 && pattern.is_superset(digit_1))
            .unwrap();
        let digit_9 = self
            .patterns
            .iter()
            .find(|&pattern| pattern.len() == 6 && pattern.is_superset(digit_4))
            .unwrap();
        let digit_0 = self
            .patterns
            .iter()
            .find(|&pattern| {
                pattern.len() == 6 && pattern.is_superset(digit_1) && pattern != digit_9
            })
            .unwrap();
        let digit_6 = self
            .patterns
            .iter()
            .find(|&pattern| pattern.len() == 6 && pattern != digit_9 && pattern != digit_0)
            .unwrap();
        let digit_5 = self
            .patterns
            .iter()
            .find(|&pattern| pattern.len() == 5 && pattern.is_subset(digit_6))
            .unwrap();
        let digit_2 = self
            .patterns
            .iter()
            .find(|&pattern| pattern.len() == 5 && pattern != digit_3 && pattern != digit_5)
            .unwrap();

        let nums = BTreeMap::from([
            (digit_1, 1),
            (digit_7, 7),
            (digit_4, 4),
            (digit_8, 8),
            (digit_3, 3),
            (digit_9, 9),
            (digit_0, 0),
            (digit_6, 6),
            (digit_5, 5),
            (digit_2, 2),
        ]);

        self.outputs
            .iter()
            .zip([1000, 100, 10, 1])
            .map(|(output, scale)| nums[output] * scale)
            .sum()
    }
}

fn part1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .flat_map(|entry| entry.outputs.iter())
        .filter(|output| matches!(output.len(), 2 | 3 | 4 | 7))
        .count()
}

fn part2(entries: &[Entry]) -> u64 {
    entries.iter().map(Entry::decode).sum()
}

fn main() -> Result<()> {
    let entries = fs::read_to_string("in/day8.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&entries);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 449);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&entries);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 968_175);
    };

    Ok(())
}
