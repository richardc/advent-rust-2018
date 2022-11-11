use itertools::Itertools;

type Number = usize;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl std::str::FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Op::*;
        match s {
            "addr" => Ok(Addr),
            "addi" => Ok(Addi),
            "mulr" => Ok(Mulr),
            "muli" => Ok(Muli),
            "banr" => Ok(Banr),
            "bani" => Ok(Bani),
            "borr" => Ok(Borr),
            "bori" => Ok(Bori),
            "setr" => Ok(Setr),
            "seti" => Ok(Seti),
            "gtir" => Ok(Gtir),
            "gtri" => Ok(Gtri),
            "gtrr" => Ok(Gtrr),
            "eqir" => Ok(Eqir),
            "eqri" => Ok(Eqri),
            "eqrr" => Ok(Eqrr),
            _ => anyhow::bail!("Invalid op {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    op: Op,
    pub a: Number,
    b: Number,
    c: Number,
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = s.split_ascii_whitespace().collect_vec();
        Ok(Instruction {
            op: chunks[0].parse()?,
            a: chunks[1].parse()?,
            b: chunks[2].parse()?,
            c: chunks[3].parse()?,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    pub registers: [Number; 6],
    pc: i32,
    pub pc_reg: usize,
    pub cycles: usize,
    pub program: Vec<Instruction>,
}

impl std::str::FromStr for Cpu {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut pc_reg = 0;
        let mut program = vec![];
        for line in input.lines() {
            if let Some(offset) = line.strip_prefix("#ip ") {
                pc_reg = offset.parse()?;
            } else {
                program.push(line.parse()?)
            }
        }

        Ok(Cpu {
            pc_reg,
            program,
            ..Cpu::default()
        })
    }
}

impl Cpu {
    pub fn apply(&mut self) {
        let instr = self.program[self.pc as usize];
        self.registers[self.pc_reg] = self.pc as Number;

        use Op::*;
        self.registers[instr.c] = match instr.op {
            Addr => self.registers[instr.a] + self.registers[instr.b],
            Addi => self.registers[instr.a] + instr.b,
            Mulr => self.registers[instr.a] * self.registers[instr.b],
            Muli => self.registers[instr.a] * instr.b,
            Banr => self.registers[instr.a] & self.registers[instr.b],
            Bani => self.registers[instr.a] & instr.b,
            Borr => self.registers[instr.a] | self.registers[instr.b],
            Bori => self.registers[instr.a] | instr.b,
            Setr => self.registers[instr.a],
            Seti => instr.a,
            Gtir => usize::from(instr.a > self.registers[instr.b]),
            Gtri => usize::from(self.registers[instr.a] > instr.b),
            Gtrr => usize::from(self.registers[instr.a] > self.registers[instr.b]),
            Eqir => usize::from(instr.a == self.registers[instr.b]),
            Eqri => usize::from(self.registers[instr.a] == instr.b),
            Eqrr => usize::from(self.registers[instr.a] == self.registers[instr.b]),
        };
        self.pc = self.registers[self.pc_reg] as i32 + 1;
        self.cycles += 1;
    }

    pub fn halted(&self) -> bool {
        self.pc < 0 || self.pc as usize >= self.program.len()
    }

    pub fn run(&mut self) {
        while !self.halted() {
            self.apply();
        }
    }

    pub fn run_till_pc(&mut self, target: usize) {
        while !self.halted() {
            self.apply();
            if self.pc == target as i32 {
                return;
            }
        }
    }
}
