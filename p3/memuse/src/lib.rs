#![no_std]

#[macro_use] extern crate log;
#[macro_use] extern crate terminal_print;
extern crate task;

use task::TASKLIST;

pub fn memuse(task_id: usize, mem_type: u32) -> Result<i32, i32>{

    info!("inside memuse: task id {}, memory type {}", task_id, mem_type);
    println!("inside memuse: task id {}, memory type {}", task_id, mem_type);

    for (id, _task) in TASKLIST.lock().iter() {
        if *id == task_id {
            return Ok(0);
        }
    }

    Err(-1)
}
