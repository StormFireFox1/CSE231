use std::env;

const INT_63_BIT_MIN: i64 = -4_611_686_018_427_387_904;
const INT_63_BIT_MAX: i64 = 4_611_686_018_427_387_903;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: i64) -> i64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    match errcode {
        1 => eprintln!("invalid argument"),
        2 => eprintln!("invalid argument"),
        3 => eprintln!("invalid argument"),
        4 => eprintln!("overflow"),
        _ => eprintln!("unknown error"),
    }
    std::process::exit(1);
}

#[export_name = "\x01snek_print"]
pub extern "C" fn snek_print(value: i64) -> i64 {
    if value == 3 { println!("true"); }
    else if value == 1 { println!("false"); }
    else if value % 2 == 0 { println!("{}", value >> 1); }
    else {
        println!("Unknown value: {}", value);
    }
    return value;
}

fn parse_input(input: &str) -> i64 {
    if input == "false" {
        1
    } else if input == "true" {
        3
    } else {
        let i = input.parse::<i64>().unwrap_or_else(|_| { panic!("overflow"); }) << 1;
        if i < INT_63_BIT_MIN || i > INT_63_BIT_MAX {
            panic!("overflow")
        }
        i
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