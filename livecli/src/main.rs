use std::{env, process::exit};

use livecli::reverse_string;

fn main() {
    // output:Args { inner: ["target\\debug\\livecli.exe", "arg1", "arg2"] }
    // we cant access inner as it is private member
    let args = env::args();

    // _ in Vec<_> ->  auto guess data type
    // here it is String -> Vec<String>
    let args: Vec<_> = args.collect();

    // 1st element of args was the program name or exe path
    let args = &args[1..];

    if args.len() < 1 {
        println!("You must enter any string");
        exit(0);
    }

    for i in args {
        println!("Original String: {}", i);
        println!("Reverse String: {}", reverse_string(i));

        println!();
    }
}
