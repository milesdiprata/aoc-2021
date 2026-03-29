use std::fs;
use std::time::Instant;

use anyhow::Result;

fn count_increases(vals: &[u32]) -> usize {
    let mut increases = 0;

    for window in vals.windows(2) {
        let &[prev, next] = window else {
            unreachable!()
        };

        if next > prev {
            increases += 1;
        }
    }

    increases
}

fn part1(depths: &[u32]) -> usize {
    self::count_increases(depths)
}

fn part2(depths: &[u32]) -> usize {
    let sums = depths
        .windows(3)
        .map(|window| window.iter().sum::<u32>())
        .collect::<Vec<_>>();

    self::count_increases(&sums)
}

fn main() -> Result<()> {
    let depths = fs::read_to_string("in/day1.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&depths);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1226);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&depths);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 1_252);
    };

    Ok(())
}
