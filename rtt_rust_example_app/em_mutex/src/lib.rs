#![no_std]

extern crate alloc;

use alloc::sync::Arc;
use core::time::Duration;
use rtt_main::rtt_main;
use rtt_rs2::mutex::Mutex;
use rtt_rs2::param::Param;
use rtt_rs2::println;
use rtt_rs2::thread;
use rtt_rs2::time;

#[rtt_main(appname = "rust_em_mutex", cmd = true, desc = "Rust example app.")]
fn main(_param: Param) {
    let counter = Arc::new(Mutex::new(0).unwrap());
    let run = move || loop {
        time::sleep(Duration::new(2, 0));
        {
            let mut c = counter.lock().unwrap();
            *c += 1;
            println!("{}", *c);
        }
    };

    let _ = thread::Thread::new()
        .name("thread 1")
        .stack_size(1024)
        .start(run.clone());
    time::sleep(Duration::new(1, 0));
    let _ = thread::Thread::new()
        .name("thread 2")
        .stack_size(1024)
        .start(run.clone());
}
