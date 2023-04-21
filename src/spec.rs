use std::fmt::Display;

#[derive(Debug)]
pub enum Val {
    Reg(Reg),
    Imm(i32),
    RegOffset(Reg, i32),
}

#[derive(Debug)]
#[allow(dead_code)] // remove when RBX and RDI are used
pub enum Reg {
    RAX,
    RBX,
    RSP,
    RDI,
}

#[derive(Debug)]
pub enum Instr {
    IMov(Val, Val),
    IAdd(Val, Val),
    ISub(Val, Val),
    IMul(Val, Val),
}

#[derive(Debug)]
pub enum Op1 {
    Add1,
    Sub1,
}

#[derive(Debug)]
pub enum Op2 {
    Plus,
    Minus,
    Times,
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Reg(r) => r.fmt(f),
            Val::Imm(imm) => f.write_str(&format!("{imm}")),
            Val::RegOffset(reg, offset) => f.write_str(&format!("[{reg} - {offset}]")),
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Reg::RAX => f.write_str("RAX"),
            Reg::RBX => f.write_str("RBX"),
            Reg::RSP => f.write_str("RSP"),
            Reg::RDI => f.write_str("RDI"),
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::IMov(val1, val2) => f.write_str(&format!("mov {val1}, {val2}")),
            Instr::IAdd(val1, val2) => f.write_str(&format!("add {val1}, {val2}")),
            Instr::ISub(val1, val2) => f.write_str(&format!("sub {val1}, {val2}")),
            Instr::IMul(val1, val2) => f.write_str(&format!("imul {val1}, {val2}")),
        }
    }
}
