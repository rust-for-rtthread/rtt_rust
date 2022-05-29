#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::sync::Arc;
use core::time::Duration;
use rtt_main::rtt_main;
use rtt_rs2::queue::Queue;
use rtt_rs2::param::Param;
use rtt_rs2::println;
use rtt_rs2::thread;
use rtt_rs2::time;

#[rtt_main(appname = "rust_em_queue", cmd = true, desc = "Rust example app.")]
fn main(_param: Param) {
    let send = Arc::new(Queue::new(2).unwrap());
    let recv = send.clone();

    let _ = thread::Thread::new()
        .name("thread 1")
        .stack_size(1024)
        .start(move || {
            loop {
                time::sleep(Duration::new(1, 0));
                send.send(String::from("msg"), 0).unwrap();
            }
        });
    time::sleep(Duration::new(1, 0));
    let _ = thread::Thread::new()
        .name("thread 2")
        .stack_size(1024)
        .start(move || {
            loop {
                println!("waiting!");
                let a = recv.recv_wait_forever().unwrap();
                println!("recv {}", a);
            }
        });
}

