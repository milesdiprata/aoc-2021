use std::fs;
use std::time::Instant;

use anyhow::Result;

#[derive(Debug)]
enum LineError {
    Corrupt(char),
    Incomplete(Vec<char>),
}

impl LineError {
    const fn is_corrupt(&self) -> bool {
        matches!(self, Self::Corrupt(_))
    }

    const fn is_incomplete(&self) -> bool {
        matches!(self, Self::Incomplete(_))
    }

    fn score(&self) -> u64 {
        match self {
            &Self::Corrupt(illegal) => match illegal {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            },
            Self::Incomplete(seq) => seq.iter().fold(0, |score, &token| {
                (score * 5)
                    + match token {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    }
            }),
        }
    }
}

fn parse_line(line: &str) -> Result<(), LineError> {
    let mut stack = Vec::new();

    for token in line.chars() {
        match token {
            '(' | '[' | '{' | '<' => stack.push(token),
            closing => {
                let expected = match closing {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    _ => continue,
                };

                match stack.pop() {
                    Some(open) if open == expected => {}
                    _ => return Err(LineError::Corrupt(closing)),
                }
            }
        }
    }

    if stack.is_empty() {
        return Ok(());
    }

    stack.reverse();
    Err(LineError::Incomplete(stack))
}

fn part1(results: &[LineError]) -> u64 {
    results
        .iter()
        .filter(|&err| err.is_corrupt())
        .map(LineError::score)
        .sum()
}

fn part2(results: &[LineError]) -> u64 {
    let mut scores = results
        .iter()
        .filter(|&err| err.is_incomplete())
        .map(LineError::score)
        .collect::<Vec<_>>();

    assert!(scores.len() % 2 != 0);

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day10.txt")?;
    let errs = input
        .lines()
        .map(self::parse_line)
        .filter_map(Result::err)
        .collect::<Vec<_>>();

    {
        let start = Instant::now();
        let part1 = self::part1(&errs);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 389_589);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&errs);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 1_190_420_163);
    };

    Ok(())
}
