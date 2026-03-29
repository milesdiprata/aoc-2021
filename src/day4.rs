use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Clone, Debug)]
struct Board {
    grid: Vec<u8>,
    marked: Vec<bool>,
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(grid: &str) -> Result<Self> {
        Ok(Self {
            grid: grid
                .lines()
                .flat_map(str::split_ascii_whitespace)
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
            marked: vec![false; Self::LEN * Self::LEN],
        })
    }
}

impl Board {
    const LEN: usize = 5;

    const fn idx(x: usize, y: usize) -> usize {
        (Self::LEN * y) + x
    }

    fn is_winner(&self) -> bool {
        for x_or_y in 0..Self::LEN {
            if (0..Self::LEN).all(|y| self.marked[Self::idx(x_or_y, y)]) {
                return true;
            }

            if (0..Self::LEN).all(|x| self.marked[Self::idx(x, x_or_y)]) {
                return true;
            }
        }

        false
    }

    fn score(&self, num: u8) -> u32 {
        let mut unmarked = 0;
        for y in 0..Self::LEN {
            for x in 0..Self::LEN {
                if !self.marked[Self::idx(x, y)] {
                    unmarked += u32::from(self.grid[Self::idx(x, y)]);
                }
            }
        }

        u32::from(num) * unmarked
    }

    fn mark(&mut self, num: u8) {
        for y in 0..Self::LEN {
            for x in 0..Self::LEN {
                if self.grid[Self::idx(x, y)] == num {
                    self.marked[Self::idx(x, y)] = true;
                }
            }
        }
    }
}

fn parse() -> Result<(Vec<u8>, Vec<Board>)> {
    let input = fs::read_to_string("in/day4.txt")?;
    let (nums, boards) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("invalid input"))?;

    let nums = nums
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let boards = boards
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<Vec<_>>>()?;

    Ok((nums, boards))
}

fn part1(nums: &[u8], mut boards: Vec<Board>) -> u32 {
    for &num in nums {
        for board in &mut boards {
            board.mark(num);

            if board.is_winner() {
                return board.score(num);
            }
        }
    }

    unreachable!()
}

fn part2(nums: &[u8], mut boards: Vec<Board>) -> u32 {
    for &num in nums {
        for board in &mut boards {
            board.mark(num);
        }

        if boards.len() == 1 && boards[0].is_winner() {
            return boards[0].score(num);
        }

        boards.retain(|board| !board.is_winner());
    }

    unreachable!()
}

fn main() -> Result<()> {
    let (nums, boards) = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&nums, boards.clone());
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 6_592);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&nums, boards);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 31_755);
    };

    Ok(())
}
