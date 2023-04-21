use sexp::Atom::*;
use sexp::*;

use crate::spec::*;

pub fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(a) => match a {
            S(string) => Expr::Id(string.to_string()),
            I(imm) => Expr::Number(i32::try_from(*imm).unwrap_or_else(|_| {
                panic!("Provided immediate does not fit in i32 bit immediate: {imm}")
            })),
            F(_) => panic!("Floats not implemented in Boa!"),
        },
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => {
                Expr::UnOp(Op1::Add1, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "sub1" => {
                Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "+" => Expr::BinOp(
                Op2::Plus,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == "-" => Expr::BinOp(
                Op2::Minus,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == "*" => Expr::BinOp(
                Op2::Times,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == "let" => {
                let mut bind_expr: Vec<(String, Expr)> = Vec::new();
                // e1 should be a vector of other bindings, so:
                if let Sexp::List(binds) = e1 {
                    if binds.is_empty() {
                        panic!("Invalid expression provided");
                    }
                    for bind in binds {
                        if let Sexp::List(bind_args) = bind {
                            match &bind_args[..] {
                                [Sexp::Atom(S(id)), e] => {
                                    // If it happens that the bind ID is some stupid
                                    // name like let or add1 or any other Boa keyword, eject.
                                    if ["add1", "sub1", "let", "+", "-", "*"].contains(&id.as_str())
                                    {
                                        panic!("Invalid expression provided");
                                    }
                                    bind_expr.push((id.to_string(), parse_expr(e)));
                                }
                                _ => {
                                    panic!("Invalid expression provided");
                                }
                            }
                        } else {
                            panic!("Invalid expression provided");
                        }
                    }
                    Expr::Let(bind_expr, Box::new(parse_expr(e2)))
                } else {
                    panic!("Invalid expression provided");
                }
            }
            _ => panic!("Invalid expression provided"),
        },
    }
}

pub fn compile_instructions(e: &Expr, si: i32, env: &im::HashMap<String, i32>) -> Vec<Instr> {
    let mut result: Vec<Instr> = Vec::new();
    match e {
        Expr::Number(n) => result.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(*n))),
        Expr::Id(x) => result.push(Instr::IMov(
            Val::Reg(Reg::RAX),
            Val::RegOffset(
                Reg::RSP,
                *env.get(x)
                    .unwrap_or_else(|| panic!("Unbound variable identifier {x}")),
            ),
        )),
        Expr::UnOp(op, e) => {
            let mut instrs = compile_instructions(e, si, env);
            result.append(&mut instrs);
            result.push(match op {
                Op1::Add1 => Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(1)),
                Op1::Sub1 => Instr::ISub(Val::Reg(Reg::RAX), Val::Imm(1)),
            });
        }
        Expr::BinOp(op, e1, e2) => {
            let mut v1 = compile_instructions(e1, si, env);
            let mut v2 = compile_instructions(e2, si + 1, env);
            result.append(&mut v1);
            result.push(Instr::IMov(
                Val::RegOffset(Reg::RSP, si * 8),
                Val::Reg(Reg::RAX),
            ));
            result.append(&mut v2);
            result.push(match op {
                Op2::Plus => Instr::IAdd(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)),
                // Reversed since v1 is in memory, and it matters what order you do subtractions in.
                Op2::Minus => Instr::ISub(Val::RegOffset(Reg::RSP, si * 8), Val::Reg(Reg::RAX)),
                Op2::Times => Instr::IMul(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)),
            });
            // Move actual result to RAX, since we used the value in memory to do v1 - v2 instead
            // of any other approach. I should fix this.
            if let Op2::Minus = op {
                result.push(Instr::IMov(
                    Val::Reg(Reg::RAX),
                    Val::RegOffset(Reg::RSP, si * 8),
                ));
            }
        }
        Expr::Let(v, e) => {
            let mut new_stack_offset = si;
            let mut new_binds = im::HashMap::new();
            for tuple in v {
                let with_binds_env = new_binds
                    .clone()
                    .intersection(env.clone())
                    .union(new_binds.clone().difference(env.clone()));
                if new_binds.contains_key(&tuple.0) {
                    panic!("Duplicate binding");
                }
                result.append(&mut compile_instructions(
                    &tuple.1,
                    new_stack_offset,
                    &with_binds_env,
                ));
                result.push(Instr::IMov(
                    Val::RegOffset(Reg::RSP, new_stack_offset * 8),
                    Val::Reg(Reg::RAX),
                ));
                new_binds.insert(tuple.0.clone(), new_stack_offset * 8);
                new_stack_offset += 1;
            }
            let with_binds_env = new_binds
                .clone()
                .intersection(env.clone())
                .union(new_binds.clone().difference(env.clone()));
            let mut new_instrs = compile_instructions(e, new_stack_offset, &with_binds_env);
            result.append(&mut new_instrs);
        }
    };
    result
}

pub fn instrs_to_string(instrs: &Vec<Instr>) -> String {
    let mut result = String::new();
    for instr in instrs {
        result += &format!("{instr}").to_string();
        result += "\n  ";
    }
    result.trim_end().to_string()
}

pub fn compile(e: &Expr) -> String {
    instrs_to_string(&&compile_instructions(e, 2, &im::HashMap::new()))
}
