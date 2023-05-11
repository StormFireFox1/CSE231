use im::HashSet;
use sexp::parse;
use spec::Program;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use crate::compiler::*;

mod compiler;
mod spec;

/// Splits a string into a vector of strings by matching the furthest
/// possible brackets.
///
/// This basically assumes all Snek files are at least vaguely trying to match
/// brackets.
fn split_sexps(contents: &String) -> Vec<String> {
    let mut sexps: Vec<String> = Vec::new();
    let mut start_index: usize = 0;
    let mut bracket_count = 0;
    let mut in_closed_brackets = false;
    // Iterate over the entire file for matching strings.
    // We'll basically do our own regex matching, because PCRE is not
    // available on the autograder. Oop.
    // This is literally a "Matching Brackets" problem on Leetcode, except
    // we keep track of the start and ending index of any particular bracket.
    // and then split our strings that way such that our "sexp" library can use them later.
    for (i, c) in contents.chars().enumerate() {
        if c == '(' {
            if in_closed_brackets {
                bracket_count += 1;
            } else {
                bracket_count = 1;
                in_closed_brackets = true;
                start_index = i;
            }
        } else if c == ')' {
            if in_closed_brackets {
                bracket_count -= 1;
                if bracket_count == 0 {
                    sexps.push(contents[start_index..=i].to_string());
                    in_closed_brackets = false;
                }
            } else {
                panic!("Invalid expression provided!")
            }
        }
    }

    // Save whatever is left, even if it's not well-formed. It should be a parse error anyway.
    if bracket_count != 0 {
        sexps.push(contents.chars().skip(start_index).collect())
    }
    sexps
}

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
    let split_contents = split_sexps(&in_contents);
    let mut exps = split_contents.iter().peekable();

    let mut program = Program {
        definitions: im::HashMap::new(),
        main: Box::new(spec::Expr::Number(1)),
    };

    while let Some(exp) = exps.next() {
        // Check if this is the last value. If it is, we're parsing the main expression.
        if exps.peek().is_none() {
            program.main = Box::new(parse_expr(
                &parse(exp).expect("Invalid expression provided!"),
            ));
            break;
        }
        // Otherwise, just parse definition of function.
        let sexp = parse(exp).expect("Invalid expression provided!");
        let def = parse_def(&sexp);

        // Check if key already exists. That means we have a duplicate function!
        if program.definitions.contains_key(&def.name) {
            panic!(
                "Invalid expression provided: duplicate function name: {}",
                def.name
            );
        }

        // Check if there are duplicate parameter names in a function.
        // Do this by checking if the number of parameters is the same
        // after using a set rather than vector. There are better ways,
        // I'm sure, but whatever.
        let mut unique_params: HashSet<String> = HashSet::new();
        for param in &def.params {
            unique_params.insert(param.clone());
        }
        if unique_params.len() != def.params.len() {
            panic!(
                "Invalid expression provided: duplicate parameters name: {}",
                def.name
            );
        }

        program.definitions = program.definitions.update(def.name.clone(), def);
    }

    // If there are no parenthese, the above loop would not have run.
    // That means we have some kind of primitive value. Just
    // parse the entire input and save that to Program.
    if split_contents.is_empty() {
        program.main = Box::new(parse_expr(
            &parse(&in_contents).expect("Invalid expression provided!"),
        ));
    }

    let result = compile(&program);
    let asm_program = format!(
        "
section .text
extern snek_error
extern snek_print
global our_code_starts_here
{}
not_bool_err:
  mov rdi, 1
  call snek_error
not_num_err:
  mov rdi, 2
  call snek_error
invalid_arg_err:
  mov rdi, 3
  call snek_error
overflow_err:
  mov rdi, 4
  call snek_error
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
