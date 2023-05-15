use sexp::Atom::*;
use sexp::*;

use crate::spec::*;

const INT_62_BIT_MIN: i64 = -2_305_843_009_213_693_952;
const INT_62_BIT_MAX: i64 = 2_305_843_009_213_693_951;
const KEYWORDS: [&str; 24] = [
    "add1", "sub1", "let", "+", "-", "*", "<", ">", ">=", "<=", "=", "true", "false", "input",
    "isnum", "isbool", "loop", "break", "set!", "if", "fun", "print", "index", "tuple"
];

fn align_to_16(n: i64) -> i64 {
    if n % 2 == 0 {
        n
    } else {
        n + 1
    }
}

// Dead code would be Bool, but we'll probably use this later.
// For now, disable warnings.
#[allow(dead_code)]
enum Type {
    Number,
    Bool,
    Tuple,
}

fn new_label(l: &mut i64, s: &str) -> crate::spec::Val {
    let current = *l;
    *l += 1;
    Val::Label(format!("{s}_{current}"))
}

fn assert_type(val: Val, t: Type) -> Vec<Instr> {
    let mut instrs = Vec::new();
    instrs.push(Instr::IMov(Val::Reg(Reg::RBX), val.clone()));
    instrs.push(Instr::IAnd(Val::Reg(Reg::RBX), Val::Imm(3)));
    match t {
        Type::Number => {
            instrs.push(Instr::ICmp(Val::Reg(Reg::RBX), Val::Imm(0)));
            instrs.push(Instr::IJne(Val::Label("not_num_err".to_string())));
        }
        Type::Bool => {
            instrs.push(Instr::ICmp(Val::Reg(Reg::RBX), Val::Imm(3)));
            instrs.push(Instr::IJne(Val::Label("not_bool_err".to_string())));
        },
        Type::Tuple => {
            instrs.push(Instr::ICmp(Val::Reg(Reg::RBX), Val::Imm(1)));
            instrs.push(Instr::IJne(Val::Label("not_tuple_err".to_string())));
        }

    }
    instrs
}

pub fn parse_def(s: &Sexp) -> Definition {
    if let Sexp::List(def_list) = s {
        if def_list.len() != 3 {
            panic!("Invalid expression provided: alleged definition does not have all required components");
        }
        // Check if the first value is the "fun" keyword.
        if def_list[0] != Sexp::Atom(S("fun".to_owned())) {
            panic!(
                "Invalid expression provided: alleged definition does not start with 'fun' keyword"
            );
        }

        let fun_name: String;
        let mut args = Vec::new();
        // Parse the second value, which should be a list with a bunch of string atoms.
        if let Sexp::List(vec) = &def_list[1] {
            match &vec[..] {
                [Sexp::Atom(S(name)), arguments @ ..] => {
                    fun_name = name.to_string();
                    for arg in arguments {
                        if let Sexp::Atom(S(arg_name)) = arg {
                            if KEYWORDS.contains(&arg_name.as_str()) {
                                panic!("Invalid expression provided: function parameters cannot be keywords!")
                            }
                            args.push(arg_name.clone());
                        } else {
                            panic!("Invalid expression provided: alleged definition's arguments are not strings!")
                        }
                    }
                }
                _ => {
                    panic!("Invalid expression provided: alleged definition does not contain a set of strings!")
                }
            }
        } else {
            panic!("Invalid expression provided: alleged definition does not have a function name and argument list")
        }

        // Parse the main expression as well now.
        let body = parse_expr(&def_list[2]);
        Definition {
            name: fun_name,
            params: args,
            body: Box::new(body),
        }
    } else {
        panic!("Invalid expression provided: alleged definition is not a list");
    }
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
                if *imm < INT_62_BIT_MIN || *imm > INT_62_BIT_MAX {
                    panic!("Invalid immediate: overflows out of 62 bits: {imm}")
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
            [Sexp::Atom(S(op)), e1, e2] if op == "index" => Expr::BinOp(
                Op2::Index,
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
            [Sexp::Atom(S(op)), es @ ..] if op == "tuple" => {
                let mut exprs: Vec<Expr> = Vec::new();
                if es.is_empty() {
                    panic!("Invalid expression provided");
                }
                for expr in es {
                    exprs.push(parse_expr(expr));
                }
                Expr::Tuple(exprs)
            },
            [Sexp::Atom(S(func)), args @ ..] => {
                if KEYWORDS.contains(&func.as_str()) {
                    panic!("Invalid expression provided");
                }
                let exprs = args.iter().map(parse_expr).collect::<Vec<Expr>>();
                Expr::Call(func.to_string(), exprs)
            },
            _ => panic!("Invalid expression provided"),
        },
    }
}

