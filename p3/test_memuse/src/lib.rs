#![no_std]

#[macro_use] extern crate terminal_print;
extern crate memuse;
extern crate alloc;
extern crate getopts;

use getopts::Options;
use alloc::vec::Vec;
use alloc::string::String;

pub fn main(args: Vec<String>) {
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optopt("t", "type", "memory type", "1: stack, 2: heap");

    let matches = match opts.parse(&args) {
        Ok(m) => m,
        Err(_f) => {
            println!("{}", _f);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage();
        return;
    }

    let task_id_str = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage();
        return;
    };

    let task_id =
        if task_id_str.parse::<usize>().is_ok() {
            task_id_str.parse::<usize>().unwrap()
        } else {
            print_usage();
            return;
        };

    let mut mem_type: u32 = 1;
    let type_str = matches.opt_str("t").unwrap_or(String::from("1"));
    let type_parsed = type_str.parse::<u32>();
    if type_parsed.is_ok() {
        mem_type = type_parsed.unwrap();
    }

    let result = memuse::memuse(task_id, mem_type);
    if result.is_ok() {
        println!("memuse result with task id {}, mem_type {}: {}", task_id, mem_type, result.unwrap());
    } else {
        println!("memuse fails with task id {}, mem_type {}", task_id, mem_type)
    }

}

fn print_usage() {
    println!("Usage: test_memuse [TASK_ID] -t [MEM_TYPE]\n
                TASK_ID: unsigned integer\n
                MEM_TYPE: 1: stack, 2: heap");
}
    