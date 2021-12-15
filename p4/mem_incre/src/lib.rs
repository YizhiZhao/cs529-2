#![no_std]

#[macro_use] extern crate terminal_print;
extern crate alloc;
extern crate getopts;
extern crate app_io;

use getopts::Options;
use alloc::{
    vec::Vec,
    string::String,
};


pub fn main(args: Vec<String>) {
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");

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

    let typ = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage();
        return;
    };

    if typ != "stack" && typ != "heap" {
        print_usage();
        return;
    }

    if let Err(e) = run(typ) {
        println!("{}", e);
        return;
    }

}


fn run(typ: String) -> Result<(), &'static str> {
    let mut stdin = app_io::stdin()?;
    let mut buf = String::new();

    let mut heap: Vec<i32> = Vec::new();
    let mut stack_mul = 1;

    loop {
        // type anything
        let cnt = stdin.read_line(&mut buf).or(Err("failed to perform read_line"))?;
        if cnt == 0 { break; }
        println!("memory increase");

        if typ == "stack" {
            recur(stack_mul * 10);
            stack_mul *= 2;
        } else {
            heap.extend([1, 2, 3, 4, 5].iter().copied());
        }

        buf.clear();
    }

    Ok(())
}


fn recur(level: u32) {
    if level == 0 {
        let mut stdin = app_io::stdin().unwrap();
        let mut buf = String::new();
        // type anything
        let _cnt = stdin.read_line(&mut buf);
        buf.clear();
        println!("recursion exit");
        return;
    } else {
        recur(level - 1);
    }
}


fn print_usage() {
    println!("Usage: mem_incre stack/heap\n
             Press ctrl-D to exit");
}