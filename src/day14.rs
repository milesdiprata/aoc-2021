use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Clone, Copy)]
struct Rule {
    pair: [u8; 2],
    insert: u8,
}

struct Polymer {
    counts: HashMap<[u8; 2], usize>,
    template_last: u8,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(rule: &str) -> Result<Self> {
        let (pair, insert) = rule
            .split_once(" -> ")
            .ok_or_else(|| anyhow!("invalid rule '{rule}'"))?;

        if pair.len() != 2 {
            bail!("invalid pair '{pair}'");
        }

        if insert.len() != 1 {
            bail!("invalid insertion '{insert}'");
        }

        Ok(Self {
            pair: [pair.as_bytes()[0], pair.as_bytes()[1]],
            insert: insert.as_bytes()[0],
        })
    }
}

impl FromStr for Polymer {
    type Err = Error;

    fn from_str(polymer: &str) -> Result<Self> {
        let mut counts = HashMap::new();

        for window in polymer.as_bytes().windows(2) {
            *counts.entry([window[0], window[1]]).or_default() += 1;
        }

        Ok(Self {
            counts,
            template_last: polymer
                .as_bytes()
                .last()
                .copied()
                .ok_or_else(|| anyhow!("empty template polymer"))?,
        })
    }
}

impl Polymer {
    fn score(&self) -> usize {
        let mut counts = HashMap::new();

        for (&pair, &count) in &self.counts {
            *counts.entry(pair[0]).or_insert(0_usize) += count;
        }

        *counts.entry(self.template_last).or_default() += 1;

        counts.values().copied().max().unwrap() - counts.values().copied().min().unwrap()
    }

    fn step(&mut self, rules: &HashMap<[u8; 2], u8>) {
        let mut next = HashMap::new();

        for (&pair, &count) in &self.counts {
            if let Some(&insert) = rules.get(&pair) {
                *next.entry([pair[0], insert]).or_default() += count;
                *next.entry([insert, pair[1]]).or_default() += count;
            } else {
                *next.entry(pair).or_default() += count;
            }
        }

        self.counts = next;
    }
}

fn parse() -> Result<(Polymer, HashMap<[u8; 2], u8>)> {
    let input = fs::read_to_string("in/day14.txt")?;
    let (polymer, rules) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("invalid input"))?;

    Ok((
        Polymer::from_str(polymer)?,
        rules
            .lines()
            .map(Rule::from_str)
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|rule| (rule.pair, rule.insert))
            .collect(),
    ))
}

fn simulate(polymer: &mut Polymer, rules: &HashMap<[u8; 2], u8>, steps: usize) -> usize {
    for _ in 0..steps {
        polymer.step(rules);
    }

    polymer.score()
}

fn main() -> Result<()> {
    let (mut polymer, rules) = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::simulate(&mut polymer, &rules, 10);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 3_555);
    };

    {
        let start = Instant::now();
        let part2 = self::simulate(&mut polymer, &rules, 40 - 10);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 4_439_442_043_739);
    };

    Ok(())
}
