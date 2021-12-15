#![no_std]

#[macro_use] extern crate log;
//#[macro_use] extern crate terminal_print;
extern crate task;
extern crate kernel_config;

use kernel_config::memory::PAGE_SIZE;

pub fn memuse(task_id: usize, mem_type: u8) -> Result<usize, &'static str>{

    info!("inside memuse: task id {}, memory type {}", task_id, mem_type);
    //println!("inside memuse: task id {}, memory type {}", task_id, mem_type);


    if let Some(task_ref) = task::get_task(task_id) {
        if mem_type == 1 {
            // stack
            let (bottom, top) = task_ref.with_kstack(|kstack|
                (kstack.bottom(), kstack.top_unusable())
            );

            let mut address = bottom;
            let mut result = 0;

            // check whether in physical memory
            while address < top {
                if task_ref.mmi.lock().page_table.translate(address).is_some() {
                    result += PAGE_SIZE;
                }
                address += PAGE_SIZE;
            }

            Ok(result)
        } else if mem_type == 2 {
            // heap
            let result = task_ref.get_heap_size();
            Ok(result)
        } else {
            Err("Unknown mem type")
        }
    }
    else {
        Err("No such task")
    }
}