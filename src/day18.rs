use std::fs;
use std::iter;
use std::mem;
use std::ops::Add;
use std::ops::AddAssign;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::bail;

#[derive(Clone, Copy, Debug)]
enum Token {
    Open,
    Close,
    Num(u8),
}

#[derive(Clone, Debug, Default)]
struct SnailfishNo {
    tokens: Vec<Token>,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "["),
            Self::Close => write!(f, "]"),
            &Self::Num(num) => write!(f, "{num}"),
        }
    }
}

impl std::fmt::Display for SnailfishNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut comma = false;

        for &token in &self.tokens {
            if comma && matches!(token, Token::Num(_) | Token::Open) {
                write!(f, ",")?;
            }

            write!(f, "{token}")?;
            comma = matches!(token, Token::Num(_) | Token::Close);
        }

        Ok(())
    }
}

impl TryFrom<char> for Token {
    type Error = Error;

    fn try_from(token: char) -> Result<Self> {
        match token {
            '[' => Ok(Self::Open),
            ']' => Ok(Self::Close),
            _ if token.is_numeric() => Ok(Self::Num(token as u8 - b'0')),
            _ => bail!("invalid token '{token}'"),
        }
    }
}

impl FromStr for SnailfishNo {
    type Err = Error;

    fn from_str(num: &str) -> Result<Self> {
        Ok(Self {
            tokens: num
                .chars()
                .filter(|&c| c != ',')
                .map(Token::try_from)
                .collect::<Result<_>>()?,
        })
    }
}

impl Add for SnailfishNo {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            tokens: iter::once(Token::Open)
                .chain(self.tokens)
                .chain(rhs.tokens)
                .chain(iter::once(Token::Close))
                .collect(),
        }
    }
}

impl AddAssign for SnailfishNo {
    fn add_assign(&mut self, rhs: Self) {
        *self = mem::take(self) + rhs;
    }
}

impl SnailfishNo {
    fn magnitude(&self) -> u64 {
        fn magnitude(tokens: &[Token], i: &mut usize) -> u64 {
            match tokens[*i] {
                Token::Num(num) => {
                    *i += 1;
                    u64::from(num)
                }
                Token::Open => {
                    *i += 1; // Consume '['

                    let left = magnitude(tokens, i);
                    let right = magnitude(tokens, i);

                    *i += 1; // Consume ']'

                    (3 * left) + (2 * right)
                }
                Token::Close => unreachable!(),
            }
        }

        magnitude(&self.tokens, &mut 0)
    }

    fn reduce(mut self) -> Self {
        loop {
            if self.explode() || self.split() {
                continue;
            }

            break;
        }

        self
    }

    fn explode(&mut self) -> bool {
        let mut depth = 0;

        for i in 0..self.tokens.len() {
            match self.tokens[i] {
                Token::Open => depth += 1,
                Token::Close => depth -= 1,
                Token::Num(_) => (),
            }

            if depth == 5 {
                // Pair is nested four deep: [Open, Num(a), Num(b), Close]
                let (Token::Num(a), Token::Num(b)) = (self.tokens[i + 1], self.tokens[i + 2])
                else {
                    continue;
                };

                for j in (0..i).rev() {
                    if let Token::Num(num) = &mut self.tokens[j] {
                        *num += a;
                        break;
                    }
                }

                for j in i + 4..self.tokens.len() {
                    if let Token::Num(num) = &mut self.tokens[j] {
                        *num += b;
                        break;
                    }
                }

                self.tokens.splice(i..i + 4, [Token::Num(0)]);

                return true;
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.tokens.len() {
            if let Token::Num(num) = self.tokens[i]
                && num >= 10
            {
                self.tokens.splice(
                    i..=i,
                    [
                        Token::Open,
                        Token::Num(num / 2),
                        Token::Num(num.div_ceil(2)),
                        Token::Close,
                    ],
                );

                return true;
            }
        }

        false
    }
}

fn part1(nums: &[SnailfishNo]) -> u64 {
    let mut result = nums[0].clone();

    for num in nums.iter().skip(1).cloned() {
        result += num;
        result = result.reduce();
    }

    result.magnitude()
}

fn part2(nums: &[SnailfishNo]) -> u64 {
    let mut max = 0;

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i != j {
                let result = (nums[i].clone() + nums[j].clone()).reduce();
                max = max.max(result.magnitude());
            }
        }
    }

    max
}

fn main() -> Result<()> {
    let nums = fs::read_to_string("in/day18.txt")?
        .lines()
        .map(SnailfishNo::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&nums);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 4_184);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&nums);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 4_731);
    };

    Ok(())
}
