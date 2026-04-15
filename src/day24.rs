use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use strum::EnumIs;
use strum::EnumString;

#[derive(Clone, Copy, Debug, EnumString)]
enum Reg {
    #[strum(serialize = "w")]
    W,
    #[strum(serialize = "x")]
    X,
    #[strum(serialize = "y")]
    Y,
    #[strum(serialize = "z")]
    Z,
}

#[derive(Clone, Copy, Debug, EnumIs)]
enum Var {
    Reg(Reg),
    Val(i64),
}

#[derive(Clone, Copy, Debug)]
enum Instr {
    Inp(Reg),
    Add((Reg, Var)),
    Mul((Reg, Var)),
    Div((Reg, Var)),
    Mod((Reg, Var)),
    Eql((Reg, Var)),
}

#[derive(Debug)]
struct Program {
    blocks: Vec<Vec<Instr>>,
}

#[derive(Debug, Default)]
struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Var {
    type Err = Error;

    fn from_str(var: &str) -> Result<Self> {
        if let Ok(reg) = Reg::from_str(var) {
            Ok(Self::Reg(reg))
        } else if let Ok(val) = var.parse() {
            Ok(Self::Val(val))
        } else {
            bail!("invalid variable '{var}'")
        }
    }
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(instr: &str) -> Result<Self> {
        if let Some(var) = instr.strip_prefix("inp ") {
            Ok(Self::Inp(var.parse()?))
        } else if let Some(vars) = instr.strip_prefix("add ") {
            let (a, b) = vars
                .split_once(' ')
                .ok_or_else(|| anyhow!("invalid add instruction '{instr}'"))?;
            Ok(Self::Add((a.parse()?, b.parse()?)))
        } else if let Some(vars) = instr.strip_prefix("mul ") {
            let (a, b) = vars
                .split_once(' ')
                .ok_or_else(|| anyhow!("invalid mul instruction '{instr}'"))?;
            Ok(Self::Mul((a.parse()?, b.parse()?)))
        } else if let Some(vars) = instr.strip_prefix("div ") {
            let (a, b) = vars
                .split_once(' ')
                .ok_or_else(|| anyhow!("invalid div instruction '{instr}'"))?;
            Ok(Self::Div((a.parse()?, b.parse()?)))
        } else if let Some(vars) = instr.strip_prefix("mod ") {
            let (a, b) = vars
                .split_once(' ')
                .ok_or_else(|| anyhow!("invalid mod instruction '{instr}'"))?;
            Ok(Self::Mod((a.parse()?, b.parse()?)))
        } else if let Some(vars) = instr.strip_prefix("eql ") {
            let (a, b) = vars
                .split_once(' ')
                .ok_or_else(|| anyhow!("invalid eql instruction '{instr}'"))?;
            Ok(Self::Eql((a.parse()?, b.parse()?)))
        } else {
            bail!("invalid instruction '{instr}'")
        }
    }
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(instrs: &str) -> Result<Self> {
        let mut blocks = Vec::new();

        for instr in instrs.lines().map(Instr::from_str) {
            let instr = instr?;
            if matches!(instr, Instr::Inp(_)) {
                blocks.push(Vec::new());
            }

            blocks
                .last_mut()
                .ok_or_else(|| anyhow!("expected leading 'inp ...' instruction in program"))?
                .push(instr);
        }

        Ok(Self { blocks })
    }
}

impl Alu {
    const fn reg(&self, reg: Reg) -> i64 {
        match reg {
            Reg::W => self.w,
            Reg::X => self.x,
            Reg::Y => self.y,
            Reg::Z => self.z,
        }
    }

    const fn reg_mut(&mut self, reg: Reg) -> &mut i64 {
        match reg {
            Reg::W => &mut self.w,
            Reg::X => &mut self.x,
            Reg::Y => &mut self.y,
            Reg::Z => &mut self.z,
        }
    }

    const fn var(&self, var: Var) -> i64 {
        match var {
            Var::Reg(reg) => self.reg(reg),
            Var::Val(val) => val,
        }
    }

    fn run(&mut self, block: &[Instr], input: i64) -> i64 {
        for &instr in block {
            match instr {
                Instr::Inp(a) => *self.reg_mut(a) = input,
                Instr::Add((a, b)) => *self.reg_mut(a) = self.reg(a) + self.var(b),
                Instr::Mul((a, b)) => *self.reg_mut(a) = self.reg(a) * self.var(b),
                Instr::Div((a, b)) => *self.reg_mut(a) = self.reg(a) / self.var(b),
                Instr::Mod((a, b)) => *self.reg_mut(a) = self.reg(a) % self.var(b),
                Instr::Eql((a, b)) => *self.reg_mut(a) = i64::from(self.reg(a) == self.var(b)),
            }
        }

        self.z
    }
}

impl Program {
    fn find_monad_max(&self) -> u64 {
        const DIGITS: [i64; 9] = [9, 8, 7, 6, 5, 4, 3, 2, 1];
        self.find(DIGITS, 0, 0, &mut HashMap::new())
            .unwrap_or_else(|| unreachable!("no valid MONAD number found"))
    }

    fn find_monad_min(&self) -> u64 {
        const DIGITS: [i64; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        self.find(DIGITS, 0, 0, &mut HashMap::new())
            .unwrap_or_else(|| unreachable!("no valid MONAD number found"))
    }

    fn find(
        &self,
        digits: [i64; 9],
        idx: usize,
        z: i64,
        cache: &mut HashMap<(usize, i64), Option<u64>>,
    ) -> Option<u64> {
        if idx == self.blocks.len() {
            return (z == 0).then_some(0);
        }

        if let Some(&cached) = cache.get(&(idx, z)) {
            return cached;
        }

        let result = digits.iter().find_map(|&digit| {
            let z_next = Alu {
                z,
                ..Default::default()
            }
            .run(&self.blocks[idx], digit);

            let sub = self.find(digits, idx + 1, z_next, cache)?;

            #[allow(clippy::cast_possible_truncation)]
            Some(sub + (digit.cast_unsigned() * 10_u64.pow((self.blocks.len() - idx - 1) as u32)))
        });

        cache.insert((idx, z), result);
        result
    }
}

fn main() -> Result<()> {
    let program = Program::from_str(&fs::read_to_string("in/day24.txt")?)?;

    {
        let start = Instant::now();
        let part1 = program.find_monad_max();
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 94_992_994_195_998);
    };

    {
        let start = Instant::now();
        let part2 = program.find_monad_min();
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 21_191_861_151_161);
    };

    Ok(())
}
