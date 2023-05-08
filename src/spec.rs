use std::fmt::Display;

use im::HashMap;

#[derive(Debug, Clone)]
pub enum Val {
    Reg(Reg),
    Imm(i64),
    RegOffset(Reg, i32),
    Label(String),
}

#[derive(Debug, Clone)]
pub enum Reg {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    R8,
    R9,
    R15,
    RSP,
    RDI,
}

#[derive(Debug)]
pub enum Instr {
    IMov(Val, Val),
    IAdd(Val, Val),
    ISub(Val, Val),
    IMul(Val, Val),
    ICmp(Val, Val),
    ICMovE(Val, Val),
    ICMovL(Val, Val),
    ICMovG(Val, Val),
    ICMovGE(Val, Val),
    ICMovLE(Val, Val),
    IAnd(Val, Val),
    IShr(Val, Val),
    IShl(Val, Val),
    ISar(Val, Val),
    IJe(Val),
    IJne(Val),
    IJg(Val),
    IJl(Val),
    IJo(Val),
    IJmp(Val),
    ICall(Val),
    ILabel(String),
}

#[derive(Debug, Clone)]
pub enum Op1 {
    Add1,
    Sub1,
    IsNum,
    IsBool,
    Print,
}

#[derive(Debug, Clone)]
pub enum Op2 {
    Plus,
    Minus,
    Times,
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: HashMap<String, Definition>,
    pub main: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Boolean(bool),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(String, Box<Expr>),
    Block(Vec<Expr>),
    Call(String, Vec<Expr>),
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Reg(r) => r.fmt(f),
            Val::Imm(imm) => f.write_str(&format!("{}", imm)),
            Val::RegOffset(reg, offset) => {
                let output = if *offset >= 0 { format!("[{reg} - {offset}]") } else { format!("[{} + {}]", reg, -offset) };
                f.write_str(&output)
            },
            Val::Label(label) => f.write_str(&format!("{label}")),
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Reg::RAX => f.write_str("RAX"),
            Reg::RBX => f.write_str("RBX"),
            Reg::RCX => f.write_str("RCX"),
            Reg::RSP => f.write_str("RSP"),
            Reg::RDI => f.write_str("RDI"),
            Reg::RDX => f.write_str("RDX"),
            Reg::RSI => f.write_str("RSI"),
            Reg::R8 => f.write_str("R8"),
            Reg::R9 => f.write_str("R9"),
            Reg::R15 => f.write_str("R15"),
            
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
            Instr::ICmp(val1, val2) => f.write_str(&format!("cmp {val1}, {val2}")),
            Instr::IAnd(val1, val2) => f.write_str(&format!("and {val1}, {val2}")),
            Instr::IJe(label) => f.write_str(&format!("je  near {label}")),
            Instr::IJne(label) => f.write_str(&format!("jne near {label}")),
            Instr::IJg(label) => f.write_str(&format!("jg  near {label}")),
            Instr::IJl(label) => f.write_str(&format!("jl  near {label}")),
            Instr::IJo(label) => f.write_str(&format!("jo  near {label}")),
            Instr::IJmp(label) => f.write_str(&format!("jmp near {label}")),
            Instr::ICall(label) => f.write_str(&format!("call {label}")),
            Instr::IShr(val1, val2) => f.write_str(&format!("shr {val1}, {val2}")),
            Instr::IShl(val1, val2) => f.write_str(&format!("shl {val1}, {val2}")),
            Instr::ISar(val1, val2) => f.write_str(&format!("sar {val1}, {val2}")),
            Instr::ILabel(label) => f.write_str(&format!("{label}:")),
            Instr::ICMovE(val1, val2) => f.write_str(&format!("cmove {val1}, {val2}")),
            Instr::ICMovL(val1, val2) => f.write_str(&format!("cmovl {val1}, {val2}")),
            Instr::ICMovG(val1, val2) => f.write_str(&format!("cmovg {val1}, {val2}")),
            Instr::ICMovGE(val1, val2) => f.write_str(&format!("cmovge {val1}, {val2}")),
            Instr::ICMovLE(val1, val2) => f.write_str(&format!("cmovle {val1}, {val2}")),
        }
    }
}