// Taken from lecture notes: https://github.com/ucsd-compilers-s23/lecture1/blob/stack_alloc_first/src/main.rs#L381
// Generated by ChatGPT mostly, slightly modified for our purposes.
fn depth(e: &Expr) -> i64 {
    match e {
        Expr::Number(_) => 0,
        Expr::Boolean(_) => 0,
        Expr::UnOp(_, expr) => depth(expr),
        Expr::BinOp(_, expr1, expr2) => depth(expr1).max(depth(expr2) + 1),
        Expr::Let(binds, expr) => {
            let bind_depth = binds
                .iter()
                .enumerate()
                .map(|x| depth(&x.1 .1) + x.0 as i64)
                .max()
                .unwrap_or(0);
            bind_depth + depth(expr) + binds.len() as i64
        }
        Expr::Id(_) => 0,
        Expr::If(expr1, expr2, expr3) => depth(expr1).max(depth(expr2)).max(depth(expr3)),
        Expr::Loop(expr) => depth(expr),
        Expr::Block(exprs) => exprs.iter().map(depth).max().unwrap_or(0),
        Expr::Break(expr) => depth(expr),
        Expr::Set(_, expr) => depth(expr),
        Expr::Call(_, es) => es.iter().map(depth).max().unwrap_or(0),
        Expr::Tuple(es) => es.iter().enumerate().map(|x| depth(&x.1) + x.0 as i64).max().unwrap_or(0)
    }
}

pub fn compile_definitions(defs: &im::HashMap<String, Definition>, label: &mut i64) -> Vec<Instr> {
    let mut instrs: Vec<Instr> = Vec::new();
    for (_, def) in defs.iter() {
        let depth = align_to_16(depth(&def.body));
        let offset: i64 = depth * 8;
        let mut func_env: im::HashMap<String, i64> = im::HashMap::new();
        for (i, arg) in def.params.iter().enumerate() {
            func_env = func_env.update(arg.clone(), -(i as i64 + 2) * 8);
        }
        instrs.append(&mut vec![
            Instr::ILabel(def.name.clone()),
            Instr::IPush(Val::Reg(Reg::RBP)),
            Instr::IMov(Val::Reg(Reg::RBP), Val::Reg(Reg::RSP)),
            Instr::ISub(Val::Reg(Reg::RSP), Val::Imm(offset)),
        ]);
        instrs.append(&mut compile_main(&def.body, 1, &func_env, label, "", defs));
        instrs.append(&mut vec![
            Instr::IAdd(Val::Reg(Reg::RSP), Val::Imm(offset)),
            Instr::IPop(Val::Reg(Reg::RBP)),
            Instr::IRet(),
        ]);
    }
    instrs
}

