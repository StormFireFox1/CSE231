use std::env;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64) -> i64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    match errcode {
        1 => eprintln!("invalid argument: not a boolean"),
        2 => eprintln!("invalid argument: not a number"),
        3 => eprintln!("invalid argument: not same type"),
        4 => eprintln!("overflow"),
        _ => eprintln!("unknown error"),
    }
    std::process::exit(1);
}

fn parse_input(input: &str) -> u64 {
    if input == "false" {
        1
    } else if input == "true" {
        3
    } else {
        input.parse::<u64>().unwrap() << 1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let i: i64 = unsafe { our_code_starts_here(input) };
    match i {
        1 => println!("false"),
        3 => println!("true"),
        _ => println!("{}", i >> 1),
    }
}