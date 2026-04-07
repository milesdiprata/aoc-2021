use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Ok;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum PacketType {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    Gt,
    Lt,
    Eq,
}

#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        type_id: PacketType,
        subpackets: Vec<Self>,
    },
}

#[derive(Debug)]
struct BitReader<'a> {
    bits: &'a [u8],
    pos: usize,
}

impl TryFrom<u8> for PacketType {
    type Error = Error;

    fn try_from(type_id: u8) -> Result<Self> {
        match type_id {
            _ if type_id == Self::Sum as u8 => Ok(Self::Sum),
            _ if type_id == Self::Product as u8 => Ok(Self::Product),
            _ if type_id == Self::Min as u8 => Ok(Self::Min),
            _ if type_id == Self::Max as u8 => Ok(Self::Max),
            _ if type_id == Self::Literal as u8 => Ok(Self::Literal),
            _ if type_id == Self::Gt as u8 => Ok(Self::Gt),
            _ if type_id == Self::Lt as u8 => Ok(Self::Lt),
            _ if type_id == Self::Eq as u8 => Ok(Self::Eq),
            _ => bail!("invalid packet type ID '{type_id}'"),
        }
    }
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(hex: &str) -> Result<Self> {
        let bits = hex
            .chars()
            .map(|nibble| {
                nibble
                    .to_digit(16)
                    .ok_or_else(|| anyhow!("invalid hex digit '{nibble}' in packet '{hex}'"))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flat_map(|nibble| (0..4).rev().map(move |i| ((nibble >> i) & 1) as u8))
            .collect::<Vec<_>>();

        BitReader::new(&bits).parse()
    }
}

impl Packet {
    fn version(&self) -> u64 {
        match self {
            &Self::Literal { version, .. } => u64::from(version),
            Self::Operator {
                version,
                subpackets,
                ..
            } => u64::from(*version) + subpackets.iter().map(Self::version).sum::<u64>(),
        }
    }

    fn value(&self) -> u64 {
        match self {
            &Self::Literal { value, .. } => value,
            Self::Operator {
                type_id,
                subpackets,
                ..
            } => match type_id {
                PacketType::Sum => subpackets.iter().map(Self::value).sum::<u64>(),
                PacketType::Product => subpackets.iter().map(Self::value).product::<u64>(),
                PacketType::Min => subpackets.iter().map(Self::value).min().unwrap(),
                PacketType::Max => subpackets.iter().map(Self::value).max().unwrap(),
                PacketType::Gt => u64::from(subpackets[0].value() > subpackets[1].value()),
                PacketType::Lt => u64::from(subpackets[0].value() < subpackets[1].value()),
                PacketType::Eq => u64::from(subpackets[0].value() == subpackets[1].value()),
                PacketType::Literal => unreachable!(),
            },
        }
    }
}

impl<'a> BitReader<'a> {
    const fn new(bits: &'a [u8]) -> Self {
        Self { bits, pos: 0 }
    }

    fn parse(&mut self) -> Result<Packet> {
        #[allow(clippy::cast_possible_truncation)]
        let version = self.read(3) as u8;

        #[allow(clippy::cast_possible_truncation)]
        let type_id = PacketType::try_from(self.read(3) as u8)?;

        if type_id == PacketType::Literal {
            Ok(Packet::Literal {
                version,
                value: self.read_literal(),
            })
        } else {
            Ok(Packet::Operator {
                version,
                type_id,
                subpackets: self.read_subpackets()?,
            })
        }
    }

    fn read(&mut self, n: usize) -> u64 {
        let data = self.bits[self.pos..self.pos + n]
            .iter()
            .fold(0_u64, |acc, &bit| (acc << 1) | u64::from(bit));
        self.pos += n;
        data
    }

    fn read_literal(&mut self) -> u64 {
        let mut value = 0;

        loop {
            let data = self.read(5);
            value = (value << 4) | (data & 0b1111);

            if data >> 4 == 0 {
                break;
            }
        }

        value
    }

    fn read_subpackets(&mut self) -> Result<Vec<Packet>> {
        #[allow(clippy::cast_possible_truncation)]
        let length_type_id = self.read(1) as u8;

        if length_type_id == 0 {
            #[allow(clippy::cast_possible_truncation)]
            let subpacket_len = self.read(15) as usize;

            let mut packets = Vec::new();
            let mut read = 0;

            while read < subpacket_len {
                let mut reader = Self {
                    bits: &self.bits[self.pos..],
                    pos: 0,
                };

                packets.push(reader.parse()?);
                self.pos += reader.pos;
                read += reader.pos;
            }

            Ok(packets)
        } else {
            #[allow(clippy::cast_possible_truncation)]
            let subpackets = self.read(11) as usize;

            let mut packets = Vec::with_capacity(subpackets);

            for _ in 0..subpackets {
                let mut reader = Self {
                    bits: &self.bits[self.pos..],
                    pos: 0,
                };

                packets.push(reader.parse()?);
                self.pos += reader.pos;
            }

            Ok(packets)
        }
    }
}

fn main() -> Result<()> {
    let packet = Packet::from_str(&fs::read_to_string("in/day16.txt")?)?;

    {
        let start = Instant::now();
        let part1 = packet.version();
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 991);
    };

    {
        let start = Instant::now();
        let part2 = packet.value();
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 1_264_485_568_252);
    };

    Ok(())
}
