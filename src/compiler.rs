use sexp::Atom::*;
use sexp::*;

use crate::spec::*;

const INT_63_BIT_MIN: i64 = -4_611_686_018_427_387_904;
const INT_63_BIT_MAX: i64 = 4_611_686_018_427_387_903;
const KEYWORDS: [&str; 22] = [
    "add1", "sub1", "let", "+", "-", "*", "<", ">", ">=", "<=", "=", "true", "false", "input",
    "isnum", "isbool", "loop", "break", "set!", "if", "fun", "print"
];

enum Type {
    Number,
    Bool,
}

fn new_label(l: &mut i32, s: &str) -> crate::spec::Val {
    let current = *l;
    *l += 1;
    Val::Label(format!("{s}_{current}"))
}

fn assert_type(val: Val, t: Type) -> Vec<Instr> {
    let mut instrs = Vec::new();
    instrs.push(Instr::IMov(Val::Reg(Reg::RBX), val));
    instrs.push(Instr::IAnd(Val::Reg(Reg::RBX), Val::Imm(1)));
    match t {
        Type::Number => {
            instrs.push(Instr::ICmp(Val::Reg(Reg::RBX), Val::Imm(0)));
            instrs.push(Instr::IJne(Val::Label("not_num_err".to_string())));
        }
        Type::Bool => {
            instrs.push(Instr::ICmp(Val::Reg(Reg::RBX), Val::Imm(1)));
            instrs.push(Instr::IJne(Val::Label("not_bool_err".to_string())));
        }
    }
    instrs
}

