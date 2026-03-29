use std::fs;
use std::time::Instant;

use anyhow::Result;

fn most_common_bit(lines: &[&str], i: usize) -> u8 {
    let ones = lines
        .iter()
        .filter(|&&lines| lines.as_bytes()[i] == b'1')
        .count();

    if 2 * ones >= lines.len() { b'1' } else { b'0' }
}

fn part1(lines: &[&str]) -> u32 {
    let len = lines[0].len();
    let gamma = (0..len).fold(0u32, |acc, i| {
        if self::most_common_bit(lines, i) == b'1' {
            acc | (1 << (len - 1 - i))
        } else {
            acc
        }
    });

    let epsilon = !gamma & ((1 << len) - 1);

    gamma * epsilon
}

fn part2(lines: &[&str]) -> u32 {
    fn filter_ratings(mut candidates: Vec<&str>, most_common: bool) -> u32 {
        let len = candidates[0].len();

        for i in 0..len {
            if candidates.len() == 1 {
                break;
            }

            let target = self::most_common_bit(&candidates, i);
            let target = if most_common { target } else { target ^ 1 };
            candidates.retain(|&line| line.as_bytes()[i] == target);
        }

        u32::from_str_radix(candidates[0], 2).unwrap()
    }

    filter_ratings(lines.to_vec(), true) * filter_ratings(lines.to_vec(), false)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day3.txt")?;
    let lines = input.lines().collect::<Vec<_>>();

    {
        let start = Instant::now();
        let part1 = self::part1(&lines);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 3_885_894);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&lines);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 4_375_225);
    };

    Ok(())
}