pub fn compile_main(
    e: &Expr,
    si: i64,
    env: &im::HashMap<String, i64>,
    l: &mut i64,
    target: &str,
    defs: &im::HashMap<String, Definition>,
) -> Vec<Instr> {
    let mut result: Vec<Instr> = Vec::new();
    match e {
        Expr::Number(n) => result.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(*n << 2))),
        Expr::Boolean(bool) => {
            result.push(Instr::IMov(
                Val::Reg(Reg::RAX),
                if *bool { Val::Imm(7) } else { Val::Imm(3) },
            ));
        }
        Expr::Id(x) => result.push(Instr::IMov(
            Val::Reg(Reg::RAX),
            Val::RegOffset(
                Reg::RBP,
                *env.get(x)
                    .unwrap_or_else(|| panic!("Unbound variable identifier {x}")),
            ),
        )),
        Expr::UnOp(op, e) => {
            let mut instrs = compile_main(e, si, env, l, target, defs);
            result.append(&mut instrs);
            match op {
                Op1::Add1 => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(4)));
                    result.push(Instr::IJo(Val::Label("overflow_err".to_string())))
                }
                Op1::Sub1 => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.push(Instr::ISub(Val::Reg(Reg::RAX), Val::Imm(4)));
                    result.push(Instr::IJo(Val::Label("overflow_err".to_string())))
                }
                Op1::IsNum => {
                    result.append(&mut vec![
                        Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(0)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(3)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(7)),
                        Instr::ICMovE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ]);
                }
                Op1::IsBool => {
                    result.append(&mut vec![
                        Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(3)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(7)),
                        Instr::ICMovE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ]);
                }
                Op1::Print => {
                    // FIXME All the stack alignment procedures here are not
                    // necessarily orthodox. This guarantees alignment, but it'd
                    // be best to fix this at some point.
                    //let index = align_to_16(si);
                    // let offset = index * 8;
                    result.append(&mut vec![
                        Instr::IMov(Val::Reg(Reg::RDI), Val::Reg(Reg::RAX)),
                        Instr::ICall(Val::Label("snek_print".to_string())),
                    ]);
                }
            }
        }
        Expr::BinOp(op, e1, e2) => {
            let mut v1 = compile_main(e1, si, env, l, target, defs);
            let mut v2 = compile_main(e2, si + 1, env, l, target, defs);
            result.append(&mut v1);
            result.push(Instr::IMov(
                Val::RegOffset(Reg::RBP, si * 8),
                Val::Reg(Reg::RAX),
            ));
            result.append(&mut v2);
            match op {
                Op2::Plus => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RBP, si * 8),
                        Type::Number,
                    ));
                    result.push(Instr::IAdd(
                        Val::Reg(Reg::RAX),
                        Val::RegOffset(Reg::RBP, si * 8),
                    ));
                    result.push(Instr::IJo(Val::Label("overflow_err".to_string())))
                }
                Op2::Minus => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RBP, si * 8),
                        Type::Number,
                    ));
                    result.push(Instr::ISub(
                        Val::RegOffset(Reg::RBP, si * 8),
                        Val::Reg(Reg::RAX),
                    ));
                    result.push(Instr::IJo(Val::Label("overflow_err".to_string())));
                    result.push(Instr::IMov(
                        Val::Reg(Reg::RAX),
                        Val::RegOffset(Reg::RBP, si * 8),
                    ))
                }
                Op2::Times => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RBP, si * 8),
                        Type::Number,
                    ));
                    result.push(Instr::ISar(Val::Reg(Reg::RAX), Val::Imm(2)));
                    result.push(Instr::IMul(
                        Val::Reg(Reg::RAX),
                        Val::RegOffset(Reg::RBP, si * 8),
                    ));
                    result.push(Instr::IJo(Val::Label("overflow_err".to_string())))
                }
                Op2::Equal => {
                    // Check if both types are equal. If not, throw the "Invalid argument error".
                    result.append(&mut vec![
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)),
                        Instr::IMov(Val::Reg(Reg::RCX), Val::RegOffset(Reg::RBP, si * 8)),
                        Instr::IAnd(Val::Reg(Reg::RBX), Val::Imm(3)),
                        Instr::IAnd(Val::Reg(Reg::RCX), Val::Imm(3)),
                        Instr::ICmp(Val::Reg(Reg::RBX), Val::Reg(Reg::RCX)),
                        Instr::IJne(Val::Label("invalid_arg_err".to_string())),
                        // Afterwards, just compare the two.
                        Instr::ICmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RBP, si * 8)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(3)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(7)),
                        Instr::ICMovE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ])
                },
                Op2::Index => {
                    result.append(&mut assert_type(Val::RegOffset(Reg::RBP, si * 8), Type::Tuple));
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut vec![
                        // Shift number expression down to be a real number to index with.
                        Instr::ISar(Val::Reg(Reg::RAX), Val::Imm(2)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RBP, si * 8)),
                        Instr::ISub(Val::Reg(Reg::RBX), Val::Imm(1)),
                        Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(1)),
                        Instr::IMul(Val::Reg(Reg::RAX), Val::Imm(8)),
                        Instr::IJo(Val::Label("overflow_err".to_string())),
                        Instr::IAdd(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)),
                        Instr::IJo(Val::Label("overflow_err".to_string())),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RBX, 0)),
                    ])
                },
                Op2::Greater => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RBP, si * 8),
                        Type::Number,
                    ));
                    result.append(&mut vec![
                        Instr::ICmp(Val::RegOffset(Reg::RBP, si * 8), Val::Reg(Reg::RAX)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(3)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(7)),
                        Instr::ICMovG(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ])
                }
                Op2::GreaterOrEqual => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RBP, si * 8),
                        Type::Number,
                    ));
                    result.append(&mut vec![
                        Instr::ICmp(Val::RegOffset(Reg::RBP, si * 8), Val::Reg(Reg::RAX)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(3)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(7)),
                        Instr::ICMovGE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ])
                }
                Op2::Less => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RBP, si * 8),
                        Type::Number,
                    ));
                    result.append(&mut vec![
                        Instr::ICmp(Val::RegOffset(Reg::RBP, si * 8), Val::Reg(Reg::RAX)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(3)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(7)),
                        Instr::ICMovL(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                    ])
                }
                Op2::LessOrEqual => {
                    result.append(&mut assert_type(Val::Reg(Reg::RAX), Type::Number));
                    result.append(&mut assert_type(
                        Val::RegOffset(Reg::RBP, si * 8),
                        Type::Number,
                    ));
                    result.append(&mut vec![
                        Instr::ICmp(Val::RegOffset(Reg::RBP, si * 8), Val::Reg(Reg::RAX)),
                        Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(3)),
                        Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(7)),
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
                result.append(&mut compile_main(
                    &tuple.1,
                    new_stack_offset,
                    &with_binds_env,
                    l,
                    target,
                    defs,
                ));
                result.push(Instr::IMov(
                    Val::RegOffset(Reg::RBP, new_stack_offset * 8),
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
                compile_main(e, new_stack_offset, &with_binds_env, l, target, defs);
            result.append(&mut new_instrs);
        }
        Expr::If(cond, then, alt) => {
            let end_label = new_label(l, "if_end");
            let else_label = new_label(l, "if_else");
            let mut cond_instrs = compile_main(cond, si, env, &mut (*l + 1), target, defs);
            let mut then_instrs = compile_main(then, si, env, &mut (*l + 2), target, defs);
            let mut alt_instrs = compile_main(alt, si, env, &mut (*l + 3), target, defs);
            result.append(&mut cond_instrs);
            result.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(3)));
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
                compile_main(e, si, env, &mut (*l + 1), &end_loop_label.to_string(), defs);
            result.push(Instr::ILabel(start_loop_label.to_string()));
            result.append(&mut e_instrs);
            result.push(Instr::IJmp(Val::Label(start_loop_label.to_string())));
            result.push(Instr::ILabel(end_loop_label.to_string()));
        }
        Expr::Break(e) => {
            if target.is_empty() {
                panic!("break outside of loop");
            }
            let mut e_instrs = compile_main(e, si, env, &mut (*l + 1), "", defs);
            result.append(&mut e_instrs);
            result.push(Instr::IJmp(Val::Label(target.to_string())));
        }
        Expr::Set(id, e) => {
            result.append(&mut compile_main(e, si, env, l, target, defs));
            result.push(Instr::IMov(
                Val::RegOffset(
                    Reg::RBP,
                    *env.get(id)
                        .unwrap_or_else(|| panic!("Unbound variable identifier {id}")),
                ),
                Val::Reg(Reg::RAX),
            ));
        }
        Expr::Block(v) => {
            for e in v {
                result.append(&mut compile_main(e, si, env, l, target, defs));
            }
        }
        Expr::Call(name, args) => {
            // Check to see if we've called an existing function with the proper number of arguments.
            let func_def = defs
                .get(name)
                .expect("Invalid expression provided: Call to undefined function");
            if args.len() != func_def.params.len() {
                panic!("Invalid expression provided: Different number of arguments for function call to {name}. Expected {}, got {}", func_def.params.len(), args.len());
            }

            // Ok, now we've checked for a valid call, so let's do it!
            // The idea is simple. Adjust the stack pointer upwards by the amount needed
            // to fit each argument. Then, make sure that compile_defs has all the envs
            // in the same spot.
            let arg_offset = align_to_16(args.len() as i64) * 8;
            let extra_space = arg_offset - (args.len() as i64) * 8;
            if extra_space != 0 {
                result.push(Instr::IPush(Val::Imm(0)));
            }
            for arg in args.iter().rev() {
                result.append(&mut compile_main(arg, si, env, l, target, defs));
                result.push(Instr::IPush(Val::Reg(Reg::RAX)))
            }
            result.push(Instr::ICall(Val::Label(name.to_string())));
            result.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::Imm(arg_offset)));
        }
        Expr::Tuple(values) => {
            let mut new_stack_offset = si;
            for value in values {
                result.append(&mut compile_main(value, new_stack_offset, env, l, target, defs));
                result.push(Instr::IMov(
                    Val::RegOffset(Reg::RBP, new_stack_offset * 8),
                    Val::Reg(Reg::RAX),
                ));
                new_stack_offset += 1;
            }
            result.push(Instr::IMov(Val::RegOffset(Reg::R15, 0), Val::Imm((values.len() as i64) << 2)));
            for (heap_index, value_index) in (si..=(new_stack_offset - 1)).enumerate() {
                result.append(&mut vec![
                    Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RBP, value_index * 8)),
                    Instr::IMov(Val::RegOffset(Reg::R15, -((heap_index as i64) + 1) * 8), Val::Reg(Reg::RAX)),
                ]);
            }
            result.append(&mut vec![
                Instr::IMov(Val::Reg(Reg::RAX), Val::Reg(Reg::R15)),
                Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(1)),
                Instr::IAdd(Val::Reg(Reg::R15), Val::Imm(values.len() as i64 * 8))
            ]);
        },
    };
    result
}

pub fn instrs_to_string(instrs: &Vec<Instr>) -> String {
    let mut result = String::new();
    for instr in instrs {
        result += &format!("{instr}").to_string();
        result += "\n";
    }
    result.to_string()
}

pub fn compile(p: &Program) -> String {
    let depth = depth(&p.main);
    let offset = align_to_16(depth + 1) * 8;
    let prelude: String =
        format!("our_code_starts_here:\n  push rbp\n  mov rbp, rsp\n  sub RSP, {offset}\n  mov [RBP - 8], RDI\n  mov R15, RSI\n");
    let postlude: String = format!("  add RSP, {offset}\n  pop rbp\n  ret\n");
    let mut env: im::HashMap<String, i64> = im::HashMap::new();
    env = env.update("input".to_owned(), 8);
    let mut label: i64 = 1;
    // Pass the function definitions using a separate variable for compile_main.
    instrs_to_string(&compile_definitions(&p.definitions, &mut label))
        + &prelude
        + &instrs_to_string(&compile_main(
            &p.main,
            2,
            &env,
            &mut label,
            "",
            &p.definitions,
        ))
        + &postlude
}
