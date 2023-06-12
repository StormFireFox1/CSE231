use std::env;
use std::collections::HashSet;
use std::convert::TryInto;

const INT_62_BIT_MIN: i64 = -2_305_843_009_213_693_952;
const INT_62_BIT_MAX: i64 = 2_305_843_009_213_693_951;
const TRUE: i64 = 7;
const FALSE: i64 = 3;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: i64, memory: *mut i64) -> i64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64, value: i64) {
    match errcode {
        1 => eprintln!("invalid argument - not a boolean: {}", value),
        2 => eprintln!("invalid argument - not a number: {}", value),
        3 => eprintln!("invalid argument - not the same type when comparing for equality!"),
        4 => eprintln!("overflow"),
        5 => eprintln!("invalid argument - not a tuple: {:#x}", value),
        6 => eprintln!("out of bounds tuple indexing error - tried to index at {}", value >> 2),
        7 => eprintln!("tuple null pointer exception!"),
        _ => eprintln!("unknown error"),
    }
    std::process::exit(1);
}

// Largely copied from https://github.com/ucsd-compilers-s23/lecture1/blob/b97536112db34a61c6fbd73edf39e6365e794e12/runtime/start.rs#L19C2-L37
// Movied to adapt to our specific implementation of type tagging.
fn snek_str(val: i64, seen : &mut HashSet<i64>) -> String {
    if val == 7 { "true".to_string() }
    else if val == 3 { "false".to_string() }
    else if val & 0b11 == 0 { format!("{}", val >> 2) }
    else if val == 1 { "nil".to_string() }
    else if val & 0b11 == 1 {
      if seen.contains(&val)  { return "(tuple ...)".to_string() }
      seen.insert(val);
      let addr = (val - 1) as *const i64;
      let size = unsafe { *addr } >> 2;
      let mut tuple_contents = String::from("(tuple ");
      for i in 1..=size {
          tuple_contents += format!("{} ", snek_str(unsafe { *addr.offset(i.try_into().expect("runtime tuple printing error")) }, seen)).as_str();
      }
      tuple_contents = tuple_contents.trim_end().to_owned() + ")";
      seen.remove(&val);
      return tuple_contents;
    }
    else {
      format!("Unknown value: {}", val)
    }
}

fn snek_compare_tuples(tuple_1: i64, tuple_2: i64, traversed_values: &mut HashSet<(Vec<i64>, Vec<i64>)>) -> i64 {
    // We are guaranteed the provided values are tuples here, so we need to just check for equality
    // on tuples, not anything else.
    // First, check if length is equal. If that's not equal, then nothing is.
    let addr_1 = (tuple_1 - 1) as *const i64;
    let addr_2 = (tuple_2 - 1) as *const i64;
    let len_1 = unsafe { *addr_1 } >> 2;
    let len_2 = unsafe { *addr_2 } >> 2;
    if len_1 != len_2 {
        return FALSE
    }

    let mut tuple_vec_1: Vec<i64> = Vec::new();
    let mut tuple_vec_2: Vec<i64> = Vec::new();
    let mut pairs_of_tuples_to_check: Vec<(i64, i64)> = Vec::new();
    // Now iterate through each value in both tuples.
    for index in 1..=len_1 {
        let val_1 = unsafe { *addr_1.offset(index.try_into().expect("runtime tuple structural equality error")) };
        let val_2 = unsafe { *addr_2.offset(index.try_into().expect("runtime tuple structural equality error")) };
        tuple_vec_1.push(val_1);
        tuple_vec_2.push(val_2);
        // Let's get the simple cases out.
        // If the type tags differ, they're different.
        if val_1 & 0b11 != val_2 & 0b11 {
            return FALSE
        }
        // At this point, the type tags are identical.
        // If the two elements are NOT tuples, check for simple equality.
        if val_1 & 0b11 != 1 {
            if val_1 != val_2 {
                return FALSE
            }
        }
        // Check if both are tuples. If they are, add them to the list of pairs to check.
        if val_1 & 0b11 == 1 {
            pairs_of_tuples_to_check.push((val_1, val_2));
        }
    }
    // At this point, the tuples seem equal. We'll want to check if we've done this comparison before.
    // If we have, then that must mean we're reached a cyclic position in our structure and thus can easily
    // return TRUE without worries.
    if traversed_values.contains(&(tuple_vec_1.clone(), tuple_vec_2.clone())) {
        return TRUE
    }

    // Otherwise, add our current tuple to the set of traversed values and recurse with the pairs of tuples
    // we have to check. If any fail, our entire comparison fails.
    traversed_values.insert((tuple_vec_1, tuple_vec_2));
    for pair in pairs_of_tuples_to_check {
        if snek_compare_tuples(pair.0, pair.1, traversed_values) == FALSE {
            return FALSE
        }
    }

    // If by this point nothing returned false, they're equal!
    return TRUE
}

#[export_name = "\x01snek_tuple_equal"]
pub extern "C" fn snek_tuple_equal(tuple_1: i64, tuple_2: i64) -> i64 {
    let mut seen = HashSet::<(Vec<i64>, Vec<i64>)>::new();
    snek_compare_tuples(tuple_1, tuple_2, &mut seen)
}

#[export_name = "\x01snek_print"]
pub extern "C" fn snek_print(value: i64) -> i64 {
    let mut seen = HashSet::<i64>::new();
    println!("{}", snek_str(value, &mut seen));
    return value;
}

fn parse_input(input: &str) -> i64 {
    if input == "false" {
        3
    } else if input == "true" {
        7
    } else {
        let i = input.parse::<i64>().unwrap_or_else(|_| { panic!("overflow"); }) << 2;
        if i < INT_62_BIT_MIN || i > INT_62_BIT_MAX {
            panic!("overflow")
        }
        i
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);
    let mut memory = Vec::<i64>::with_capacity(1000000);
    let buffer :*mut i64 = memory.as_mut_ptr();
    
    let i : i64 = unsafe { our_code_starts_here(input, buffer) };
    let mut seen = HashSet::<i64>::new();
    println!("{}", snek_str(i, &mut seen));
}