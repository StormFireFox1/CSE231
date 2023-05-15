use std::env;

const INT_62_BIT_MIN: i64 = -2_305_843_009_213_693_952;
const INT_62_BIT_MAX: i64 = 2_305_843_009_213_693_951;

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
        _ => eprintln!("unknown error"),
    }
    std::process::exit(1);
}

#[export_name = "\x01snek_print"]
pub extern "C" fn snek_print(value: i64) -> i64 {
    if value == 7 { println!("true"); }
    else if value == 3 { println!("false"); }
    else if value & 0b11 == 0 { println!("{}", value >> 2); }
    else {
        println!("Unknown value: {}", value);
    }
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
    match i {
        3 => println!("false"),
        7 => println!("true"),
        _ => println!("{}", i >> 2),
    }
}