pub fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(a) => match a {
            S(string) => match string.as_str() {
                "true" => Expr::Boolean(true),
                "false" => Expr::Boolean(false),
                _ => Expr::Id(string.to_string()),
            },
            I(imm) => {
                if *imm < INT_63_BIT_MIN || *imm > INT_63_BIT_MAX {
                    panic!("Invalid immediate: overflows out of 63 bits: {imm}")
                }
                Expr::Number(*imm)
            }
            F(_) => panic!("Floats not implemented in Boa!"),
        },
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => {
                Expr::UnOp(Op1::Add1, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "sub1" => {
                Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "isnum" => {
                Expr::UnOp(Op1::IsNum, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "isbool" => {
                Expr::UnOp(Op1::IsBool, Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), e] if op == "print" => {
                Expr::UnOp(Op1::Print, Box::new(parse_expr(e)))
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
            [Sexp::Atom(S(op)), e1, e2] if op == "<" => Expr::BinOp(
                Op2::Less,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == ">" => Expr::BinOp(
                Op2::Greater,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == ">=" => Expr::BinOp(
                Op2::GreaterOrEqual,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == "<=" => Expr::BinOp(
                Op2::LessOrEqual,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), e1, e2] if op == "=" => Expr::BinOp(
                Op2::Equal,
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),
            [Sexp::Atom(S(op)), Sexp::Atom(S(id)), e] if op == "set!" => {
                if KEYWORDS.contains(&id.as_str()) {
                    panic!("Invalid expression provided")
                } else {
                    Expr::Set(id.to_string(), Box::new(parse_expr(e)))
                }
            }
            [Sexp::Atom(S(op)), e1, e2, e3] if op == "if" => Expr::If(
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
                Box::new(parse_expr(e3)),
            ),
            [Sexp::Atom(S(op)), e] if op == "break" => Expr::Break(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "loop" => Expr::Loop(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), es @ ..] if op == "block" => {
                let mut exprs: Vec<Expr> = Vec::new();
                if es.is_empty() {
                    panic!("Invalid expression provided");
                }
                for expr in es {
                    exprs.push(parse_expr(expr));
                }
                Expr::Block(exprs)
            }
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
                                    if KEYWORDS.contains(&id.as_str()) {
                                        panic!("Invalid expression provided: used reserved keyword in let binding");
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

pub fn compile_instructions(
    e: &Expr,
    si: i32,
    env: &im::HashMap<String, i32>,
    l: &mut i32,
    target: &str,
) -> Vec<Instr> {
    let mut result: Vec<Instr> = Vec::new();
    match e {
        Expr::Number(n) => result.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(*n << 1))),
        Expr::Boolean(bool) => {
            result.push(Instr::IMov(
                Val::Reg(Reg::RAX),
                if *bool { Val::Imm(3) } else { Val::Imm(1) },
            ));
        }
        Expr::Id(x) => result.push(Instr::IMov(
            Val::Reg(Reg::RAX),
            Val::RegOffset(
                Reg::RSP,
                *env.get(x)
                    .unwrap_or_else(|| panic!("Unbound variable identifier {x}")),
            ),
        )),
        Expr::UnOp(op, e) => {
            let mut instrs = compile_instructions(e, si, env, l, target);
            result.append(&mut instrs);
            match op {
                Op1::Add1 => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(2)));
                    result.push(Instr::IJo(Val::Label("overflow_err".to_string())))
                }
                Op1::Sub1 => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.push(Instr::ISub(Val::Reg(Reg::RAX), Val::Imm(2)));
                    result.push(Instr::IJo(Val::Label("overflow_err".to_string())))
                }
                Op1::IsNum => {
                    result.append(&mut vec![
                        Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(0)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)),
                        Instr::ICMovE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ]);
                }
                Op1::IsBool => {
                    result.append(&mut vec![
                        Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)),
                        Instr::ICMovE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ]);
                },
                Op1::Print => {
                    let index = if si % 2 == 1 { si + 2 } else { si + 1 };
                    let offset = index * 8;
                    result.append(&mut vec![
                        Instr::ISub(Val::Reg(Reg::RSP), Val::Imm(offset.into())),
                        Instr::IMov(Val::RegOffset(Reg::RSP, 0), Val::Reg(Reg::RDI)),
                        Instr::IMov(Val::Reg(Reg::RDI), Val::Reg(Reg::RAX)),
                        Instr::ICall(Val::Label("snek_print".to_string())),
                        Instr::IMov(Val::Reg(Reg::RDI), Val::RegOffset(Reg::RSP, 0)),
                        Instr::IAdd(Val::Reg(Reg::RSP), Val::Imm(offset.into())),
                    ]);
                },
            }
        }
        Expr::BinOp(op, e1, e2) => {
            let mut v1 = compile_instructions(e1, si, env, l, target);
            let mut v2 = compile_instructions(e2, si + 1, env, l, target);
            result.append(&mut v1);
            result.push(Instr::IMov(
                Val::RegOffset(Reg::RSP, si * 8),
                Val::Reg(Reg::RAX),
            ));
            result.append(&mut v2);
            match op {
                Op2::Plus => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RSP, si * 8),
                        Type::Number,
                    ));
                    result.push(Instr::IAdd(
                        Val::Reg(Reg::RAX),
                        Val::RegOffset(Reg::RSP, si * 8),
                    ));
                    result.push(Instr::IJo(Val::Label("overflow_err".to_string())))
                }
                Op2::Minus => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RSP, si * 8),
                        Type::Number,
                    ));
                    result.push(Instr::ISub(
                        Val::RegOffset(Reg::RSP, si * 8),
                        Val::Reg(Reg::RAX),
                    ));
                    result.push(Instr::IJo(Val::Label("overflow_err".to_string())));
                    result.push(Instr::IMov(
                        Val::Reg(Reg::RAX),
                        Val::RegOffset(Reg::RSP, si * 8),
                    ))
                }
                Op2::Times => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RSP, si * 8),
                        Type::Number,
                    ));
                    result.push(Instr::ISar(Val::Reg(Reg::RAX), Val::Imm(1)));
                    result.push(Instr::IMul(
                        Val::Reg(Reg::RAX),
                        Val::RegOffset(Reg::RSP, si * 8),
                    ));
                    result.push(Instr::IJo(Val::Label("overflow_err".to_string())))
                }
                Op2::Equal => {
                    // Check if both types are equal. If not, throw the "Invalid argument error".
                    result.append(&mut vec![
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)),
                        Instr::IMov(Val::Reg(Reg::RCX), Val::RegOffset(Reg::RSP, si * 8)),
                        Instr::IAnd(Val::Reg(Reg::RBX), Val::Imm(1)),
                        Instr::IAnd(Val::Reg(Reg::RCX), Val::Imm(1)),
                        Instr::ICmp(Val::Reg(Reg::RBX), Val::Reg(Reg::RCX)),
                        Instr::IJne(Val::Label("invalid_arg_err".to_string())),
                        // Afterwards, just compare the two.
                        Instr::ICmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)),
                        Instr::ICMovE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ])
                }
                Op2::Greater => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RSP, si * 8),
                        Type::Number,
                    ));
                    result.append(&mut vec![
                        Instr::ICmp(Val::RegOffset(Reg::RSP, si * 8), Val::Reg(Reg::RAX)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)),
                        Instr::ICMovG(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ])
                }
                Op2::GreaterOrEqual => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RSP, si * 8),
                        Type::Number,
                    ));
                    result.append(&mut vec![
                        Instr::ICmp(Val::RegOffset(Reg::RSP, si * 8), Val::Reg(Reg::RAX)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)),
                        Instr::ICMovGE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ])
                }
                Op2::Less => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RSP, si * 8),
                        Type::Number,
                    ));
                    result.append(&mut vec![
                        Instr::ICmp(Val::RegOffset(Reg::RSP, si * 8), Val::Reg(Reg::RAX)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)),
                        Instr::ICMovL(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ])
                }
                Op2::LessOrEqual => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RSP, si * 8),
                        Type::Number,
                    ));
                    result.append(&mut vec![
                        Instr::ICmp(Val::RegOffset(Reg::RSP, si * 8), Val::Reg(Reg::RAX)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)),
                        Instr::ICMovLE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ])
                }
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
                    l,
                    target,
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
            let mut new_instrs =
                compile_instructions(e, new_stack_offset, &with_binds_env, l, target);
            result.append(&mut new_instrs);
        }
        Expr::If(cond, then, alt) => {
            let end_label = new_label(l, "if_end");
            let else_label = new_label(l, "if_else");
            let mut cond_instrs = compile_instructions(cond, si, env, &mut (*l + 1), target);
            let mut then_instrs = compile_instructions(then, si, env, &mut (*l + 2), target);
            let mut alt_instrs = compile_instructions(alt, si, env, &mut (*l + 3), target);
            result.append(&mut cond_instrs);
            result.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
            result.push(Instr::IJe(else_label.clone()));
            result.append(&mut then_instrs);
            result.push(Instr::IJmp(end_label.clone()));
            result.push(Instr::ILabel(else_label.to_string()));
            result.append(&mut alt_instrs);
            result.push(Instr::ILabel(end_label.to_string()));
        }
        Expr::Loop(e) => {
            let start_loop_label = new_label(l, "loop_start");
            let end_loop_label = new_label(l, "loop_end");
            let mut e_instrs =
                compile_instructions(e, si, env, &mut (*l + 1), &end_loop_label.to_string());
            result.push(Instr::ILabel(start_loop_label.to_string()));
            result.append(&mut e_instrs);
            result.push(Instr::IJmp(Val::Label(start_loop_label.to_string())));
            result.push(Instr::ILabel(end_loop_label.to_string()));
        }
        Expr::Break(e) => {
            if target.is_empty() {
                panic!("break outside of loop");
            }
            let mut e_instrs = compile_instructions(e, si, env, &mut (*l + 1), "");
            result.append(&mut e_instrs);
            result.push(Instr::IJmp(Val::Label(target.to_string())));
        }
        Expr::Set(id, e) => {
            result.append(&mut compile_instructions(e, si, env, l, target));
            result.push(Instr::IMov(
                Val::RegOffset(
                    Reg::RSP,
                    *env.get(id)
                        .unwrap_or_else(|| panic!("Unbound variable identifier {id}")),
                ),
                Val::Reg(Reg::RAX),
            ));
        }
        Expr::Block(v) => {
            for e in v {
                result.append(&mut compile_instructions(e, si, env, l, target));
            }
        },
        Expr::Call(_, _) => todo!(),
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
    println!("{:?}", e);
    let prelude: String = String::from("mov [RSP - 16], RDI\n  ");
    let mut env: im::HashMap<String, i32> = im::HashMap::new();
    let mut label: i32 = 1;
    env.insert("input".to_string(), 16);
    prelude + &instrs_to_string(&compile_instructions(e, 3, &env, &mut label, ""))
}
