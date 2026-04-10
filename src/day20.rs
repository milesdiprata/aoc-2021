use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use strum::EnumString;

type Pos = aoc_2021::Pos<i64>;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Default, EnumString)]
enum Pixel {
    #[default]
    #[strum(serialize = ".")]
    Dark,

    #[strum(serialize = "#")]
    Light,
}

#[derive(Debug)]
struct Enhancement {
    pixels: Vec<Pixel>,
}

#[derive(Clone, Debug)]
struct Image {
    pixels: HashMap<Pos, Pixel>,
    default: Pixel,
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dark => write!(f, "."),
            Self::Light => write!(f, "#"),
        }
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_min, x_max) = (self.x_min(), self.x_max());
        let (y_min, y_max) = (self.y_min(), self.y_max());

        for y in y_min..=y_max {
            if y > y_min {
                writeln!(f)?;
            }

            for x in x_min..=x_max {
                let pixel = self.get(Pos::new(x, y));
                write!(f, "{pixel}")?;
            }
        }

        Ok(())
    }
}

impl FromStr for Enhancement {
    type Err = Error;

    fn from_str(pixels: &str) -> Result<Self> {
        Ok(Self {
            pixels: pixels
                .chars()
                .map(|c| c.to_string().parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl FromStr for Image {
    type Err = Error;

    fn from_str(image: &str) -> Result<Self> {
        let mut pixels = HashMap::new();

        for (y, row) in image.lines().enumerate() {
            for (x, pixel) in row.char_indices() {
                let x = i64::try_from(x)?;
                let y = i64::try_from(y)?;
                let pixel = pixel.to_string().parse()?;
                pixels.insert(Pos::new(x, y), pixel);
            }
        }

        Ok(Self {
            pixels,
            default: Pixel::default(),
        })
    }
}

impl Image {
    fn x_min(&self) -> i64 {
        self.pixels
            .keys()
            .copied()
            .map(Pos::x)
            .min()
            .unwrap_or_default()
    }

    fn x_max(&self) -> i64 {
        self.pixels
            .keys()
            .copied()
            .map(Pos::x)
            .max()
            .unwrap_or_default()
    }

    fn y_min(&self) -> i64 {
        self.pixels
            .keys()
            .copied()
            .map(Pos::y)
            .min()
            .unwrap_or_default()
    }

    fn y_max(&self) -> i64 {
        self.pixels
            .keys()
            .copied()
            .map(Pos::y)
            .max()
            .unwrap_or_default()
    }

    fn get(&self, pos: Pos) -> Pixel {
        self.pixels.get(&pos).copied().unwrap_or(self.default)
    }

    fn consideration_pixels(&self, pos: Pos) -> [Pixel; 9] {
        [
            pos.up().left(),
            pos.up(),
            pos.up().right(),
            pos.left(),
            pos,
            pos.right(),
            pos.down().left(),
            pos.down(),
            pos.down().right(),
        ]
        .map(|pixel| self.get(pixel))
    }

    fn enhance(&mut self, enhancement: &Enhancement) {
        let (x_min, x_max) = (self.x_min(), self.x_max());
        let (y_min, y_max) = (self.y_min(), self.y_max());

        let mut next = HashMap::new();

        for y in y_min - 1..=y_max + 1 {
            for x in x_min - 1..=x_max + 1 {
                let pos = Pos::new(x, y);
                let binary = self
                    .consideration_pixels(pos)
                    .map(|pixel| pixel as usize)
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (i, &pixel)| acc | pixel << (8 - i));

                next.insert(pos, enhancement.pixels[binary]);
            }
        }

        self.pixels = next;
        self.default = match self.default {
            Pixel::Dark => enhancement.pixels.first().copied().unwrap(),
            Pixel::Light => enhancement.pixels.last().copied().unwrap(),
        };
    }
}

fn parse() -> Result<(Enhancement, Image)> {
    let input = fs::read_to_string("in/day20.txt")?;
    let (enhancement, image) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("invalid input"))?;

    Ok((enhancement.parse()?, image.parse()?))
}

fn part1(enhancement: &Enhancement, image: &mut Image) -> usize {
    for _ in 0..2 {
        image.enhance(enhancement);
    }

    image
        .pixels
        .values()
        .filter(|&&pixel| matches!(pixel, Pixel::Light))
        .count()
}

fn part2(enhancement: &Enhancement, image: &mut Image) -> usize {
    for _ in 0..50 - 2 {
        image.enhance(enhancement);
    }

    image
        .pixels
        .values()
        .filter(|&&pixel| matches!(pixel, Pixel::Light))
        .count()
}

fn main() -> Result<()> {
    let (enhancement, mut image) = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&enhancement, &mut image);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 5_571);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&enhancement, &mut image);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 17_965);
    };

    Ok(())
}
