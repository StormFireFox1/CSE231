use sexp::parse;
use std::env;
use std::fs::File;
use std::io::prelude::*;

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

    let expr = parse_expr(&parse(&in_contents).expect("Invalid expression provided"));
    let result = compile(&expr);
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
