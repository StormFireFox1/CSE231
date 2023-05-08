use sexp::parse;
use spec::{Definition, Program};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use pcre::Pcre;

use crate::compiler::*;

mod compiler;
mod spec;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    // We need to extract the separate blocks of s-exps, since `sexp` doesn't process
    // multiple s-expressions at once. Use regex pattern that recursively matches for furthest
    // matching parentheses in the input. Use PCRE, because it's a regex library
    // that properly manages recursion and doesn't have a terrible looking regex.
    //
    // Regex pattern from StackOverflow: https://stackoverflow.com/questions/546433/regular-expression-to-match-balanced-parentheses
    // Tested with https://regexr.com
    let mut blocks_regex = Pcre::compile(r"\((?:[^)(]+|(?R))*+\)").unwrap();
    let mut exps = blocks_regex.matches(&in_contents).peekable();

    let mut program = Program {
        definitions: im::HashMap::new(),
        main: Box::new(spec::Expr::Number(1)),
    };

    while let Some(exp) = exps.next() {
      // Check if this is the last value. If it is, we're parsing the main expression.
      if exps.peek().is_none() {
        program.main = Box::new(parse_expr(&parse(&exp.group(0)).unwrap()));
        break;
      }
      // Otherwise, just parse definition of function.
      let sexp = parse(&exp.group(0)).unwrap();
      let def = parse_def(&sexp);
      program.definitions = program.definitions.update(def.name.clone(), def);
    }

    // If there are no matches, the above loop would have not ran. 
    // That means we have some kind of primitive value. Just
    // parse the entire input and save that to Program.
    let mut blocks_regex = Pcre::compile(r"\((?:[^)(]+|(?R))*+\)").unwrap();
    if blocks_regex.matches(&in_contents).count() == 0 {
        program.main = Box::new(parse_expr(&parse(&in_contents).unwrap()));
    }

    let result = compile(&program);
    let asm_program = format!(
        "
section .text
extern snek_error
extern snek_print
global our_code_starts_here
our_code_starts_here:
  {}
  ret
not_bool_err:
  mov rdi, 1
  push rsp
  call snek_error
not_num_err:
  mov rdi, 2
  push rsp
  call snek_error
invalid_arg_err:
  mov rdi, 3
  push rsp
  call snek_error
overflow_err:
  mov rdi, 4
  push rsp
  call snek_error
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}