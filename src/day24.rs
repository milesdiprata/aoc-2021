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
struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    input: i64,
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

impl Alu {
    const fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            input: 0,
        }
    }

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

    const fn is_model_no_valid(&self) -> bool {
        let mut no = self.input.abs();

        if no == 0 {
            return true;
        }

        while no > 0 {
            if no % 10 == 0 {
                return true;
            }

            no /= 10;
        }

        false
    }

    fn run(&mut self, program: &[Instr], input: &[i64]) -> i64 {
        let mut input = input.iter().copied();

        for &instr in program {
            match instr {
                Instr::Inp(a) => *self.reg_mut(a) = input.next().unwrap(),
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

fn part1(program: &[Instr]) -> i64 {
    todo!()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let program = fs::read_to_string("in/day24.txt")?
        .lines()
        .map(Instr::from_str)
        .collect::<Result<Vec<_>>>()?;

    // dbg!(&program);

    {
        let start = Instant::now();
        let part1 = self::part1(&program);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 0);
    };

    {
        let start = Instant::now();
        let part2 = self::part2();
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 0);
    };

    Ok(())
}
