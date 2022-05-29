#![no_std]

use core::time::Duration;
use rtt_main::rtt_main;
use rtt_rs2::param::Param;
use rtt_rs2::println;
use rtt_rs2::thread;
use rtt_rs2::time;

#[rtt_main(
    appname = "rust_em_thread",
    cmd = true,
    desc = "Rust example app."
)]
fn main(_param: Param) {
    let _ = thread::Thread::new()
        .name("thread 1")
        .stack_size(1024)
        .start(move || {
            loop {
                println!("thread a will sleep 1s");
                time::sleep(Duration::new(1, 0));
            }
        });

    let _ = thread::Thread::new()
        .name("thread 2")
        .stack_size(1024)
        .start(move || {
            loop {
                println!("thread b will sleep 3s");
                time::sleep(Duration::new(3, 0));
            }
        });
}
